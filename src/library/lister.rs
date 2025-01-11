use std::collections::HashMap;
use std::fs::{OpenOptions, read_dir};
use std::io::{Read, Write};
use std::path::{PathBuf, Path};
use crate::library::config::Configuration;
use crate::library::routine::Routine;
use serde_yaml;

pub(crate) struct Lister {
    pub(crate) file_path: PathBuf,
    pub(crate) paths: HashMap<String, PathBuf>,
}

pub(crate) const LISTER_FILE_NAME: &str = "lister.yaml";  // Name of the file used to store the paths

impl Lister {
    /// Creates a new `Lister` instance by reading the configuration directory and initializing the paths
    /// If the lister file is empty, it will initialize the paths by scanning the directory.
    pub(crate) fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Retrieve the configuration directory
        let config_dir = Configuration::get_dir()
            .expect("Could not get config directory");

        // Construct the file path to the lister YAML file
        let file_path = config_dir.join(LISTER_FILE_NAME);

        // Open the file for reading and writing, creating it if it doesn't exist
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&file_path)?;

        // Read the contents of the file into a buffer
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // If the file is empty, initialize paths from the configuration directory
        let paths = if buffer.is_empty() {
            Self::initialize_paths(&config_dir)?
        } else {
            // Otherwise, parse the YAML data from the file
            Self::parse_lister_file(&buffer)?
        };

        Ok(Self { file_path, paths })
    }

    /// Initializes the paths by scanning the configuration directory for valid `.toml` files
    /// It writes an initial comment to the file if it's being created.
    fn initialize_paths(config_dir: &Path) -> Result<HashMap<String, PathBuf>, Box<dyn std::error::Error>> {
        // Open the lister file for writing (will create it if it doesn't exist)
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(config_dir.join(LISTER_FILE_NAME))?;

        // Write a comment indicating the start of the file
        file.write_all(r"# Lister file start".as_bytes())?;

        // Scan the configuration directory for `.toml` files excluding the config file itself
        let paths = read_dir(config_dir)?
            .filter_map(|entry| entry.ok())  // Filter out any invalid entries
            .filter(|entry| {
                let path = entry.path();
                // Only include files that are not directories, aren't the config file, and have a `.toml` extension
                path.is_file() &&
                    path.file_name().map(|name| name != super::config::CONFIG_FILE_NAME && name.to_string_lossy().ends_with(".toml")).unwrap_or(false)
            })
            .map(|entry| {
                let path = entry.path();
                // Read the routine data from each valid `.toml` file
                let routine_data = Routine::read(&path).ok()?;
                // Return the routine title and canonicalized path
                Some((routine_data.base.title, dunce::canonicalize(path).ok()?))
            })
            .filter_map(|opt| opt)  // Filter out any `None` values from the above map operation
            .collect::<HashMap<_, _>>();  // Collect the results into a HashMap

        Ok(paths)
    }

    /// Parses the given buffer (which should be YAML data) into a HashMap of file paths
    pub(crate) fn parse_lister_file(buffer: &[u8]) -> Result<HashMap<String, PathBuf>, Box<dyn std::error::Error>> {
        // Convert the buffer to a string and parse the YAML content into a HashMap
        let data = String::from_utf8_lossy(buffer);
        let mut paths: HashMap<String, PathBuf> = serde_yaml::from_str(&data)?;

        // Check for updates
        for path in read_dir(Configuration::get_dir().expect("Could not get config directory")).expect("Could not read config directory") {
            let path = path?.path();
            if path.is_dir() {
                continue;
            }
            let file_name = String::from(path.file_name().expect("Could not get file name").to_string_lossy());
            if !file_name.ends_with(".toml") {
                continue;
            }
            let routine = Routine::read(&path).expect(&format!("Could not read {}", file_name));
            paths.insert(routine.base.title, dunce::canonicalize(&path).expect("Could not canonicalize path"));
        }

        Ok(paths)
    }

    /// Dumps the current paths into the lister YAML file
    pub(crate) fn dump(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Convert the paths HashMap to a YAML string
        let output = serde_yaml::to_string(&self.paths)?;

        // Open the file for writing and save the YAML string
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.file_path)
            .expect("Could not open file");
        file.write_all(output.as_bytes())
            .expect("Could not write to file");

        Ok(())
    }

    /// Adds a new file path to the lister, using either a provided name or the title from the routine
    pub(crate) fn add_file(&mut self, path_buf: &PathBuf, name: Option<String>) {
        // If a name is provided, use it as the key; otherwise, use the title from the routine file
        let title = name.unwrap_or_else(|| {
            Routine::read(path_buf)
                .map(|routine| routine.base.title)
                .unwrap_or_else(|_| String::from("Routine not found"))  // Default title if the routine is not found
        });

        // Insert the file path into the HashMap under the determined title
        self.paths.insert(title, path_buf.clone());
    }

    /// Returns a reference to the stored paths HashMap
    pub(crate) fn get_data(&self) -> &HashMap<String, PathBuf> {
        &self.paths
    }
}
