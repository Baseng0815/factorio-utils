use prototypes::{Database, MachineId, RecipeId, ResourceId, dump};

use planner::{EdgeEndpoint, PlanConfig, PlanRequest, Rate, plan};

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("off")),
        )
        .with_test_writer()
        .try_init();
}

fn item(name: &str) -> ResourceId {
    ResourceId::Item(name.into())
}

fn fluid(name: &str) -> ResourceId {
    ResourceId::Fluid(name.into())
}

const VANILLA_LIKE: &str = r#"{
    "item": {
        "iron-ore":     { "stack_size": 50 },
        "iron-plate":   { "stack_size": 100 },
        "copper-ore":   { "stack_size": 50 },
        "copper-plate": { "stack_size": 100 },
        "copper-cable": { "stack_size": 200 },
        "electronic-circuit": { "stack_size": 200 }
    },
    "fluid": {},
    "assembling-machine": {
        "assembling-machine-1": {
            "crafting_categories": ["crafting"],
            "crafting_speed": 0.5,
            "energy_usage": "75kW",
            "module_slots": 0
        },
        "assembling-machine-2": {
            "crafting_categories": ["crafting"],
            "crafting_speed": 0.75,
            "energy_usage": "150kW",
            "module_slots": 2
        }
    },
    "furnace": {
        "stone-furnace": {
            "crafting_categories": ["smelting"],
            "crafting_speed": 1.0,
            "energy_usage": "90kW"
        },
        "steel-furnace": {
            "crafting_categories": ["smelting"],
            "crafting_speed": 2.0,
            "energy_usage": "90kW"
        }
    },
    "mining-drill": {
        "electric-mining-drill": {
            "resource_categories": ["basic-solid"],
            "mining_speed": 0.5,
            "energy_usage": "90kW",
            "module_slots": 3
        }
    },
    "recipe": {
        "iron-plate": {
            "category": "smelting",
            "energy_required": 3.2,
            "ingredients": [{"type":"item","name":"iron-ore","amount":1}],
            "results":     [{"type":"item","name":"iron-plate","amount":1}]
        },
        "copper-plate": {
            "category": "smelting",
            "energy_required": 3.2,
            "ingredients": [{"type":"item","name":"copper-ore","amount":1}],
            "results":     [{"type":"item","name":"copper-plate","amount":1}]
        },
        "copper-cable": {
            "category": "crafting",
            "energy_required": 0.5,
            "ingredients": [{"type":"item","name":"copper-plate","amount":1}],
            "results":     [{"type":"item","name":"copper-cable","amount":2}]
        },
        "electronic-circuit": {
            "category": "crafting",
            "energy_required": 0.5,
            "ingredients": [
                {"type":"item","name":"iron-plate","amount":1},
                {"type":"item","name":"copper-cable","amount":3}
            ],
            "results": [{"type":"item","name":"electronic-circuit","amount":1}]
        }
    },
    "resource": {
        "iron-ore":   { "category": "basic-solid", "minable": { "mining_time": 1.0, "result": "iron-ore" } },
        "copper-ore": { "category": "basic-solid", "minable": { "mining_time": 1.0, "result": "copper-ore" } }
    }
}"#;

fn load_db() -> Database {
    dump::from_str(VANILLA_LIKE).unwrap()
}

#[test]
fn plans_smelting_recurses_to_mining() {
    init_tracing();
    let db = load_db();
    let request = PlanRequest::new().want(item("iron-plate"), Rate::per_second(1.0));
    let line = plan(&db, &request).unwrap();

    let recipes: Vec<&str> = line.nodes.iter().map(|n| n.recipe.as_str()).collect();
    assert!(recipes.contains(&"iron-plate"));
    assert!(recipes.contains(&"iron-ore"));

    assert!(line.raw_inputs.is_empty());

    let out = line.outputs.get(&item("iron-plate")).unwrap();
    assert!((out.as_per_second() - 1.0).abs() < 1e-9);
}

#[test]
fn defaults_to_fastest_machine_per_category() {
    init_tracing();
    let db = load_db();
    let request = PlanRequest::new().want(item("iron-plate"), Rate::per_second(1.0));
    let line = plan(&db, &request).unwrap();

    let smelting_node = line
        .nodes
        .iter()
        .find(|n| n.recipe.as_str() == "iron-plate")
        .unwrap();
    assert_eq!(smelting_node.machine.as_str(), "steel-furnace");
}

#[test]
fn raw_override_short_circuits_recursion() {
    init_tracing();
    let db = load_db();
    let config = PlanConfig::new().with_raw(item("iron-plate"));
    let request = PlanRequest::new()
        .want(item("iron-plate"), Rate::per_second(1.0))
        .with_config(config);
    let line = plan(&db, &request).unwrap();

    assert!(line.nodes.is_empty());
    let raw = line.raw_inputs.get(&item("iron-plate")).unwrap();
    assert!((raw.as_per_second() - 1.0).abs() < 1e-9);
}

#[test]
fn machine_override_is_respected() {
    init_tracing();
    let db = load_db();
    let config = PlanConfig::new()
        .with_machine("smelting".into(), MachineId::from("stone-furnace"));
    let request = PlanRequest::new()
        .want(item("iron-plate"), Rate::per_second(1.0))
        .with_config(config);
    let line = plan(&db, &request).unwrap();

    let smelting = line
        .nodes
        .iter()
        .find(|n| n.recipe.as_str() == "iron-plate")
        .unwrap();
    assert_eq!(smelting.machine.as_str(), "stone-furnace");
    assert!((smelting.runs_per_second - 1.0 / 1.0).abs() < 1e-9);
    let cps = 1.0 / 3.2;
    let expected_machines = smelting.runs_per_second / cps;
    assert!((smelting.machines_needed - expected_machines).abs() < 1e-6);
}

#[test]
fn ambiguous_recipe_errors_without_override() {
    init_tracing();
    let dump_text = r#"{
        "item": { "x": { "stack_size": 1 } },
        "assembling-machine": {
            "asm": { "crafting_categories": ["crafting"], "crafting_speed": 1.0, "energy_usage": "1kW" }
        },
        "recipe": {
            "x-a": { "category": "crafting", "energy_required": 1, "ingredients": [], "results": [{"type":"item","name":"x","amount":1}] },
            "x-b": { "category": "crafting", "energy_required": 1, "ingredients": [], "results": [{"type":"item","name":"x","amount":1}] }
        }
    }"#;
    let db = dump::from_str(dump_text).unwrap();
    let request = PlanRequest::new().want(item("x"), Rate::per_second(1.0));
    let err = plan(&db, &request).unwrap_err();
    assert!(matches!(err, planner::Error::AmbiguousRecipe { .. }));
}

#[test]
fn ambiguity_resolved_via_override() {
    init_tracing();
    let dump_text = r#"{
        "item": { "x": { "stack_size": 1 } },
        "assembling-machine": {
            "asm": { "crafting_categories": ["crafting"], "crafting_speed": 1.0, "energy_usage": "1kW" }
        },
        "recipe": {
            "x-a": { "category": "crafting", "energy_required": 1, "ingredients": [], "results": [{"type":"item","name":"x","amount":1}] },
            "x-b": { "category": "crafting", "energy_required": 1, "ingredients": [], "results": [{"type":"item","name":"x","amount":1}] }
        }
    }"#;
    let db = dump::from_str(dump_text).unwrap();
    let config = PlanConfig::new().with_recipe(item("x"), RecipeId::from("x-b"));
    let request = PlanRequest::new()
        .want(item("x"), Rate::per_second(1.0))
        .with_config(config);
    let line = plan(&db, &request).unwrap();
    assert_eq!(line.nodes.len(), 1);
    assert_eq!(line.nodes[0].recipe.as_str(), "x-b");
}

#[test]
fn coproducts_become_surplus_outputs() {
    init_tracing();
    let dump_text = r#"{
        "item": { "a": { "stack_size": 1 }, "b": { "stack_size": 1 } },
        "assembling-machine": {
            "asm": { "crafting_categories": ["crafting"], "crafting_speed": 1.0, "energy_usage": "1kW" }
        },
        "recipe": {
            "ab": {
                "category": "crafting",
                "energy_required": 1,
                "ingredients": [],
                "results": [
                    {"type":"item","name":"a","amount":1},
                    {"type":"item","name":"b","amount":2}
                ]
            }
        }
    }"#;
    let db = dump::from_str(dump_text).unwrap();
    let request = PlanRequest::new().want(item("a"), Rate::per_second(1.0));
    let line = plan(&db, &request).unwrap();

    let out_a = line.outputs.get(&item("a")).unwrap().as_per_second();
    let out_b = line.outputs.get(&item("b")).unwrap().as_per_second();
    assert!((out_a - 1.0).abs() < 1e-9);
    assert!((out_b - 2.0).abs() < 1e-9);
}

#[test]
fn surplus_satisfies_later_demand_without_extra_runs() {
    init_tracing();
    let dump_text = r#"{
        "item": { "a": { "stack_size": 1 }, "b": { "stack_size": 1 } },
        "assembling-machine": {
            "asm": { "crafting_categories": ["crafting"], "crafting_speed": 1.0, "energy_usage": "1kW" }
        },
        "recipe": {
            "ab": {
                "category": "crafting",
                "energy_required": 1,
                "ingredients": [],
                "results": [
                    {"type":"item","name":"a","amount":1},
                    {"type":"item","name":"b","amount":2}
                ]
            }
        }
    }"#;
    let db = dump::from_str(dump_text).unwrap();
    let request = PlanRequest::new()
        .want(item("a"), Rate::per_second(1.0))
        .want(item("b"), Rate::per_second(1.0));
    let line = plan(&db, &request).unwrap();

    assert_eq!(line.nodes.len(), 1);
    let node = &line.nodes[0];
    assert!((node.runs_per_second - 1.0).abs() < 1e-9);

    let out_a = line.outputs.get(&item("a")).unwrap().as_per_second();
    let out_b = line.outputs.get(&item("b")).unwrap().as_per_second();
    assert!((out_a - 1.0).abs() < 1e-9);
    assert!((out_b - 2.0).abs() < 1e-9);
}

#[test]
fn mining_recursion_emits_raw_for_fluids_with_no_recipe() {
    init_tracing();
    let dump_text = r#"{
        "item": { "uranium-ore": { "stack_size": 50 } },
        "fluid": { "sulfuric-acid": { "default_temperature": 25 } },
        "mining-drill": {
            "drill": { "resource_categories": ["basic-solid"], "mining_speed": 0.5, "energy_usage": "90kW" }
        },
        "resource": {
            "uranium-ore": {
                "category": "basic-solid",
                "minable": {
                    "mining_time": 2.0,
                    "results": [{"type":"item","name":"uranium-ore","amount":1}],
                    "required_fluid": "sulfuric-acid",
                    "fluid_amount": 10
                }
            }
        }
    }"#;
    let db = dump::from_str(dump_text).unwrap();
    let request = PlanRequest::new().want(item("uranium-ore"), Rate::per_second(1.0));
    let line = plan(&db, &request).unwrap();

    assert_eq!(line.nodes.len(), 1);
    let acid_rate = line.raw_inputs.get(&fluid("sulfuric-acid")).unwrap();
    assert!((acid_rate.as_per_second() - 1.0).abs() < 1e-9);

    let edges_from_external: Vec<_> = line
        .edges
        .iter()
        .filter(|e| matches!(e.from, EdgeEndpoint::External))
        .collect();
    assert_eq!(edges_from_external.len(), 1);
    assert!(edges_from_external[0].resource.is_fluid());
}

#[test]
fn target_with_no_recipe_becomes_raw_input() {
    init_tracing();
    let dump_text = r#"{ "item": { "x": { "stack_size": 1 } } }"#;
    let db = dump::from_str(dump_text).unwrap();
    let request = PlanRequest::new().want(item("x"), Rate::per_second(1.0));
    let line = plan(&db, &request).unwrap();
    assert!(line.nodes.is_empty());
    let raw = line.raw_inputs.get(&item("x")).unwrap();
    assert!((raw.as_per_second() - 1.0).abs() < 1e-9);
}
