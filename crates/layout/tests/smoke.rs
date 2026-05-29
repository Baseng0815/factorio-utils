use std::collections::HashMap;

use blueprint::entity::EntityKind;
use planner::{NodeId, ProductionLine, ProductionNode};
use prototypes::{CraftingCategory, Database, Machine, MachineId, MachineKind, RecipeId};

use layout::{solve, Error, LayoutConfig};

fn assembler() -> Machine {
    Machine {
        kind: MachineKind::AssemblingMachine,
        crafting_categories: vec![CraftingCategory::from("crafting")],
        crafting_speed: 1.0,
        module_slots: 0,
        energy_usage_watts: 0.0,
        tile_width: 3,
        tile_height: 3,
        icon: None,
    }
}

fn tiny_db() -> Database {
    let mut db = Database::new();
    db.machines
        .insert(MachineId::from("assembling-machine-1"), assembler());
    db
}

fn two_node_line() -> ProductionLine {
    ProductionLine {
        nodes: vec![
            ProductionNode {
                id: NodeId::new(0),
                recipe: RecipeId::from("iron-gear-wheel"),
                machine: MachineId::from("assembling-machine-1"),
                runs_per_second: 1.0,
                machines_needed: 1.0,
            },
            ProductionNode {
                id: NodeId::new(1),
                recipe: RecipeId::from("electronic-circuit"),
                machine: MachineId::from("assembling-machine-1"),
                runs_per_second: 1.0,
                machines_needed: 1.0,
            },
        ],
        edges: vec![],
        raw_inputs: HashMap::new(),
        outputs: HashMap::new(),
    }
}

#[test]
fn places_two_machines_without_overlap() {
    let db = tiny_db();
    let line = two_node_line();
    let world = solve(&db, &line, &LayoutConfig::new(10, 10)).expect("expected SAT");
    assert_eq!(world.len(), 2);
    let positions: Vec<_> = world.entities().map(|e| e.position).collect();
    let dx = (positions[0].x - positions[1].x).abs();
    let dy = (positions[0].y - positions[1].y).abs();
    assert!(
        dx >= 3.0 || dy >= 3.0,
        "machines overlap: {:?} vs {:?}",
        positions[0],
        positions[1],
    );
    for e in world.entities() {
        assert!(matches!(e.kind, EntityKind::AssemblingMachine(_)));
    }
}

#[test]
fn returns_unsat_for_tiny_grid() {
    let db = tiny_db();
    let line = two_node_line();
    let err = solve(&db, &line, &LayoutConfig::new(3, 3)).unwrap_err();
    assert!(matches!(err, Error::Unsat { width: 3, height: 3 }));
}

#[test]
fn fits_machines_inside_bounded_grid() {
    let db = tiny_db();
    let line = two_node_line();
    let world = solve(&db, &line, &LayoutConfig::new(10, 10)).expect("expected SAT");
    for e in world.entities() {
        let half = 1.5;
        assert!(e.position.x - half >= 0.0 && e.position.x + half <= 10.0);
        assert!(e.position.y - half >= 0.0 && e.position.y + half <= 10.0);
    }
}
