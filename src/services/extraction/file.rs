use bollard::Docker;
use bollard::query_parameters::DownloadFromContainerOptionsBuilder;
use futures_util::StreamExt;
use glob::Pattern;
use std::io::Cursor;
use std::path;
use tar::Archive;

use super::utils::{create_staging_dir, get_base_from_glob, is_path_glob};
use crate::constants;
use crate::models::container::Container;
use crate::services::discovery::get_docker_client;

pub async fn extract(container: &Container) -> Result<String, String> {
    if let Ok(docker) = get_docker_client() {
        if container.labels.online {
            let pause_result = docker.pause_container(&container.name).await;

            if pause_result.is_err() {
                return Err(format!(
                    "Failed to pause container {}: {}",
                    container.name,
                    pause_result.err().unwrap()
                ));
            }
        }

        let staging_result = create_staging_dir();

        if staging_result.is_err() {
            return Err(format!(
                "Failed to create staging directory: {}",
                staging_result.err().unwrap()
            ));
        }

        let staging_path = format!("{}/{}", constants::STAGING_DIR, container.name);

        let mut errors: Vec<String> = vec![];
        for path in container.labels.file_paths.iter() {
            if is_path_glob(path) {
                let glob_root = get_base_from_glob(path);
                let matches =
                    expand_glob_in_container(path, &glob_root, container, &docker).await?;

                for matched_path in matches {
                    if let Err(e) =
                        copy_file(container, &matched_path, &staging_path, &docker).await
                    {
                        errors.push(format!("Failed to copy file {}: {}", matched_path, e));
                    }
                }
            } else {
                if let Err(e) = copy_file(container, path, &staging_path, &docker).await {
                    errors.push(format!("Failed to copy file {}: {}", path, e));
                }
            }
        }
        if container.labels.online {
            let unpause_result = docker.unpause_container(&container.name).await;

            if unpause_result.is_err() {
                return Err(format!(
                    "Failed to unpause container {}: {}",
                    container.name,
                    unpause_result.err().unwrap()
                ));
            }
        }

        if errors.len() == 0 {
            Ok(staging_path)
        } else {
            Err(errors.join("; "))
        }
    } else {
        Err("Failed to connect to Docker".to_string())
    }
}

pub async fn copy_file(
    container: &Container,
    source_path: &str,
    target_path: &str,
    docker: &Docker,
) -> Result<(), String> {
    let mut archive = prepare_tar_with_contents(container, source_path, docker).await?;

    archive.unpack(target_path).map_err(|e| e.to_string())?;

    Ok(())
}

async fn prepare_tar_with_contents(
    container: &Container,
    source_path: &str,
    docker: &Docker,
) -> Result<Archive<Cursor<Vec<u8>>>, String> {
    let options = DownloadFromContainerOptionsBuilder::new()
        .path(source_path)
        .build();

    let mut stream = docker.download_from_container(&container.name, Some(options));

    let mut buffer: Vec<u8> = Vec::new();
    while let Some(chunk) = stream.next().await {
        let bytes = chunk.map_err(|e| e.to_string())?;
        buffer.extend_from_slice(&bytes);
    }

    let cursor = Cursor::new(buffer);
    Ok(tar::Archive::new(cursor))
}

async fn expand_glob_in_container(
    pattern: &str,
    source_path: &str,
    container: &Container,
    docker: &Docker,
) -> Result<Vec<String>, String> {
    let mut archive = prepare_tar_with_contents(container, source_path, docker).await?;

    let paths: Vec<String> = archive
        .entries()
        .unwrap()
        .map(|e| e.unwrap().path().unwrap().display().to_string())
        .collect();

    let reanchored_paths = reanchor_paths(&paths, source_path);

    let glob_pattern = Pattern::new(pattern).map_err(|e| e.to_string())?;

    Ok(reanchored_paths
        .into_iter()
        .filter(|path| glob_pattern.matches(path))
        .collect::<Vec<_>>())
}

fn reanchor_paths(paths: &Vec<String>, source_path: &str) -> Vec<String> {
    /*
    Paths received from the Docker API are anchored at the base of the path.
    For example: ["dir/file1.txt", "dir/file2.txt"], instead of "/data/dir/file1.txt", "/data/dir/file2.txt".
    Source path in this case is "/data/dir", and we will have a glob pattern like "/data/dir/\*.txt" (Ignore the escape)
    */

    // Get the last part of the source path, which is the anchor for the paths received from Docker (e.g. "dir" in the example above)
    let last_dir = path::Path::new(source_path)
        .components()
        .last()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap();

    // Replace the anchor with the full source path to get the correct paths (e.g. replace "dir/file1.txt" with "/data/dir/file1.txt")
    paths
        .into_iter()
        .map(|p| p.replacen(last_dir, source_path, 1))
        .collect()
}
