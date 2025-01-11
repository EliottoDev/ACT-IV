#[cfg(test)]
mod lister_tests {
    use std::fs;
    use std::fs::remove_dir_all;
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use crate::library::config::Configuration;
    use crate::library::lister::{Lister, LISTER_FILE_NAME};

    // Helper function to create a mock routine TOML file
    fn create_mock_routine(file_path: &Path) {
        let routine_content = r#"[base]
title = "Notes"
path = "/path/to/notes"

[base.interval]
commit_interval = "1d"
sync_method = "thorough"
timestamp = true
time_zone = "UTC"

[crypt]
encrypt = true
password = "secretpassword"

[press]
compression = "gz"

[advanced]
exclude = ["*.tmp", "cache/", "logs/"]
sync_on_startup = true

[ntfy]
ntfy_topic = "sync-updates"
notification_on_success = true
notification_on_failure = true

[git]
remote = "https://github.com/exam/ple"
branch = "main"
force_push = false
"#;

        let mut file = fs::File::create(file_path).unwrap();
        file.write_all(routine_content.as_bytes()).unwrap();
    }

    #[test]
    fn test_new_creates_lister_with_empty_file() {
        let config_dir = Configuration::get_dir().unwrap();

        let lister_file = config_dir.join(LISTER_FILE_NAME);
        let lister = Lister::new();

        assert!(lister.is_ok());
        assert!(lister_file.exists());

        remove_dir_all(&config_dir).unwrap();
    }

    #[test]
    fn test_initialize_paths_with_toml_files() {
        let config_dir = Configuration::get_dir().unwrap();

        // Crea un archivo .toml simulado
        let file_1 = config_dir.join("notes.toml");
        create_mock_routine(&file_1);

        let lister = Lister::new().unwrap();

        assert!(lister.paths.contains_key("Notes"));
        assert_eq!(lister.paths.get("Notes").unwrap(), &dunce::canonicalize(file_1).unwrap());

        remove_dir_all(&config_dir).unwrap();
    }

    #[test]
    fn test_parse_lister_file_with_valid_yaml() {
        let yaml_data = r#"
Notes: ./tasks.toml
Tasks: ./tasks.toml
"#;

        let buffer = yaml_data.as_bytes();
        let parsed_paths = Lister::parse_lister_file(buffer);

        assert!(parsed_paths.is_ok());
        let paths = parsed_paths.unwrap();
        assert_eq!(paths.get("Notes").unwrap(), &dunce::canonicalize(Configuration::get_dir().unwrap().join("notes.toml")).unwrap());
        assert_eq!(paths.get("Tasks").unwrap(), &dunce::canonicalize(Configuration::get_dir().unwrap().join("tasks.toml")).unwrap());
    }

    #[test]
    fn test_add_file_adds_new_entry() {
        let config_dir = Configuration::get_dir().unwrap();

        let mut lister = Lister::new().unwrap();
        let file_path = config_dir.join("new_routine.toml");
        create_mock_routine(&file_path);

        // Agrega un archivo con un nombre personalizado
        lister.add_file(&file_path, Some("CustomName".to_string()));

        assert!(lister.paths.contains_key("CustomName"));
        assert_eq!(lister.paths.get("CustomName").unwrap(), &file_path);

        remove_dir_all(&config_dir).unwrap();
    }

    #[test]
    fn test_dump_writes_to_file() {
        let config_dir = Configuration::get_dir().unwrap();

        let mut lister = Lister::new().unwrap();
        let file_path = config_dir.join("test_dump.toml");
        create_mock_routine(&file_path);

        lister.add_file(&file_path, Some("DumpTest".to_string()));

        let result = lister.dump();
        assert!(result.is_ok());

        let lister_file_content = fs::read_to_string(lister.file_path).unwrap();
        assert!(lister_file_content.contains("DumpTest"));

        remove_dir_all(&config_dir).unwrap();
    }
}
