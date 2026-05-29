use itertools::Itertools;
use tracing::{info, instrument, trace};

use crate::entity::{
    AssemblingMachine, Chest, Entity, EntityKind, Furnace, Inserter, ItemStack, MiningDrill,
    Splitter, UndergroundBelt,
};
use crate::wire::{
    Envelope, RawBlueprint, RawEntity, RawEntityExtras, RawInserterFilter, RawItemStack,
};
use crate::world::World;

#[instrument(level = "debug", skip_all)]
pub fn from_world(world: &World) -> Envelope {
    let entities = world
        .entities_sorted_by_number()
        .map(raw_from_entity)
        .collect_vec();
    info!(entities = entities.len(), "encoded world into blueprint");
    let blueprint = RawBlueprint {
        item: Some("blueprint".to_owned()),
        label: world.label.clone(),
        description: world.description.clone(),
        version: world.version,
        icons: opaque_to_value(&world.icons),
        entities,
        tiles: opaque_to_value(&world.tiles),
        schedules: opaque_to_value(&world.schedules),
    };
    Envelope {
        blueprint: Some(blueprint),
        ..Envelope::default()
    }
}

fn opaque_to_value(o: &crate::world::OpaqueJson) -> Option<serde_json::Value> {
    if o.is_null() { None } else { Some(o.0.clone()) }
}

fn raw_from_entity(e: &Entity) -> RawEntity {
    trace!(number = %e.number, name = %e.name, "encoding entity");
    RawEntity {
        entity_number: e.number.as_u64(),
        name: e.name.as_str().to_owned(),
        position: e.position.into(),
        direction: direction_for_wire(e.direction),
        extras: extras_from_kind(&e.kind),
    }
}

fn direction_for_wire(d: crate::world::Direction) -> Option<u8> {
    if d == crate::world::Direction::North {
        None
    } else {
        Some(d.as_u8())
    }
}

fn extras_from_kind(kind: &EntityKind) -> RawEntityExtras {
    let mut extras = RawEntityExtras::default();
    match kind {
        EntityKind::AssemblingMachine(m) => fill_assembling_machine(&mut extras, m),
        EntityKind::Furnace(f) => fill_furnace(&mut extras, f),
        EntityKind::MiningDrill(m) => fill_mining_drill(&mut extras, m),
        EntityKind::Inserter(i) => fill_inserter(&mut extras, i),
        EntityKind::UndergroundBelt(u) => fill_underground_belt(&mut extras, u),
        EntityKind::Splitter(s) => fill_splitter(&mut extras, s),
        EntityKind::Chest(c) => fill_chest(&mut extras, c),
        EntityKind::TransportBelt(_)
        | EntityKind::Pipe(_)
        | EntityKind::PipeToGround(_)
        | EntityKind::Pump(_)
        | EntityKind::PowerPole(_) => {}
    }
    extras
}

fn fill_assembling_machine(extras: &mut RawEntityExtras, m: &AssemblingMachine) {
    extras.recipe = m.recipe.as_ref().map(|r| r.as_str().to_owned());
    extras.items = raw_modules(&m.modules);
}

fn fill_furnace(extras: &mut RawEntityExtras, f: &Furnace) {
    extras.items = raw_modules(&f.modules);
}

fn fill_mining_drill(extras: &mut RawEntityExtras, m: &MiningDrill) {
    extras.items = raw_modules(&m.modules);
    extras.resource_filter = m.resource_filter.as_ref().map(|r| r.as_str().to_owned());
}

fn fill_inserter(extras: &mut RawEntityExtras, i: &Inserter) {
    extras.override_stack_size = i.override_stack_size;
    if !i.filters.is_empty() {
        extras.filters = Some(
            i.filters
                .iter()
                .enumerate()
                .map(|(idx, f)| RawInserterFilter {
                    index: idx as u32 + 1,
                    name: f.as_str().to_owned(),
                })
                .collect(),
        );
    }
}

fn fill_underground_belt(extras: &mut RawEntityExtras, u: &UndergroundBelt) {
    extras.belt_io = Some(u.io.as_str().to_owned());
}

fn fill_splitter(extras: &mut RawEntityExtras, s: &Splitter) {
    extras.input_priority = s.input_priority.map(|p| p.as_str().to_owned());
    extras.output_priority = s.output_priority.map(|p| p.as_str().to_owned());
    extras.filter = s.filter.as_ref().map(|f| f.as_str().to_owned());
}

fn fill_chest(extras: &mut RawEntityExtras, c: &Chest) {
    extras.bar = c.bar;
}

fn raw_modules(modules: &[ItemStack]) -> Option<Vec<RawItemStack>> {
    if modules.is_empty() {
        None
    } else {
        Some(
            modules
                .iter()
                .map(|s| RawItemStack {
                    item: s.item.as_str().to_owned(),
                    count: s.count,
                })
                .collect(),
        )
    }
}
