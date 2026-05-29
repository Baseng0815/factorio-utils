use std::env;
use std::process::ExitCode;

use blueprint::decode_string;

fn main() -> ExitCode {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .try_init();

    let mut args = env::args().skip(1);
    let Some(blueprint_string) = args.next() else {
        eprintln!("usage: render-string <blueprint-string> [output.png]");
        return ExitCode::FAILURE;
    };
    let output = args.next().unwrap_or_else(|| "blueprint.png".to_owned());

    let world = match decode_string(&blueprint_string) {
        Ok(w) => w,
        Err(err) => {
            eprintln!("decode failed: {}", err);
            return ExitCode::FAILURE;
        }
    };

    println!("decoded {} entities", world.len());

    let rendered = world.render();
    println!("rendered {}x{} pixels", rendered.width(), rendered.height());

    if let Err(err) = rendered.export_as_png(&output) {
        eprintln!("export failed: {}", err);
        return ExitCode::FAILURE;
    }

    println!("wrote {}", output);
    ExitCode::SUCCESS
}
