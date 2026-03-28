use crate::models::container::Container;
use crate::models::repo::Repository;
use crate::services::discovery::get_docker_client;
use crate::services::extraction::file::copy_file;

pub async fn extract(container: &Container, repository: &Repository) -> Result<(), String> {
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

        for path in container.labels.file_paths.iter() {
            if let Err(e) =
                copy_file(container, path, &format!("/tmp/backup/{}", path), &docker).await
            {
                return Err(format!("Failed to copy file {}: {}", path, e));
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
        Ok(())
    } else {
        Err("Failed to connect to Docker".to_string())
    }
}
