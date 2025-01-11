use crate::library::routine::Routine;
use std::path::{Path, PathBuf};

pub(crate) fn info(routine_path: &PathBuf) {
    if !routine_path.exists() {
        println!("Routine not found");
        return;
    }

    Routine::read(&routine_path)
        .unwrap_or_else(|err| panic!("Error reading routine: {}", err))
        .print();
}
