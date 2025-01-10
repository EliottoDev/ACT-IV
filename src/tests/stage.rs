#[cfg(test)]
mod routine_tests {
    // Importing the necessary code
    use crate::library::routine::Routine;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_routine_read_success() {
        // Create a temporary directory to write the routine file
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("routine.toml");

        // Prepare a simple TOML content for the test
        let toml_content = r#"
            [base]
            title = "Notes"
            path = "/path/to/notes"
            interval = { commit_interval = "1d", sync_method = "thorough", timestamp = true, time_zone = "UTC" }

            [crypt]
            encrypt = true
            password = "secretpassword"
            password_eval = "/path/to/command"

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

        // Write the TOML content to the file
        let mut file = File::create(&file_path).unwrap();
        file.write_all(toml_content.as_bytes()).unwrap();

        // Try reading the routine from the file
        let routine = Routine::read(&file_path.to_str().unwrap().to_string());

        // Assert that the read routine is valid
        assert!(routine.is_ok());
    }

    #[test]
    fn test_routine_read_failure() {
        // Trying to read a non-existent file
        let routine = Routine::read(&"non_existent_file.toml".to_string());

        // Assert that the routine read failed
        assert!(routine.is_err());
    }
}
