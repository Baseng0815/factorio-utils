use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use recipes::{dump, ItemId, RecipeId, ResourceId};
use tracing_subscriber::EnvFilter;

use planner::export::write_dot;
use planner::{plan, FactorioInstall, IconResolver, PlanConfig, PlanRequest, Rate};

const DEFAULT_TARGET: &str = "electronic-circuit";
const DEFAULT_RATE_PER_MIN: f64 = 60.0;

fn main() -> ExitCode {
    init_tracing();
    let (target, rate_per_min, factorio_dir) = parse_args();
    let path = dump_path();
    let db = match dump::load_from_path(&path) {
        Ok(db) => db,
        Err(err) => {
            eprintln!("failed to load {}: {err}", path.display());
            return ExitCode::FAILURE;
        }
    };
    let request = build_request(&target, rate_per_min);
    let line = match plan(&db, &request) {
        Ok(line) => line,
        Err(err) => {
            eprintln!("planning failed: {err}");
            return ExitCode::FAILURE;
        }
    };
    let resolver: Option<Box<dyn IconResolver>> =
        factorio_dir.map(|p| Box::new(FactorioInstall::new(p)) as Box<dyn IconResolver>);
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    if let Err(err) = write_dot(&line, &db, resolver.as_deref(), &mut out) {
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

fn parse_args() -> (String, f64, Option<PathBuf>) {
    let mut args = std::env::args().skip(1);
    let target = args.next().unwrap_or_else(|| {
        eprintln!(
            "usage: export-dot [ITEM] [RATE_PER_MIN] [FACTORIO_DIR] \
             (defaulting to {DEFAULT_TARGET} at {DEFAULT_RATE_PER_MIN}/min, no icons)"
        );
        DEFAULT_TARGET.to_owned()
    });
    let rate = args
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_RATE_PER_MIN);
    let factorio_dir = args.next().map(PathBuf::from);
    (target, rate, factorio_dir)
}

fn build_request(target: &str, rate_per_min: f64) -> PlanRequest {
    let config = PlanConfig::new()
        .with_recipe(
            ResourceId::Item(ItemId::from("electronic-circuit")),
            RecipeId::from("electronic-circuit"),
        )
        .with_recipe(
            ResourceId::Item(ItemId::from("iron-plate")),
            RecipeId::from("iron-plate"),
        )
        .with_recipe(
            ResourceId::Item(ItemId::from("copper-cable")),
            RecipeId::from("copper-cable"),
        )
        .with_recipe(
            ResourceId::Item(ItemId::from("copper-plate")),
            RecipeId::from("copper-plate"),
        )
        .with_raw(ResourceId::Item(ItemId::from("iron-ore")))
        .with_raw(ResourceId::Item(ItemId::from("copper-ore")));
    PlanRequest::new()
        .want(
            ResourceId::Item(ItemId::from(target)),
            Rate::per_minute(rate_per_min),
        )
        .with_config(config)
}

