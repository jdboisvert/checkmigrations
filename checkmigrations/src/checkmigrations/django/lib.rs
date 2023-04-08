use std::fs;

use super::super::utils;

pub fn check_migrations(path: &str) {
    // Your code to check migrations for the given Django app.
    println!("Checking migrations for Django app at path: {}", path);

    // Check if the migrations directory exists
    let migrations_dir_path = format!("{}/migrations", path);
    if !std::path::Path::new(&migrations_dir_path).exists() {
        eprintln!("No migrations directory found at path: {}", migrations_dir_path);
        std::process::exit(1);
    }

    // Check if the migrations directory has any duplicate migration files (e.g. 0001 prefix for multiple files)
    let mut migration_files = Vec::new();
    match fs::read_dir(migrations_dir_path) {
        Ok(entries) => {
            for entry_result in entries {
                match entry_result {
                    Ok(entry) => {
                        let file_name = entry.file_name().into_string().unwrap();
                        let prefix = file_name.split("_").next().unwrap();

                        migration_files.push(prefix.to_owned()); // Clone the prefix string
                    }
                    Err(err) => {
                        eprintln!("Error reading file: {}", err);
                        std::process::exit(1);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading directory: {}", err);
            std::process::exit(1);
        }
    }

    if utils::lib::has_duplicates(&migration_files) {
        eprintln!("Duplicate migration files found");
        std::process::exit(1);
    }
}
