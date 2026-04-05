use crate::models::{
    backup::{self, Backup},
    container::Container,
    extraction::ExtractionType,
};
use crate::services::extraction;
use std::fs;

use fs_extra::dir::{CopyOptions, copy};

pub fn init_backup(backup: &Backup) -> Result<(), String> {
    let root_dir = backup.root();

    let root_dir_result = fs::create_dir_all(root_dir);

    if root_dir_result.is_err() {
        return Err(format!(
            "Failed to create backup root directory: {}",
            root_dir_result.err().unwrap()
        ));
    }

    Ok(())
}

pub async fn backup_container(backup: &Backup, container: &Container) -> Result<(), String> {
    let backup_status = match container.labels.extraction_type {
        ExtractionType::File => extraction::file::extract(container).await,
        ExtractionType::Pocketbase => unimplemented!("Pocketbase extraction not implemented"),
        ExtractionType::SQLite => extraction::sqlite::extract(container).await,
        ExtractionType::TinyDB => unimplemented!("TinyDB extraction not implemented"),
    };

    match backup_status {
        Ok(staging_dir) => {
            let target_dir = create_dir_for_container(backup.root(), &container.name);

            match target_dir {
                Ok(dir) => copy_from_staging(&staging_dir, &dir),
                Err(e) => Err(e),
            }
        }
        Err(error) => Err(error),
    }
}

pub fn complete_backup(backup: &mut Backup) {
    backup.complete();
}

fn copy_from_staging(staging_dir: &String, target_dir: &String) -> Result<(), String> {
    let options = CopyOptions::new();
    copy(staging_dir, target_dir, &options)
        .map_err(|e| format!("Failed to copy from staging to target: {}", e))
        .and(Ok(()))
}

fn create_dir_for_container(root_dir: &String, container_name: &String) -> Result<String, String> {
    let full_path = format!("{}/{}", root_dir, container_name);

    fs::create_dir_all(full_path.clone())
        .map_err(|_| format!("Failed to create container directory {}", container_name))
        .and(Ok(full_path))
}

fn gather_contents(backup: &Backup) -> Vec<(String, usize)> {
    // Implement logic to gather contents of the backup if needed

    for entry in fs::read_dir(backup.root()) {}
    vec![]
}
