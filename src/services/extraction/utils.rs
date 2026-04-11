use std::fs;
use std::path::Path;

use crate::constants;

pub fn create_staging_dir() -> Result<(), String> {
    std::fs::create_dir_all(constants::STAGING_DIR).map_err(|e| e.to_string())
}

pub fn remove_staging_dir() -> Result<(), String> {
    std::fs::remove_dir_all(constants::STAGING_DIR).map_err(|e| e.to_string())
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

pub fn is_path_glob(path: &str) -> bool {
    let glob_characters = ["*", "?", "[", "{"];

    glob_characters
        .iter()
        .any(|&glob_char| path.contains(glob_char))
}

pub fn get_base_from_glob(pattern: &str) -> String {
    pattern
        .split("/")
        .map(|piece| {
            if !is_path_glob(&piece) {
                Some(piece.to_string())
            } else {
                None
            }
        })
        .fuse()
        .flatten()
        .collect::<Vec<_>>()
        .join("/")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_base_from_glob_no_glob() {
        let pattern = "/foo/bar";
        let expected = "/foo/bar".to_string();

        let actual = get_base_from_glob(&pattern);

        assert_eq!(actual, expected);
    }
}
