use std::env;
use std::process::ExitCode;

use blueprint::{decode_string, Entity};

fn main() -> ExitCode {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .try_init();

    let Some(arg) = env::args().nth(1) else {
        eprintln!("usage: decode-string <blueprint-string>");
        return ExitCode::FAILURE;
    };

    match decode_string(&arg) {
        Ok(world) => {
            println!("decoded {} entities", world.len());
            if let Some(label) = &world.label {
                println!("label: {}", label);
            }
            for e in world.entities_sorted_by_number() {
                print_entity(e);
            }
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("decode failed: {}", err);
            ExitCode::FAILURE
        }
    }
}

fn print_entity(e: &Entity) {
    println!("{}", e);
}
