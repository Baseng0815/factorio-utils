use crate::entity::{Entity, EntityKind};

pub type Color = [u8; 4];

pub fn color_for(entity: &Entity) -> Color {
    match &entity.kind {
        EntityKind::AssemblingMachine(_) => [85, 130, 200, 255],
        EntityKind::Furnace(_) => [220, 120, 50, 255],
        EntityKind::MiningDrill(_) => [140, 80, 200, 255],
        EntityKind::Inserter(_) => inserter_color(entity.name.as_str()),
        EntityKind::TransportBelt(_) => belt_color(entity.name.as_str()),
        EntityKind::UndergroundBelt(_) => underground_belt_color(entity.name.as_str()),
        EntityKind::Splitter(_) => splitter_color(entity.name.as_str()),
        EntityKind::Pipe(_) => [80, 160, 220, 255],
        EntityKind::PipeToGround(_) => [50, 110, 180, 255],
        EntityKind::Pump(_) => [70, 200, 230, 255],
        EntityKind::Chest(_) => [160, 110, 70, 255],
        EntityKind::PowerPole(_) => [200, 160, 100, 255],
    }
}

fn inserter_color(name: &str) -> Color {
    match name {
        "burner-inserter" => [150, 90, 50, 255],
        "long-handed-inserter" => [200, 60, 60, 255],
        "fast-inserter" => [80, 140, 230, 255],
        "filter-inserter" => [180, 90, 200, 255],
        "stack-inserter" | "bulk-inserter" => [80, 180, 110, 255],
        "stack-filter-inserter" => [110, 200, 160, 255],
        _ => [240, 210, 70, 255],
    }
}

fn belt_color(name: &str) -> Color {
    match name {
        "fast-transport-belt" => [220, 140, 60, 255],
        "express-transport-belt" => [80, 170, 220, 255],
        "turbo-transport-belt" => [180, 90, 220, 255],
        _ => [200, 200, 200, 255],
    }
}

fn underground_belt_color(name: &str) -> Color {
    match name {
        "fast-underground-belt" => [170, 100, 40, 255],
        "express-underground-belt" => [50, 130, 180, 255],
        "turbo-underground-belt" => [140, 60, 180, 255],
        _ => [140, 140, 140, 255],
    }
}

fn splitter_color(name: &str) -> Color {
    match name {
        "fast-splitter" => [200, 130, 50, 255],
        "express-splitter" => [60, 150, 200, 255],
        "turbo-splitter" => [160, 70, 200, 255],
        _ => [170, 170, 170, 255],
    }
}

pub fn size_for(entity: &Entity) -> (f64, f64) {
    if let Some(specific) = name_specific_size(entity.name.as_str()) {
        return specific;
    }
    match &entity.kind {
        EntityKind::AssemblingMachine(_) => (3.0, 3.0),
        EntityKind::Furnace(_) => (2.0, 2.0),
        EntityKind::MiningDrill(_) => (3.0, 3.0),
        EntityKind::Inserter(_) => (1.0, 1.0),
        EntityKind::TransportBelt(_) => (1.0, 1.0),
        EntityKind::UndergroundBelt(_) => (1.0, 1.0),
        EntityKind::Splitter(_) => (2.0, 1.0),
        EntityKind::Pipe(_) => (1.0, 1.0),
        EntityKind::PipeToGround(_) => (1.0, 1.0),
        EntityKind::Pump(_) => (1.0, 2.0),
        EntityKind::Chest(_) => (1.0, 1.0),
        EntityKind::PowerPole(_) => (1.0, 1.0),
    }
}

fn name_specific_size(name: &str) -> Option<(f64, f64)> {
    match name {
        "electromagnetic-plant" => Some((4.0, 4.0)),
        "cryogenic-plant" => Some((7.0, 5.0)),
        "biochamber" => Some((3.0, 3.0)),
        "electric-furnace" => Some((3.0, 3.0)),
        "rocket-silo" => Some((9.0, 9.0)),
        "big-mining-drill" => Some((5.0, 5.0)),
        "steel-chest" | "iron-chest" | "wooden-chest" => Some((1.0, 1.0)),
        _ => None,
    }
}

pub fn lighten(c: Color, amount: u8) -> Color {
    [
        c[0].saturating_add(amount),
        c[1].saturating_add(amount),
        c[2].saturating_add(amount),
        c[3],
    ]
}

pub fn darken(c: Color, amount: u8) -> Color {
    [
        c[0].saturating_sub(amount),
        c[1].saturating_sub(amount),
        c[2].saturating_sub(amount),
        c[3],
    ]
}
