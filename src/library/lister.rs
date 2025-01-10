use std::collections::HashMap;
use std::path::PathBuf;
use rusqlite::{params, Connection};
use crate::library::config::Configuration;

pub(crate) struct Lister {
    connection: Connection,
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
                      path TEXT NOT NULL UNIQUE,
                   )"#,
            [],
        ).expect("Could not create library db.");

        Self {
            connection: conn,
        }
    }

    pub(crate) fn stop(self) {
        self.connection.close().expect("Error while closing database.");
    }

    pub(crate) fn update(&self) {
        let conn = &self.connection;

        let mut statement =
            self.connection
                .prepare("SELECT key, path FROM paths")
                .expect("Error preparing statement query to local database.");

        let rows = statement
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    PathBuf::from(row.get::<_, String>(1)?)
                ))
            })
            .expect("Error while querying local database.");

        for row in rows {
            let (key, path) = row.expect("Error processing query to local database.");
            if !path.exists() {
                self.connection.execute(
                    "DELETE FROM paths WHERE key = ?1",
                    params![key])
                    .expect(format!("Error while deleting path: {}", key).as_str());
            }
        }
    }

    /// Write to the routine library for ACT-IV
    pub(crate) fn dump(&self) {
        let config_dir = match Configuration::get_dir() {
            Ok(path) => { path },
            Err(error) => { panic!("Error trying to get config dir: {}", error); }
        };
        let connection = Connection::open(config_dir.join(LISTER_FILE_NAME)).unwrap();
        todo!("End implementation")
    }

    /// Read from the routine library for ACT-IV
    pub(crate) fn read() {

    }
}
