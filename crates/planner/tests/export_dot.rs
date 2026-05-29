use std::path::PathBuf;

use recipes::{Database, IconRef, ResourceId, dump};

use planner::export::write_dot;
use planner::{IconResolver, PlanConfig, PlanRequest, Rate, plan};

const DUMP: &str = r#"{
    "item": {
        "iron-ore":   { "stack_size": 50,  "icon": "__base__/icons/iron-ore.png",   "icon_size": 64 },
        "iron-plate": { "stack_size": 100, "icon": "__base__/icons/iron-plate.png", "icon_size": 64 }
    },
    "fluid": {},
    "furnace": {
        "stone-furnace": {
            "crafting_categories": ["smelting"],
            "crafting_speed": 1.0,
            "energy_usage": "90kW",
            "icon": "__base__/icons/stone-furnace.png",
            "icon_size": 64
        }
    },
    "mining-drill": {
        "electric-mining-drill": {
            "resource_categories": ["basic-solid"],
            "mining_speed": 0.5,
            "energy_usage": "90kW"
        }
    },
    "recipe": {
        "iron-plate": {
            "category": "smelting",
            "energy_required": 3.2,
            "ingredients": [{"type":"item","name":"iron-ore","amount":1}],
            "results":     [{"type":"item","name":"iron-plate","amount":1}]
        }
    },
    "resource": {
        "iron-ore": { "category": "basic-solid", "minable": { "mining_time": 1.0, "result": "iron-ore" } }
    }
}"#;

fn item(name: &str) -> ResourceId {
    ResourceId::Item(name.into())
}

fn load_db() -> Database {
    dump::from_str(DUMP).unwrap()
}

fn build_line(db: &Database) -> planner::ProductionLine {
    let request = PlanRequest::new().want(item("iron-plate"), Rate::per_second(1.0));
    plan(db, &request).unwrap()
}

#[test]
fn writes_digraph_header_and_closer() {
    let db = load_db();
    let line = build_line(&db);
    let mut out = Vec::new();
    write_dot(&line, &db, None, &mut out).unwrap();
    let text = String::from_utf8(out).unwrap();
    assert!(text.starts_with("digraph ProductionLine {"));
    assert!(text.trim_end().ends_with('}'));
}

#[test]
fn includes_machine_node_per_production_node() {
    let db = load_db();
    let line = build_line(&db);
    let mut out = Vec::new();
    write_dot(&line, &db, None, &mut out).unwrap();
    let text = String::from_utf8(out).unwrap();
    for node in &line.nodes {
        let id = format!("n{}", node.id.index());
        assert!(
            text.contains(&format!("    {id} [label=<")),
            "missing node {id} in output:\n{text}",
        );
    }
}

#[test]
fn includes_source_and_sink_nodes() {
    let db = load_db();
    let line = build_line(&db);
    let mut out = Vec::new();
    write_dot(&line, &db, None, &mut out).unwrap();
    let text = String::from_utf8(out).unwrap();
    for resource in line.raw_inputs.keys() {
        assert!(text.contains(&format!("\"src:{}\"", resource.as_str())));
    }
    for resource in line.outputs.keys() {
        assert!(text.contains(&format!("\"sink:{}\"", resource.as_str())));
    }
}

#[test]
fn includes_edge_per_production_edge() {
    let db = load_db();
    let line = build_line(&db);
    let mut out = Vec::new();
    write_dot(&line, &db, None, &mut out).unwrap();
    let text = String::from_utf8(out).unwrap();
    assert_eq!(text.matches(" -> ").count(), line.edges.len());
    for edge in &line.edges {
        assert!(text.contains(edge.resource.as_str()));
    }
}

#[test]
fn omits_img_tags_when_no_resolver() {
    let db = load_db();
    let line = build_line(&db);
    let mut out = Vec::new();
    write_dot(&line, &db, None, &mut out).unwrap();
    let text = String::from_utf8(out).unwrap();
    assert!(!text.contains("<IMG"), "unexpected IMG tag without resolver:\n{text}");
}

struct FixedResolver;

impl IconResolver for FixedResolver {
    fn resolve(&self, icon: &IconRef) -> Option<PathBuf> {
        Some(PathBuf::from("/icons").join(
            icon.path
                .trim_start_matches("__base__/")
                .trim_start_matches('/'),
        ))
    }
}

#[test]
fn emits_img_tags_when_resolver_provided() {
    let db = load_db();
    let config = PlanConfig::new().with_raw(item("iron-ore"));
    let request = PlanRequest::new()
        .want(item("iron-plate"), Rate::per_second(1.0))
        .with_config(config);
    let line = plan(&db, &request).unwrap();
    let mut out = Vec::new();
    write_dot(&line, &db, Some(&FixedResolver), &mut out).unwrap();
    let text = String::from_utf8(out).unwrap();
    assert!(text.contains("<IMG"), "expected IMG tag with resolver:\n{text}");
    assert!(text.contains("/icons/icons/stone-furnace.png"));
    assert!(text.contains("/icons/icons/iron-ore.png"));
    assert!(text.contains("/icons/icons/iron-plate.png"));
}

#[test]
fn html_escapes_special_characters_in_resource_names() {
    let mut db = Database::new();
    db.items.insert(
        "weird&name".into(),
        recipes::Item {
            stack_size: 1,
            icon: None,
        },
    );
    let mut line = planner::ProductionLine {
        nodes: Vec::new(),
        edges: Vec::new(),
        raw_inputs: Default::default(),
        outputs: Default::default(),
    };
    line.outputs.insert(item("weird&name"), Rate::per_second(1.0));
    let mut out = Vec::new();
    write_dot(&line, &db, None, &mut out).unwrap();
    let text = String::from_utf8(out).unwrap();
    assert!(text.contains("weird&amp;name"));
}
