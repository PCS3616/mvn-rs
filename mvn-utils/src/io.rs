use std::fs;
use std::path::{Path, PathBuf};

pub fn file_exists(path: &str) -> Result<PathBuf, &'static str> {
    let path = Path::new(path);
    if let Ok(exists) = path.try_exists() {
        if exists {
            return Ok(path.to_path_buf());
        }
    }
    Err("input file does not exist")
}

pub fn read_to_string(path: &PathBuf) -> String {
    fs::read_to_string(path)
        .expect("failed to read file")
        .to_uppercase()
}
