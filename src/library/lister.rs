use std::collections::HashMap;
use std::path::PathBuf;
use rusqlite::{params, Connection};
use crate::library::config::Configuration;

pub(crate) struct Lister {
    files: HashMap<String, PathBuf>,
}

const LISTER_FILE_NAME: &str = "lister.db";

impl Lister {
    pub(crate) fn new() -> Self {
        let config_dir = Configuration::get_dir().unwrap();
        let conn = Connection::open(config_dir.join(LISTER_FILE_NAME)).unwrap();

        conn.execute(
            r#"CREATE TABLE IF NOT EXISTS paths (
                      id INTEGER PRIMARY KEY,
                      key TEXT NOT NULL UNIQUE,
                      path TEXT NOT NULL,
                   )"#,
            [],
        ).expect("Could not create library db.");

        Self { files: HashMap::new() }
    }

    pub(crate) fn stop(&self) {

    }

    /// Write to the routine library for ACT-IV
    pub(crate) fn dump(&mut self) {
        let config_dir = match Configuration::get_dir() {
            Ok(path) => { path },
            Err(error) => { panic!("Error trying to get config dir: {}", error); }
        };
        let connection = Connection::open(config_dir.join(LISTER_FILE_NAME)).unwrap();
        for (&key, path) in &self.files {
            if !path.exists() {
                self.files.remove_entry(&key);
                continue;
            }

            connection.execute(
                r#"INSERT INTO paths (key, path) VALUES (?1, ?2)"#,
                params![key, path.to_str().unwrap()],
            ).expect("Error during dumping to db.");
        }
    }


    /// Read from the routine library for ACT-IV
    pub(crate) fn read() {

    }
}
