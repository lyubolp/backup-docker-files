mod constants;
mod models;
mod services;

use crate::{
    models::repo::{self, Repository, load_repository, save_repository},
    services::discovery::{collect_containers_to_backup, get_docker_client},
    services::extraction::sqlite::extract,
};

#[tokio::main]
async fn main() {
    let docker_client = get_docker_client().unwrap();
    let containers = collect_containers_to_backup(&docker_client).await;
    println!("Containers to backup: {:?}", containers);

    let repo = load_repository("temp_repo").unwrap();

    let extract_result = extract(&containers[0], &repo).await;

    println!("Extract result: {:?}", extract_result);
}
