use blueprint::entity::{
    AssemblingMachine, Chest, Furnace, Inserter, MiningDrill, Splitter, TransportBelt,
    UndergroundBelt,
};
use blueprint::{Entity, World, decode_string, encode_string};
use blueprint::world::Direction;

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("off")),
        )
        .with_test_writer()
        .try_init();
}

fn sample_world() -> World {
    let mut world = World::new();
    world.label = Some("test blueprint".into());
    world.version = Some(562949956763648);
    world.add(
        Entity::new(
            "assembling-machine-2",
            (3.5, 3.5),
            AssemblingMachine::default()
                .with_recipe("iron-gear-wheel")
                .with_module("speed-module", 2),
        )
        .with_direction(Direction::East),
    );
    world.add(Entity::new(
        "fast-inserter",
        (5.5, 3.5),
        Inserter::default().with_filter("iron-gear-wheel"),
    ));
    world.add(Entity::new("transport-belt", (6.5, 3.5), TransportBelt));
    world.add(Entity::new(
        "underground-belt",
        (7.5, 3.5),
        UndergroundBelt::input(),
    ));
    world.add(Entity::new("steel-furnace", (10.5, 3.5), Furnace::default()));
    world.add(Entity::new(
        "electric-mining-drill",
        (13.5, 13.5),
        MiningDrill::default().with_module("productivity-module", 3),
    ));
    world.add(Entity::new(
        "splitter",
        (8.0, 4.0),
        Splitter::default().with_filter("iron-plate"),
    ));
    world.add(Entity::new("iron-chest", (9.5, 3.5), Chest::default().with_bar(8)));
    world
}

#[test]
fn world_roundtrips_through_blueprint_string() {
    init_tracing();
    let world = sample_world();
    let s = encode_string(&world).unwrap();
    assert!(s.starts_with('0'));
    let decoded = decode_string(&s).unwrap();
    assert_eq!(decoded.len(), world.len());
    let orig: Vec<_> = world.entities_sorted_by_number().collect();
    let back: Vec<_> = decoded.entities_sorted_by_number().collect();
    for (a, b) in orig.iter().zip(back.iter()) {
        assert_eq!(a.name, b.name, "entity name mismatch for {}", a.number);
        assert_eq!(a.position, b.position, "position mismatch for {}", a.number);
        assert_eq!(a.direction, b.direction, "direction mismatch for {}", a.number);
        assert_eq!(a.kind, b.kind, "kind mismatch for {}", a.number);
    }
    assert_eq!(decoded.label, world.label);
    assert_eq!(decoded.version, world.version);
}

#[test]
fn decoding_back_after_encoding_is_idempotent() {
    init_tracing();
    let world = sample_world();
    let s1 = encode_string(&world).unwrap();
    let world2 = decode_string(&s1).unwrap();
    let s2 = encode_string(&world2).unwrap();
    let world3 = decode_string(&s2).unwrap();
    assert_eq!(world2.len(), world3.len());
}
