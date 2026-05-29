use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::ExitCode;

use prototypes::{dump, ItemId};
use tracing_subscriber::EnvFilter;

use blueprint::World;
use planner::{plan, PlanConfig, PlanRequest, ProductionLine, Rate};

use layout::{solve, LayoutConfig};

fn main() -> ExitCode {
    init_tracing();
    let path = dump_path();
    let db = match dump::load_from_path(&path) {
        Ok(db) => db,
        Err(err) => {
            eprintln!("failed to load {}: {err}", path.display());
            return ExitCode::FAILURE;
        }
    };
    let config = PlanConfig::new()
        .with_raw(ItemId::IRON_ORE)
        .with_raw(ItemId::COPPER_ORE);
    let request = PlanRequest::new()
        .want(ItemId::ELECTRONIC_CIRCUIT, Rate::per_minute(60.0))
        .with_config(config);
    print_targets(&request);
    let line = match plan(&db, &request) {
        Ok(line) => line,
        Err(err) => {
            eprintln!("planning failed: {err}");
            return ExitCode::FAILURE;
        }
    };
    print_plan(&line);
    let world = match solve(&db, &line, &LayoutConfig::new(20, 20)) {
        Ok(world) => world,
        Err(err) => {
            eprintln!("layout failed: {err}");
            return ExitCode::FAILURE;
        }
    };
    print_layout(&world);
    let output = png_path();
    if let Err(err) = world.render().export_as_png(&output) {
        eprintln!("png export failed: {err}");
        return ExitCode::FAILURE;
    }
    println!("\nwrote {}", output.display());
    ExitCode::SUCCESS
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
}

fn dump_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../crates/prototypes/resources/data-raw-dump.json")
}

fn png_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../plan-and-layout.png")
}

fn print_targets(request: &PlanRequest) {
    println!("== Targets ==");
    for (resource, rate) in &request.targets {
        println!("  {resource}: {rate}");
    }
    println!();
}

fn print_plan(line: &ProductionLine) {
    println!("== Production Line ==");
    println!("  {line}\n");
    print_machine_totals(line);
    print_nodes(line);
    print_edges(line);
    print_raw_inputs(line);
    print_outputs(line);
}

fn print_machine_totals(line: &ProductionLine) {
    let mut totals: BTreeMap<String, f64> = BTreeMap::new();
    for node in &line.nodes {
        *totals.entry(node.machine.to_string()).or_default() += node.machines_needed;
    }
    println!("== Machine Totals ({}) ==", totals.len());
    for (machine, count) in totals {
        println!("  {count:>7.2} × {machine}");
    }
    println!();
}

fn print_nodes(line: &ProductionLine) {
    let mut nodes: Vec<_> = line.nodes.iter().collect();
    nodes.sort_by(|a, b| a.recipe.as_str().cmp(b.recipe.as_str()));
    println!("== Nodes ({}) ==", nodes.len());
    for node in nodes {
        println!("  {node}");
    }
    println!();
}

fn print_edges(line: &ProductionLine) {
    let mut edges: Vec<_> = line.edges.iter().collect();
    edges.sort_by(|a, b| {
        a.resource
            .as_str()
            .cmp(b.resource.as_str())
            .then_with(|| a.from.to_string().cmp(&b.from.to_string()))
    });
    println!("== Edges ({}) ==", edges.len());
    for edge in edges {
        println!("  {edge}");
    }
    println!();
}

fn print_raw_inputs(line: &ProductionLine) {
    let mut entries: Vec<_> = line.raw_inputs.iter().collect();
    entries.sort_by(|(a, _), (b, _)| a.as_str().cmp(b.as_str()));
    println!("== Raw Inputs ({}) ==", entries.len());
    for (resource, rate) in entries {
        println!("  {resource}: {rate}");
    }
    println!();
}

fn print_outputs(line: &ProductionLine) {
    let mut entries: Vec<_> = line.outputs.iter().collect();
    entries.sort_by(|(a, _), (b, _)| a.as_str().cmp(b.as_str()));
    println!("== Outputs ({}) ==", entries.len());
    for (resource, rate) in entries {
        println!("  {resource}: {rate}");
    }
}

fn print_layout(world: &World) {
    println!("\n== Layout ({} entities) ==", world.len());
    for entity in world.entities() {
        println!("  {entity}");
    }
}
