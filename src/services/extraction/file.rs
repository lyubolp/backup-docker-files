use bollard::Docker;
use bollard::query_parameters::DownloadFromContainerOptionsBuilder;
use futures_util::StreamExt;
use std::io::Cursor;

use crate::models::container::Container;
use crate::models::repo::Repository;
use crate::services::discovery::get_docker_client;

pub async fn extract(container: &Container) -> Result<String, String> {
    if let Ok(docker) = get_docker_client() {
        unimplemented!("Not implemented")
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
    tar::Archive::new(cursor)
        .unpack(target_path)
        .map_err(|e| e.to_string())?;

    Ok(())
}
