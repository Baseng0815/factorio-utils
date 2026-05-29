use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use recipes::{dump, ItemId};
use tracing_subscriber::EnvFilter;

use planner::export::write_dot;
use planner::{plan, FactorioInstall, IconResolver, PlanConfig, PlanRequest, Rate};

const FACTORIO_DIR: &str = "/home/bastian/.steam/steam/steamapps/common/Factorio";

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
    let line = match plan(&db, &request) {
        Ok(line) => line,
        Err(err) => {
            eprintln!("planning failed: {err}");
            return ExitCode::FAILURE;
        }
    };
    let resolver: Box<dyn IconResolver> =
        Box::new(FactorioInstall::new(FACTORIO_DIR)) as Box<dyn IconResolver>;
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    if let Err(err) = write_dot(&line, &db, Some(resolver.as_ref()), &mut out) {
        eprintln!("dot write failed: {err}");
        return ExitCode::FAILURE;
    }
    if let Err(err) = out.flush() {
        eprintln!("flush failed: {err}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_writer(io::stderr)
        .init();
}

fn dump_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../recipes/resources/data-raw-dump.json")
}
