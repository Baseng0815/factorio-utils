use std::path::PathBuf;
use std::process::ExitCode;

use tracing_subscriber::EnvFilter;

use planner::{plan, PlanConfig, PlanRequest, Rate};
use prototypes::{dump, ItemId};

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
    let request = PlanRequest::new()
        .want(ItemId::ELECTRONIC_CIRCUIT, Rate::per_minute(60.0))
        .with_config(
            PlanConfig::new()
                .with_raw(ItemId::IRON_ORE)
                .with_raw(ItemId::COPPER_ORE),
        );
    let line = match plan(&db, &request) {
        Ok(l) => l,
        Err(err) => {
            eprintln!("plan failed: {err}");
            return ExitCode::FAILURE;
        }
    };
    println!("planned: {line}");
    let world = match solve(&db, &line, &LayoutConfig::new(20, 20)) {
        Ok(w) => w,
        Err(err) => {
            eprintln!("layout failed: {err}");
            return ExitCode::FAILURE;
        }
    };
    println!("placed {} entities", world.len());
    let output = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../layout-circuits.png");
    if let Err(err) = world.render().export_as_png(&output) {
        eprintln!("png export failed: {err}");
        return ExitCode::FAILURE;
    }
    println!("wrote {}", output.display());
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
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../prototypes/resources/data-raw-dump.json")
}
