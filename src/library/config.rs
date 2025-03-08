use std::{path::PathBuf, env, fs, io, error};
use std::collections::HashMap;
use std::fs::OpenOptions;

pub(crate) const CONFIG_FILE_NAME: &str = "config.toml";

pub(crate) struct Configuration;

impl Configuration {
    /// Find the config directory
    pub(crate) fn get_dir() -> Result<PathBuf, Box<dyn error::Error>> {
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