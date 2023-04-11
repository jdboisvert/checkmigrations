use crate::checkmigrations::django::lib::{check_migrations, get_migration_files};

use std::fs;
use tempfile::TempDir;

#[cfg(test)]
mod tests {
    use crate::checkmigrations::django::lib::MIGRATION_FILE_NAME_DELIMITER;

    use super::*;

    fn create_temp_migration_files(temp_dir: &TempDir, file_names: &[&str]) {
        let migrations_dir = temp_dir.path().join("migrations");
        fs::create_dir(&migrations_dir).expect("Failed to create 'migrations' directory");

        for file_name in file_names {
            let file_path = migrations_dir.join(file_name);
            println!("file_path: {:?}", file_path);
            fs::File::create(file_path).expect("Failed to create temporary migration file");
        }
    }

    #[test]
    fn test_get_migration_files() {
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let file_names = ["0001_initial.py", "0002_second.py", "0003_third.py"];
        create_temp_migration_files(&temp_dir, &file_names);

        let migration_files =
            get_migration_files(temp_dir.path().join("migrations").to_str().unwrap());

        assert_eq!(
            migration_files.len(),
            file_names.len(),
            "Expected to find all migration files"
        );
        for file_name in file_names {
            let prefix = file_name
                .split(MIGRATION_FILE_NAME_DELIMITER)
                .next()
                .unwrap();
            assert!(
                migration_files.contains(&prefix.to_owned()),
                "Expected to find migration file with prefix: {}",
                prefix
            );
        }
    }

    #[test]
    fn test_check_migrations_no_duplicates() {
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let file_names = ["0001_initial.py", "0002_second.py", "0003_third.py"];
        create_temp_migration_files(&temp_dir, &file_names);

        let result = check_migrations(temp_dir.path().to_str().unwrap());
        assert!(result.is_ok(), "Expected no duplicate migration files");
    }

    #[test]
    fn test_check_migrations_with_duplicates() {
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let file_names = ["0001_initial.py", "0002_second.py", "0001_duplicate.py"];
        create_temp_migration_files(&temp_dir, &file_names);

        let result = check_migrations(temp_dir.path().to_str().unwrap());

        assert!(result.is_err(), "Expected duplicate migration files");
        assert_eq!(result.unwrap_err(), "Duplicate migration files found");
    }
}
