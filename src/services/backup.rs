use crate::models::{
    backup::{self, Backup},
    container::Container,
    extraction::ExtractionType,
};
use crate::services::extraction;
use crate::services::extraction::utils::{create_staging_dir, remove_staging_dir, walk_dir};
use std::fs;

use fs_extra::dir::{CopyOptions, copy};

pub async fn create_backup(backup: &mut Backup, containers: &Vec<Container>) -> Result<(), String> {
    init_backup(backup)?;

    create_staging_dir()?;
    for container in containers {
        if container.labels.enabled {
            backup_container(backup, container).await?;
        }
    }

    complete_backup(backup);

    remove_staging_dir()?;
    Ok(())
}

fn init_backup(backup: &Backup) -> Result<(), String> {
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

async fn backup_container(backup: &Backup, container: &Container) -> Result<(), String> {
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

fn complete_backup(backup: &mut Backup) {
    let contents = gather_contents(backup);
    let total_size: u64 = contents.iter().map(|path| get_size_of_content(path)).sum();

    backup.complete(contents, total_size);
}

fn copy_from_staging(staging_dir: &String, target_dir: &String) -> Result<(), String> {
    let options = CopyOptions::new().content_only(true);

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

fn gather_contents(backup: &Backup) -> Vec<String> {
    walk_dir(backup.root())
}

fn get_size_of_content(path: &String) -> u64 {
    fs::metadata(path).map_or_else(|_| 0, |metadata| metadata.len())
}
