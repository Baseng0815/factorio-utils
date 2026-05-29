use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::ExitCode;

use recipes::{ItemId, ResourceId, dump};
use tracing_subscriber::EnvFilter;

use planner::{PlanRequest, ProductionLine, Rate, plan};

const DEFAULT_TARGET: &str = "electronic-circuit";
const DEFAULT_RATE_PER_MIN: f64 = 60.0;

fn main() -> ExitCode {
    init_tracing();
    let (target, rate_per_min) = parse_args();
    let path = dump_path();
    let db = match dump::load_from_path(&path) {
        Ok(db) => db,
        Err(err) => {
            eprintln!("failed to load {}: {err}", path.display());
            return ExitCode::FAILURE;
        }
    };
    let request = PlanRequest::new().want(
        ResourceId::Item(ItemId::from(target.as_str())),
        Rate::per_minute(rate_per_min),
    );
    print_targets(&request);
    match plan(&db, &request) {
        Ok(line) => {
            print_plan(&line);
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("planning failed: {err}");
            ExitCode::FAILURE
        }
    }
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
}

fn dump_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../recipes/resources/database-dump.json")
}

fn parse_args() -> (String, f64) {
    let mut args = std::env::args().skip(1);
    let target = args.next().unwrap_or_else(|| {
        eprintln!(
            "usage: plan-target [ITEM] [RATE_PER_MIN] \
             (defaulting to {DEFAULT_TARGET} at {DEFAULT_RATE_PER_MIN}/min)\n"
        );
        DEFAULT_TARGET.to_owned()
    });
    let rate = args
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_RATE_PER_MIN);
    (target, rate)
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
            .name()
            .cmp(b.resource.name())
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
    entries.sort_by(|(a, _), (b, _)| a.name().cmp(b.name()));
    println!("== Raw Inputs ({}) ==", entries.len());
    for (resource, rate) in entries {
        println!("  {resource}: {rate}");
    }
    println!();
}

fn print_outputs(line: &ProductionLine) {
    let mut entries: Vec<_> = line.outputs.iter().collect();
    entries.sort_by(|(a, _), (b, _)| a.name().cmp(b.name()));
    println!("== Outputs ({}) ==", entries.len());
    for (resource, rate) in entries {
        println!("  {resource}: {rate}");
    }
}
