mod models;
mod services;

use crate::services::discovery::{collect_containers_to_backup, get_docker_client};

#[tokio::main]
async fn main() {
    if let Ok(docker_client) = get_docker_client() {
        let containers = collect_containers_to_backup(&docker_client).await;
        println!("Containers to backup: {:?}", containers);
    } else {
        eprintln!("Failed to connect to Docker.");
    };
}
