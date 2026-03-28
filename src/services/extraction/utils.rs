use crate::constants;

pub fn create_staging_dir() -> Result<(), String> {
    std::fs::create_dir_all(constants::STAGING_DIR).map_err(|e| e.to_string())
}
