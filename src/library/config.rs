use std::{path::PathBuf, env, fs, io, error};
use std::collections::HashMap;
use std::fs::OpenOptions;
use rusqlite::{params, Connection};

const CONFIG_FILE_NAME: &str = "config.toml";
const LIBRARY_FILE_NAME: &str = "library.db";

pub(crate) struct Configuration;
pub(crate) struct Library {
    files: HashMap<String, PathBuf>,
}

impl Configuration {
    /// Find the config directory
    fn get_dir() -> Result<PathBuf, Box<dyn error::Error>> {
        let home_dir = home::home_dir().ok_or("Could not find home directory.")?;

        let config_dir =
            home_dir
                .join(".config")
                .join("ACT-IV");

        if let Err(error) = fs::create_dir_all(&config_dir) {
            if error.kind() != io::ErrorKind::AlreadyExists {
                return Err(Box::new(error));
            }
        }

        Ok(config_dir)
    }

    /// Write to the global config file for ACT-IV
    pub(crate) fn write() -> io::Result<()> {
        let config_file_path =
            Self::get_dir()
                .expect("Error getting config directory.")
                .join(CONFIG_FILE_NAME);

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(config_file_path)
            .expect("Could not open config file.");

        // write!(file, "Hello, world!")?;

        Ok(())
    }
    /// Read from the global config file for ACT-IV
    pub(crate) fn read() -> Result<HashMap<String, String>, Box<dyn error::Error>> {
        let config_file_path = Self::get_dir()?.join(CONFIG_FILE_NAME);
        let content = fs::read_to_string(config_file_path)?;

        let config: HashMap<String, String> = toml::from_str(&content)?;
        Ok(config)
    }
}
impl Library {

    pub(crate) fn new() -> Self {
        let config_dir = Configuration::get_dir().unwrap();
        let conn = Connection::open(config_dir.join(LIBRARY_FILE_NAME)).unwrap();

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
        let connection = Connection::open(config_dir.join(LIBRARY_FILE_NAME)).unwrap();
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