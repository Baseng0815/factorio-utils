use std::path::PathBuf;
use std::process::ExitCode;

use recipes::{Database, dump};
use tracing_subscriber::EnvFilter;

fn main() -> ExitCode {
    init_tracing();
    let path = dump_path();
    match dump::load_from_path(&path) {
        Ok(db) => {
            print_database(&db);
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("failed to load {}: {err}", path.display());
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
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources/data-raw-dump.json")
}

fn print_database(db: &Database) {
    println!("{db}\n");
    print_items(db);
    print_fluids(db);
    print_machines(db);
    print_recipes(db);
}

fn print_items(db: &Database) {
    let mut items: Vec<_> = db.items.values().collect();
    items.sort_by(|a, b| a.id.as_str().cmp(b.id.as_str()));
    println!("== Items ({}) ==", items.len());
    for item in items {
        println!("  {item}");
    }
    println!();
}

fn print_fluids(db: &Database) {
    let mut fluids: Vec<_> = db.fluids.values().collect();
    fluids.sort_by(|a, b| a.id.as_str().cmp(b.id.as_str()));
    println!("== Fluids ({}) ==", fluids.len());
    for fluid in fluids {
        println!("  {fluid}");
    }
    println!();
}

fn print_machines(db: &Database) {
    let mut machines: Vec<_> = db.machines.values().collect();
    machines.sort_by(|a, b| {
        a.kind
            .to_string()
            .cmp(&b.kind.to_string())
            .then_with(|| a.id.as_str().cmp(b.id.as_str()))
    });
    println!("== Machines ({}) ==", machines.len());
    for machine in machines {
        println!("  {machine}");
    }
    println!();
}

fn print_recipes(db: &Database) {
    let mut recipes: Vec<_> = db.recipes.values().collect();
    recipes.sort_by(|a, b| {
        a.category
            .as_str()
            .cmp(b.category.as_str())
            .then_with(|| a.id.as_str().cmp(b.id.as_str()))
    });
    println!("== Recipes ({}) ==", recipes.len());
    let mut current_category: Option<&str> = None;
    for recipe in recipes {
        let cat = recipe.category.as_str();
        if current_category != Some(cat) {
            println!("  [{cat}]");
            current_category = Some(cat);
        }
        println!("    {recipe}");
    }
}
