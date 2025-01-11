use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;
use crate::library::config::Configuration;
use crate::library::routine::Routine;

pub(crate) struct Lister {
    file_path: PathBuf,
    paths: HashMap<String, PathBuf>,
}

pub(crate) const LISTER_FILE_NAME: &str = "lister.yaml";

impl Lister {
    pub(crate) fn new() -> Self {
        let config_dir = Configuration::get_dir()
            .expect("Error getting config directory");

        let file_path = config_dir.join(LISTER_FILE_NAME);

        let mut file = OpenOptions::new().read(true).write(true).create(true).open(&file_path).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        let length = file.read_to_end(&mut buffer).unwrap();

        // Hard coded, needs refactorizing
        let paths: HashMap<String, PathBuf> = if !length > 0 {
            file.write_all(r"# Lister file start".as_bytes()).unwrap();

            config_dir
                .read_dir()
                .expect("Error reading config directory")
                .map(|entry| entry.expect("Error reading config dir entries").path())
                .filter(|path| {
                    !path.is_dir()
                        && path.file_name().unwrap_or("".as_ref()) != super::config::CONFIG_FILE_NAME
                        && String::from(path.file_name().unwrap().to_str().unwrap()).ends_with(".toml")
                })
                .map(|path| {
                    let routine_data = Routine::read(&path)
                        .expect("Error reading routine files");
                    (routine_data.base.title, path.canonicalize().expect("Error canonicalizing routine files"))
                }).collect::<HashMap<_, _>>()
        }
        else { serde_yaml::from_str(&*String::from_utf8(buffer).expect("Error converting routine file to UTF-8")).expect("Error parsing Lister") };

        Self {
            file_path,
            paths
        }
    }

    /// Write to the routine library for ACT-IV
    pub(crate) fn dump(&mut self) {
        let output = serde_yaml::to_string(&self.paths).expect("Error dumping Lister");
        let mut file = OpenOptions::new().write(true).open(&self.file_path).expect("Error opening Lister");
        file.write_all(output.as_bytes()).expect("Error writing Lister");
    }

    pub(crate) fn add_file(&mut self, path_buf: &PathBuf, name: &Option<String>) {
        if let Some(name) = name {
            self.paths.insert(name.clone(), path_buf.clone());
            return;
        }

        let routine = Routine::read(&path_buf).expect("Error reading routine files");
        self.paths.insert(routine.base.title.clone(), path_buf.clone());
    }

    /// Get file's data
    pub(crate) fn get_data(&self) -> &HashMap<String, PathBuf> { &self.paths }
}
