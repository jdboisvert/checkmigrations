use clap::Parser;

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
    println!("framework: {}", args.framework);
    println!("path_to_app: {}", args.path_to_app.display());

    let framework = args.framework; 
    let path = args.path_to_app;

    match framework.as_str() {
        DJANGO => {
            println!("django was selected!");
            println!("path: {}", path.display());
            // checkmigrations::django::check_migrations(path);
        },
        _ => {
            println!("unknown framework");
        }
    }

}
