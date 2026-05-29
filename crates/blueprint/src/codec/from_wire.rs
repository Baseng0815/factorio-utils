use tracing::{info, instrument, trace, warn};

use prototypes::{ItemId, RecipeId};

use super::kind_table::{KindHint, hint_for};
use crate::entity::{
    AssemblingMachine, BeltIo, Chest, Entity, EntityKind, EntityNumber, Furnace, Inserter,
    ItemStack, MiningDrill, Pipe, PipeToGround, PowerPole, Pump, Splitter, TransportBelt,
    UndergroundBelt,
};
use crate::error::{Error, Result};
use crate::wire::{Envelope, RawBlueprint, RawEntity, RawEntityExtras, RawItemStack};
use crate::world::{Direction, EntityName, OpaqueJson, World};

#[instrument(level = "debug", skip_all)]
pub fn to_world(envelope: Envelope) -> Result<World> {
    let blueprint = match envelope.blueprint {
        Some(b) => b,
        None => {
            let variant = envelope.variant_name().unwrap_or("(empty)");
            warn!(variant, "envelope has no `blueprint` field");
            return Err(Error::UnsupportedEnvelope(variant));
        }
    };
    let world = world_from_blueprint(blueprint)?;
    info!(entities = world.len(), "decoded blueprint into world");
    Ok(world)
}

fn world_from_blueprint(b: RawBlueprint) -> Result<World> {
    let mut world = World::new();
    world.label = b.label;
    world.description = b.description;
    world.version = b.version;
    world.icons = b.icons.map(OpaqueJson::from).unwrap_or_default();
    world.tiles = b.tiles.map(OpaqueJson::from).unwrap_or_default();
    world.schedules = b.schedules.map(OpaqueJson::from).unwrap_or_default();
    for raw in b.entities {
        let entity = entity_from_raw(raw)?;
        world.insert_with_number(entity)?;
    }
    Ok(world)
}

#[instrument(level = "trace", skip(raw), fields(number = raw.entity_number, name = %raw.name))]
fn entity_from_raw(raw: RawEntity) -> Result<Entity> {
    let number = EntityNumber::new(raw.entity_number);
    let name = EntityName::new(raw.name.clone());
    let position = raw.position.into();
    let direction = raw.direction.map(Direction::from).unwrap_or_default();
    let kind = kind_from_raw(&raw.name, &raw.extras, raw.entity_number)?;
    trace!(?kind, "decoded entity");
    Ok(Entity {
        number,
        name,
        position,
        direction,
        kind,
    })
}

fn kind_from_raw(name: &str, extras: &RawEntityExtras, number: u64) -> Result<EntityKind> {
    let Some(hint) = hint_for(name) else {
        warn!(name, "no kind hint for entity");
        return Err(Error::UnsupportedEntity(EntityName::new(name.to_owned())));
    };
    match hint {
        KindHint::AssemblingMachine => Ok(assembling_machine(extras).into()),
        KindHint::Furnace => Ok(furnace(extras).into()),
        KindHint::MiningDrill => Ok(mining_drill(extras).into()),
        KindHint::Inserter => Ok(inserter(extras).into()),
        KindHint::TransportBelt => Ok(TransportBelt.into()),
        KindHint::UndergroundBelt => Ok(underground_belt(extras, number)?.into()),
        KindHint::Splitter => Ok(splitter(extras).into()),
        KindHint::Pipe => Ok(Pipe.into()),
        KindHint::PipeToGround => Ok(PipeToGround.into()),
        KindHint::Pump => Ok(Pump.into()),
        KindHint::Chest => Ok(chest(extras).into()),
        KindHint::PowerPole => Ok(PowerPole.into()),
    }
}

fn assembling_machine(extras: &RawEntityExtras) -> AssemblingMachine {
    AssemblingMachine {
        recipe: extras.recipe.as_deref().map(RecipeId::from),
        modules: modules_from(&extras.items),
    }
}

fn furnace(extras: &RawEntityExtras) -> Furnace {
    Furnace {
        modules: modules_from(&extras.items),
    }
}

fn mining_drill(extras: &RawEntityExtras) -> MiningDrill {
    MiningDrill {
        modules: modules_from(&extras.items),
        resource_filter: extras.resource_filter.as_deref().map(ItemId::from),
    }
}

fn inserter(extras: &RawEntityExtras) -> Inserter {
    let mut filters: Vec<ItemId> = extras
        .filters
        .as_ref()
        .map(|fs| fs.iter().map(|f| ItemId::from(f.name.as_str())).collect())
        .unwrap_or_default();
    if let Some(filter) = extras.filter.as_deref() {
        filters.push(ItemId::from(filter));
    }
    Inserter {
        filters,
        override_stack_size: extras.override_stack_size,
    }
}

fn underground_belt(extras: &RawEntityExtras, number: u64) -> Result<UndergroundBelt> {
    let io = match extras.belt_io.as_deref() {
        Some(s) => BeltIo::from(s),
        None => {
            return Err(Error::MalformedEntity {
                number,
                reason: "underground-belt missing `type` (input|output)".into(),
            });
        }
    };
    Ok(UndergroundBelt { io })
}

fn splitter(extras: &RawEntityExtras) -> Splitter {
    Splitter {
        input_priority: extras.input_priority.as_deref().map(Into::into),
        output_priority: extras.output_priority.as_deref().map(Into::into),
        filter: extras.filter.as_deref().map(ItemId::from),
    }
}

fn chest(extras: &RawEntityExtras) -> Chest {
    Chest { bar: extras.bar }
}

fn modules_from(items: &Option<Vec<RawItemStack>>) -> Vec<ItemStack> {
    items
        .as_ref()
        .map(|v| {
            v.iter()
                .map(|s| ItemStack {
                    item: ItemId::from(s.item.as_str()),
                    count: s.count,
                })
                .collect()
        })
        .unwrap_or_default()
}
