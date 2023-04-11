use clap::Parser;
mod checkmigrations;

#[derive(Parser)]
struct Cli {
    // The framework to use
    framework: String,
    // The path to the migrations directory to check
    path_to_app: std::path::PathBuf,
}

const DJANGO: &str = "django";

fn main() {
    let args = Cli::parse();

    let framework = args.framework;
    let path = args.path_to_app;

    match framework.as_str() {
        DJANGO => match checkmigrations::django::lib::check_migrations(path.to_str().unwrap()) {
            Ok(()) => println!("No duplicate migration files found"),
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        },
        _ => {
            println!("unknown framework");
            std::process::exit(1);
        }
    }
}
