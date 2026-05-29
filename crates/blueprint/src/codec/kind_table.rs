#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KindHint {
    AssemblingMachine,
    Furnace,
    MiningDrill,
    Inserter,
    TransportBelt,
    UndergroundBelt,
    Splitter,
    Pipe,
    PipeToGround,
    Pump,
    Chest,
    PowerPole,
}

pub fn hint_for(name: &str) -> Option<KindHint> {
    if let Some(hint) = hint_for_exact(name) {
        return Some(hint);
    }
    hint_for_family(name)
}

fn hint_for_exact(name: &str) -> Option<KindHint> {
    Some(match name {
        "transport-belt" | "fast-transport-belt" | "express-transport-belt"
        | "turbo-transport-belt" => KindHint::TransportBelt,
        "underground-belt" | "fast-underground-belt" | "express-underground-belt"
        | "turbo-underground-belt" => KindHint::UndergroundBelt,
        "splitter" | "fast-splitter" | "express-splitter" | "turbo-splitter" => {
            KindHint::Splitter
        }
        "inserter" | "fast-inserter" | "long-handed-inserter" | "filter-inserter"
        | "stack-inserter" | "stack-filter-inserter" | "bulk-inserter" | "burner-inserter" => {
            KindHint::Inserter
        }
        "pipe" => KindHint::Pipe,
        "pipe-to-ground" => KindHint::PipeToGround,
        "pump" | "offshore-pump" => KindHint::Pump,
        "small-electric-pole" | "medium-electric-pole" | "big-electric-pole" | "substation" => {
            KindHint::PowerPole
        }
        _ => return None,
    })
}

fn hint_for_family(name: &str) -> Option<KindHint> {
    if name.starts_with("assembling-machine") || name == "chemical-plant" || name == "centrifuge"
        || name == "oil-refinery" || name == "electromagnetic-plant" || name == "biochamber"
        || name == "cryogenic-plant" || name == "foundry"
    {
        return Some(KindHint::AssemblingMachine);
    }
    if name.ends_with("-furnace") || name == "recycler" {
        return Some(KindHint::Furnace);
    }
    if name.ends_with("-mining-drill") || name == "pumpjack" {
        return Some(KindHint::MiningDrill);
    }
    if name.ends_with("-chest") || name == "steel-chest" || name == "wooden-chest" {
        return Some(KindHint::Chest);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_assembling_machine_resolves() {
        assert_eq!(hint_for("assembling-machine-2"), Some(KindHint::AssemblingMachine));
    }

    #[test]
    fn inserter_family_resolves() {
        assert_eq!(hint_for("fast-inserter"), Some(KindHint::Inserter));
        assert_eq!(hint_for("bulk-inserter"), Some(KindHint::Inserter));
    }

    #[test]
    fn unknown_returns_none() {
        assert_eq!(hint_for("radar"), None);
    }
}
