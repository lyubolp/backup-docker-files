use std::fs;
use std::path::Path;

use crate::constants;

pub fn create_staging_dir() -> Result<(), String> {
    std::fs::create_dir_all(constants::STAGING_DIR).map_err(|e| e.to_string())
}

pub fn walk_dir(root: &String) -> Vec<String> {
    let path = Path::new(root);
    let mut result = Vec::new();
    let mut stack = vec![path.to_path_buf()];

    while let Some(current) = stack.pop() {
        for entry in fs::read_dir(&current).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            if entry_path.is_dir() {
                stack.push(entry_path);
            } else {
                result.push(entry_path.display().to_string());
            }
        }
    }

    result
}
