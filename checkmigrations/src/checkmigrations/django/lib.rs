use std::fs;

use super::super::utils;

const MIGRATION_FILE_NAME_DELIMITER: &str = "_";
const DJANGO_MIGRATION_DIR: &str = "migrations";

fn get_migration_files(path: &str) -> Vec<String> {
    let mut migration_files = Vec::new();
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry_result in entries {
                match entry_result {
                    Ok(entry) => {
                        let file_name = entry.file_name().into_string().unwrap();
                        let prefix = file_name
                            .split(MIGRATION_FILE_NAME_DELIMITER)
                            .next()
                            .unwrap();

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

    migration_files
}

pub fn check_migrations(path: &str) -> Result<(), String> {
    println!("Checking migrations for Django app at path: {}", path);

    let migrations_dir_path = format!("{}/{}", path, DJANGO_MIGRATION_DIR);
    if !std::path::Path::new(&migrations_dir_path).exists() {
        return Err(format!(
            "No migrations directory found at path: {}",
            migrations_dir_path
        ));
    }

    // Check if the migrations directory has any duplicate migration files (e.g. 0001 prefix for multiple files)
    let migration_files = get_migration_files(&migrations_dir_path);

    if utils::lib::has_duplicates(&migration_files) {
        return Err(String::from("Duplicate migration files found"));
    }

    Ok(())
}

#[cfg(test)]
#[path = "./lib_tests.rs"]
mod lib_tests;
