use clap::Parser;

#[derive(Parser)]
struct Cli {
    // The framework to use
    framework: String,
    // The path to the migrations directory to check
    path_to_app: std::path::PathBuf,
}


fn main() {
    let args = Cli::parse();
    println!("framework: {}", args.framework);
    println!("path_to_app: {}", args.path_to_app.display());
}
