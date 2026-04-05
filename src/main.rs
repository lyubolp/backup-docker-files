mod constants;
mod models;
mod services;

use crate::{
    models::repo::{self, Repository, load_repository, save_repository},
    services::discovery::{collect_containers_to_backup, get_docker_client},
    services::extraction::sqlite::extract,
};

use std::fs;

#[tokio::main]
async fn main() {
    let docker_client = get_docker_client().unwrap();
    let containers = collect_containers_to_backup(&docker_client).await;
    println!("Containers to backup: {:?}", containers);

    for entry in fs::read_dir(".").unwrap() {
        let entry = entry.unwrap();
        println!("Found file: {}", entry.path().display());
    }
}
