mod constants;
mod models;
mod services;

use crate::{
    models::backup::Backup,
    models::repo::{self, Repository, load_repository, save_repository},
    services::backup::create_backup,
    services::discovery::{collect_containers_to_backup, get_docker_client},
};

#[tokio::main]
async fn main() {
    let docker_client = get_docker_client().unwrap();
    let containers = collect_containers_to_backup(&docker_client).await;
    println!("Containers to backup: {:?}", containers);

    let mut backup = Backup::new("temp_backup".to_string());

    let result = create_backup(&mut backup, &containers).await;

    println!("Backup result: {:?}", result);
}
