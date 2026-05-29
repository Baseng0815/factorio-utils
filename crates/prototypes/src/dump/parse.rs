use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use serde_json::Value;
use tracing::{info, info_span, instrument, trace, warn};

use super::energy::parse_energy;
use super::raw::{
    RawBoundingBox, RawFluid, RawIconFields, RawIngredient, RawItem, RawMachine, RawMinable,
    RawMiningDrill, RawProduct, RawRecipe, RawResource,
};
use crate::database::Database;
use crate::error::{Error, Result};
use crate::icon::{DEFAULT_ICON_SIZE, IconRef};
use crate::machine::{CraftingCategory, Machine, MachineId, MachineKind};
use crate::recipe::{Ingredient, Product, ProductAmount, Recipe, RecipeId};
use crate::resource::{Fluid, FluidId, Item, ItemId, ResourceId};

pub fn load_from_path(path: impl AsRef<Path>) -> Result<Database> {
    let path = path.as_ref();
    let _span = info_span!("load_dump", path = %path.display()).entered();
    let file = File::open(path)?;
    from_reader(BufReader::new(file))
}

pub fn from_reader<R: Read>(reader: R) -> Result<Database> {
    let dump: Value = serde_json::from_reader(reader)?;
    build(&dump)
}

pub fn from_str(text: &str) -> Result<Database> {
    let dump: Value = serde_json::from_str(text)?;
    build(&dump)
}

pub fn from_slice(bytes: &[u8]) -> Result<Database> {
    let dump: Value = serde_json::from_slice(bytes)?;
    build(&dump)
}

#[instrument(level = "debug", skip_all)]
fn build(dump: &Value) -> Result<Database> {
    let mut db = Database::new();
    extract_items(&mut db, dump)?;
    extract_fluids(&mut db, dump)?;
    extract_machines(&mut db, dump)?;
    extract_mining_drills(&mut db, dump)?;
    extract_recipes(&mut db, dump)?;
    extract_resources(&mut db, dump)?;
    info!(
        items = db.items.len(),
        fluids = db.fluids.len(),
        recipes = db.recipes.len(),
        machines = db.machines.len(),
        "factorio database loaded",
    );
    Ok(db)
}

const ITEM_PROTOTYPES: &[&str] = &[
    "item",
    "tool",
    "module",
    "capsule",
    "gun",
    "ammo",
    "armor",
    "repair-tool",
    "item-with-entity-data",
    "rail-planner",
    "blueprint",
    "blueprint-book",
    "deconstruction-item",
    "upgrade-item",
    "copy-paste-tool",
    "selection-tool",
    "item-with-tags",
    "item-with-label",
    "spidertron-remote",
    "mining-tool",
];

const MACHINE_PROTOTYPES: &[(&str, MachineKind)] = &[
    ("assembling-machine", MachineKind::AssemblingMachine),
    ("furnace", MachineKind::Furnace),
    ("rocket-silo", MachineKind::RocketSilo),
];

fn prototype_entries<'a>(
    dump: &'a Value,
    prototype: &str,
) -> Option<&'a serde_json::Map<String, Value>> {
    dump.get(prototype).and_then(Value::as_object)
}

#[instrument(level = "debug", skip_all)]
fn extract_items(db: &mut Database, dump: &Value) -> Result<()> {
    let before = db.items.len();
    for prototype in ITEM_PROTOTYPES {
        let Some(entries) = prototype_entries(dump, prototype) else {
            continue;
        };
        for (name, raw) in entries {
            let raw_item: RawItem = decode(prototype, name, raw)?;
            trace!(
                prototype = prototype,
                item = %name,
                stack_size = raw_item.stack_size,
                "registered item",
            );
            let icon = extract_icon(&raw_item.icon);
            db.items.insert(
                ItemId::from(name.as_str()),
                Item {
                    stack_size: raw_item.stack_size,
                    icon,
                },
            );
        }
    }
    info!(count = db.items.len() - before, "loaded items");
    Ok(())
}

#[instrument(level = "debug", skip_all)]
fn extract_fluids(db: &mut Database, dump: &Value) -> Result<()> {
    let Some(entries) = prototype_entries(dump, "fluid") else {
        return Ok(());
    };
    let before = db.fluids.len();
    for (name, raw) in entries {
        let raw_fluid: RawFluid = decode("fluid", name, raw)?;
        trace!(
            fluid = %name,
            default_temperature = raw_fluid.default_temperature,
            "registered fluid",
        );
        let icon = extract_icon(&raw_fluid.icon);
        db.fluids.insert(
            FluidId::from(name.as_str()),
            Fluid {
                default_temperature: raw_fluid.default_temperature,
                icon,
            },
        );
    }
    info!(count = db.fluids.len() - before, "loaded fluids");
    Ok(())
}

#[instrument(level = "debug", skip_all)]
fn extract_machines(db: &mut Database, dump: &Value) -> Result<()> {
    let before = db.machines.len();
    for (prototype, kind) in MACHINE_PROTOTYPES {
        let Some(entries) = prototype_entries(dump, prototype) else {
            continue;
        };
        for (name, raw) in entries {
            let raw_machine: RawMachine = decode(prototype, name, raw)?;
            let machine = build_machine(*kind, raw_machine)?;
            trace!(
                machine = %name,
                kind = ?machine.kind,
                categories = ?machine.crafting_categories,
                crafting_speed = machine.crafting_speed,
                module_slots = machine.module_slots,
                energy_watts = machine.energy_usage_watts,
                "registered machine",
            );
            db.machines.insert(MachineId::from(name.as_str()), machine);
        }
    }
    info!(count = db.machines.len() - before, "loaded machines");
    Ok(())
}

#[instrument(level = "debug", skip_all)]
fn extract_mining_drills(db: &mut Database, dump: &Value) -> Result<()> {
    let Some(entries) = prototype_entries(dump, "mining-drill") else {
        return Ok(());
    };
    let before = db.machines.len();
    for (name, raw) in entries {
        let raw_drill: RawMiningDrill = decode("mining-drill", name, raw)?;
        let drill = build_mining_drill(raw_drill)?;
        trace!(
            drill = %name,
            categories = ?drill.crafting_categories,
            mining_speed = drill.crafting_speed,
            module_slots = drill.module_slots,
            energy_watts = drill.energy_usage_watts,
            "registered mining drill",
        );
        db.machines.insert(MachineId::from(name.as_str()), drill);
    }
    info!(count = db.machines.len() - before, "loaded mining drills");
    Ok(())
}

fn build_mining_drill(raw: RawMiningDrill) -> Result<Machine> {
    let module_slots = raw
        .module_slots
        .or_else(|| raw.module_specification.as_ref().map(|s| s.module_slots))
        .unwrap_or(0);
    let energy_usage_watts = parse_energy(&raw.energy_usage)?;
    let icon = extract_icon(&raw.icon);
    let crafting_categories = raw
        .resource_categories
        .into_iter()
        .map(CraftingCategory::from)
        .collect();
    let (tile_width, tile_height) = resolve_tile_size(
        raw.tile_width,
        raw.tile_height,
        raw.selection_box.as_ref(),
        raw.collision_box.as_ref(),
    );
    Ok(Machine {
        kind: MachineKind::MiningDrill,
        crafting_categories,
        crafting_speed: raw.mining_speed,
        module_slots,
        energy_usage_watts,
        tile_width,
        tile_height,
        icon,
    })
}

fn resolve_tile_size(
    tile_width: Option<u32>,
    tile_height: Option<u32>,
    selection_box: Option<&RawBoundingBox>,
    collision_box: Option<&RawBoundingBox>,
) -> (u32, u32) {
    let from_box = selection_box.or(collision_box).map(tile_size_from_box);
    let w = tile_width.or(from_box.map(|(w, _)| w)).unwrap_or(1);
    let h = tile_height.or(from_box.map(|(_, h)| h)).unwrap_or(1);
    (w, h)
}

fn tile_size_from_box(b: &RawBoundingBox) -> (u32, u32) {
    let w = (b.max[0] - b.min[0]).ceil().max(1.0) as u32;
    let h = (b.max[1] - b.min[1]).ceil().max(1.0) as u32;
    (w, h)
}

#[instrument(level = "debug", skip_all)]
fn extract_resources(db: &mut Database, dump: &Value) -> Result<()> {
    let Some(entries) = prototype_entries(dump, "resource") else {
        return Ok(());
    };
    let before = db.recipes.len();
    let mut skipped = 0usize;
    for (name, raw) in entries {
        let raw_resource: RawResource = decode("resource", name, raw)?;
        let Some(minable) = raw_resource.minable else {
            trace!(resource = %name, "skipping resource: no minable section");
            skipped += 1;
            continue;
        };
        let recipe = build_mining_recipe(name, raw_resource.category, minable)?;
        trace!(
            resource = %name,
            category = %recipe.category,
            mining_time = recipe.crafting_time,
            ingredients = recipe.ingredients.len(),
            products = recipe.products.len(),
            "synthesized mining recipe",
        );
        db.recipes.insert(RecipeId::from(name.as_str()), recipe);
    }
    info!(
        synthesized = db.recipes.len() - before,
        skipped,
        "loaded mining recipes from resources",
    );
    Ok(())
}

fn build_mining_recipe(name: &str, category: String, minable: RawMinable) -> Result<Recipe> {
    if minable.mining_time <= 0.0 {
        warn!(
            resource = %name,
            mining_time = minable.mining_time,
            "resource has non-positive mining time",
        );
    }
    let ingredients = mining_ingredients(minable.required_fluid.clone(), minable.fluid_amount);
    let products = mining_products(name, minable.result, minable.count, minable.results)?;
    Ok(Recipe {
        category: CraftingCategory::from(category),
        ingredients,
        products,
        crafting_time: minable.mining_time,
        allow_productivity: true,
    })
}

fn mining_ingredients(required_fluid: Option<String>, fluid_amount: f64) -> Vec<Ingredient> {
    let Some(fluid) = required_fluid else {
        return Vec::new();
    };
    vec![Ingredient {
        resource: ResourceId::Fluid(FluidId::from(fluid)),
        amount: fluid_amount / 10.0,
    }]
}

fn mining_products(
    resource_name: &str,
    legacy_result: Option<String>,
    legacy_count: Option<u32>,
    results: Vec<RawProduct>,
) -> Result<Vec<Product>> {
    if !results.is_empty() {
        return results.into_iter().map(build_product).collect();
    }
    let item_name = legacy_result.unwrap_or_else(|| resource_name.to_owned());
    Ok(vec![Product {
        resource: ResourceId::Item(ItemId::from(item_name)),
        amount: ProductAmount::Fixed(legacy_count.unwrap_or(1) as f64),
        probability: 1.0,
    }])
}

#[instrument(level = "debug", skip_all)]
fn extract_recipes(db: &mut Database, dump: &Value) -> Result<()> {
    let Some(entries) = prototype_entries(dump, "recipe") else {
        return Ok(());
    };
    let before = db.recipes.len();
    for (name, raw) in entries {
        let raw_recipe: RawRecipe = decode("recipe", name, raw)?;
        let recipe = build_recipe(name, raw_recipe)?;
        trace!(
            recipe = %name,
            category = %recipe.category,
            crafting_time = recipe.crafting_time,
            ingredients = recipe.ingredients.len(),
            products = recipe.products.len(),
            allow_productivity = recipe.allow_productivity,
            "registered recipe",
        );
        db.recipes.insert(RecipeId::from(name.as_str()), recipe);
    }
    info!(count = db.recipes.len() - before, "loaded crafting recipes");
    Ok(())
}

fn decode<T: serde::de::DeserializeOwned>(kind: &'static str, name: &str, raw: &Value) -> Result<T> {
    serde_json::from_value(raw.clone()).map_err(|err| Error::MalformedPrototype {
        kind,
        name: name.to_owned(),
        reason: err.to_string(),
    })
}

fn build_machine(kind: MachineKind, raw: RawMachine) -> Result<Machine> {
    let module_slots = raw
        .module_slots
        .or_else(|| raw.module_specification.as_ref().map(|s| s.module_slots))
        .unwrap_or(0);
    let energy_usage_watts = parse_energy(&raw.energy_usage)?;
    let icon = extract_icon(&raw.icon);
    let crafting_categories = raw
        .crafting_categories
        .into_iter()
        .map(CraftingCategory::from)
        .collect();
    let (tile_width, tile_height) = resolve_tile_size(
        raw.tile_width,
        raw.tile_height,
        raw.selection_box.as_ref(),
        raw.collision_box.as_ref(),
    );
    Ok(Machine {
        kind,
        crafting_categories,
        crafting_speed: raw.crafting_speed,
        module_slots,
        energy_usage_watts,
        tile_width,
        tile_height,
        icon,
    })
}

fn build_recipe(name: &str, raw: RawRecipe) -> Result<Recipe> {
    if raw.energy_required <= 0.0 {
        warn!(
            recipe = %name,
            crafting_time = raw.energy_required,
            "recipe has non-positive crafting time",
        );
    }
    let ingredients = raw
        .ingredients
        .into_iter()
        .map(build_ingredient)
        .collect::<Result<Vec<_>>>()?;
    let products = raw
        .results
        .into_iter()
        .map(build_product)
        .collect::<Result<Vec<_>>>()?;
    Ok(Recipe {
        category: CraftingCategory::from(raw.category),
        ingredients,
        products,
        crafting_time: raw.energy_required,
        allow_productivity: raw.allow_productivity,
    })
}

fn build_ingredient(raw: RawIngredient) -> Result<Ingredient> {
    Ok(Ingredient {
        resource: resource_id(&raw.ty, &raw.name)?,
        amount: raw.amount,
    })
}

fn build_product(raw: RawProduct) -> Result<Product> {
    let amount = match (raw.amount, raw.amount_min, raw.amount_max) {
        (Some(n), _, _) => ProductAmount::Fixed(n),
        (None, Some(min), Some(max)) => ProductAmount::Range { min, max },
        _ => ProductAmount::Fixed(1.0),
    };
    Ok(Product {
        resource: resource_id(&raw.ty, &raw.name)?,
        amount,
        probability: raw.probability,
    })
}

fn extract_icon(raw: &RawIconFields) -> Option<IconRef> {
    if let Some(path) = &raw.icon {
        return Some(IconRef::new(
            path.clone(),
            raw.icon_size.unwrap_or(DEFAULT_ICON_SIZE),
        ));
    }
    let layers = raw.icons.as_ref()?;
    let first = layers.first()?;
    Some(IconRef::new(
        first.icon.clone(),
        first.icon_size.or(raw.icon_size).unwrap_or(DEFAULT_ICON_SIZE),
    ))
}

fn resource_id(ty: &str, name: &str) -> Result<ResourceId> {
    match ty {
        "item" => Ok(ResourceId::Item(ItemId::from(name))),
        "fluid" => Ok(ResourceId::Fluid(FluidId::from(name))),
        other => Err(Error::UnknownResourceType(other.to_owned())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_tracing() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("off")),
            )
            .with_test_writer()
            .try_init();
    }

    const SAMPLE: &str = r#"{
        "item": {
            "iron-ore": { "stack_size": 50 },
            "iron-plate": { "stack_size": 100 }
        },
        "fluid": {
            "water": { "default_temperature": 15 }
        },
        "assembling-machine": {
            "assembling-machine-1": {
                "crafting_categories": ["crafting"],
                "crafting_speed": 0.5,
                "energy_usage": "75kW",
                "module_slots": 0
            }
        },
        "furnace": {
            "stone-furnace": {
                "crafting_categories": ["smelting"],
                "crafting_speed": 1.0,
                "energy_usage": "90kW"
            }
        },
        "recipe": {
            "iron-plate": {
                "category": "smelting",
                "energy_required": 3.2,
                "ingredients": [{"type": "item", "name": "iron-ore", "amount": 1}],
                "results": [{"type": "item", "name": "iron-plate", "amount": 1}]
            }
        }
    }"#;

    #[test]
    fn loads_sample_dump() {
        init_tracing();
        let db = from_str(SAMPLE).unwrap();
        assert_eq!(db.items.len(), 2);
        assert_eq!(db.fluids.len(), 1);
        assert_eq!(db.machines.len(), 2);
        assert_eq!(db.recipes.len(), 1);

        let recipe = &db[&RecipeId::from("iron-plate")];
        assert_eq!(recipe.category.as_str(), "smelting");
        assert_eq!(recipe.crafting_time, 3.2);
        assert_eq!(recipe.ingredients.len(), 1);
        assert_eq!(recipe.products.len(), 1);

        let furnace = &db[&MachineId::from("stone-furnace")];
        assert_eq!(furnace.crafting_speed, 1.0);
        assert_eq!(furnace.energy_usage_watts, 90_000.0);

        let smelters: Vec<_> = db
            .machines_for_recipe(recipe)
            .map(|m| m.as_str().to_owned())
            .collect();
        assert_eq!(smelters, vec!["stone-furnace"]);
    }

    const MINING_SAMPLE: &str = r#"{
        "item": {
            "iron-ore": { "stack_size": 50 },
            "uranium-ore": { "stack_size": 50 }
        },
        "fluid": {
            "sulfuric-acid": { "default_temperature": 25 },
            "crude-oil": { "default_temperature": 25 }
        },
        "mining-drill": {
            "electric-mining-drill": {
                "resource_categories": ["basic-solid"],
                "mining_speed": 0.5,
                "energy_usage": "90kW",
                "module_slots": 3
            },
            "pumpjack": {
                "resource_categories": ["basic-fluid"],
                "mining_speed": 1.0,
                "energy_usage": "90kW",
                "module_slots": 2
            }
        },
        "resource": {
            "iron-ore": {
                "category": "basic-solid",
                "minable": { "mining_time": 1.0, "result": "iron-ore" }
            },
            "uranium-ore": {
                "category": "basic-solid",
                "minable": {
                    "mining_time": 2.0,
                    "results": [{"type":"item","name":"uranium-ore","amount":1}],
                    "required_fluid": "sulfuric-acid",
                    "fluid_amount": 10
                }
            },
            "crude-oil": {
                "category": "basic-fluid",
                "minable": {
                    "mining_time": 1.0,
                    "results": [{"type":"fluid","name":"crude-oil","amount_min":10,"amount_max":10}]
                }
            }
        }
    }"#;

    #[test]
    fn loads_mining_drills_and_resources() {
        init_tracing();
        let db = from_str(MINING_SAMPLE).unwrap();

        let drill = &db[&MachineId::from("electric-mining-drill")];
        assert_eq!(drill.kind, MachineKind::MiningDrill);
        assert_eq!(drill.crafting_speed, 0.5);
        assert!(drill.supports(&CraftingCategory::from("basic-solid")));

        let iron = &db[&RecipeId::from("iron-ore")];
        assert!(iron.ingredients.is_empty());
        assert_eq!(iron.expected_yield(&ItemId::from("iron-ore").into()), 1.0);
        assert_eq!(iron.crafting_time, 1.0);

        let drills: Vec<_> = db
            .machines_for_recipe(iron)
            .map(|m| m.as_str().to_owned())
            .collect();
        assert_eq!(drills, vec!["electric-mining-drill"]);
    }

    #[test]
    fn mining_with_required_fluid_divides_amount_by_ten() {
        init_tracing();
        let db = from_str(MINING_SAMPLE).unwrap();
        let uranium = &db[&RecipeId::from("uranium-ore")];
        assert_eq!(uranium.ingredients.len(), 1);
        let acid = &uranium.ingredients[0];
        assert!(acid.resource.is_fluid());
        assert_eq!(acid.resource.as_str(), "sulfuric-acid");
        assert!((acid.amount - 1.0).abs() < 1e-9);
    }

    #[test]
    fn pumpjack_extracts_fluid() {
        init_tracing();
        let db = from_str(MINING_SAMPLE).unwrap();
        let oil = &db[&RecipeId::from("crude-oil")];
        assert_eq!(oil.products.len(), 1);
        assert!(oil.products[0].resource.is_fluid());
        assert_eq!(oil.expected_yield(&FluidId::from("crude-oil").into()), 10.0);

        let drills: Vec<_> = db
            .machines_for_recipe(oil)
            .map(|m| m.as_str().to_owned())
            .collect();
        assert_eq!(drills, vec!["pumpjack"]);
    }

    #[test]
    fn parses_icon_field_for_item() {
        init_tracing();
        let dump = r#"{
            "item": {
                "iron-plate": {
                    "stack_size": 100,
                    "icon": "__base__/graphics/icons/iron-plate.png",
                    "icon_size": 64
                }
            }
        }"#;
        let db = from_str(dump).unwrap();
        let icon = db[&ItemId::from("iron-plate")].icon.as_ref().unwrap();
        assert_eq!(icon.path, "__base__/graphics/icons/iron-plate.png");
        assert_eq!(icon.size, 64);
    }

    #[test]
    fn falls_back_to_first_icon_layer() {
        init_tracing();
        let dump = r#"{
            "fluid": {
                "lubricant": {
                    "default_temperature": 25,
                    "icons": [
                        {"icon": "__base__/graphics/icons/lubricant.png", "icon_size": 32},
                        {"icon": "__base__/graphics/icons/overlay.png"}
                    ]
                }
            }
        }"#;
        let db = from_str(dump).unwrap();
        let icon = db[&FluidId::from("lubricant")].icon.as_ref().unwrap();
        assert_eq!(icon.path, "__base__/graphics/icons/lubricant.png");
        assert_eq!(icon.size, 32);
    }

    #[test]
    fn icon_defaults_to_64_when_size_missing() {
        init_tracing();
        let dump = r#"{
            "item": {
                "thing": {
                    "stack_size": 1,
                    "icon": "__base__/graphics/icons/thing.png"
                }
            }
        }"#;
        let db = from_str(dump).unwrap();
        let icon = db[&ItemId::from("thing")].icon.as_ref().unwrap();
        assert_eq!(icon.size, 64);
    }

    #[test]
    fn item_without_icon_is_none() {
        init_tracing();
        let dump = r#"{ "item": { "x": { "stack_size": 1 } } }"#;
        let db = from_str(dump).unwrap();
        assert!(db[&ItemId::from("x")].icon.is_none());
    }

    #[test]
    fn product_amount_range_expected() {
        init_tracing();
        let dump = r#"{
            "recipe": {
                "uranium-ore-processing": {
                    "category": "centrifuging",
                    "energy_required": 12,
                    "ingredients": [{"type":"item","name":"uranium-ore","amount":10}],
                    "results": [
                        {"type":"item","name":"uranium-235","amount":1,"probability":0.007},
                        {"type":"item","name":"uranium-238","amount":1,"probability":0.993}
                    ]
                }
            }
        }"#;
        let db = from_str(dump).unwrap();
        let r = &db[&RecipeId::from("uranium-ore-processing")];
        let yield_235 = r.expected_yield(&ItemId::from("uranium-235").into());
        assert!((yield_235 - 0.007).abs() < 1e-9);
    }
}
