use std::collections::HashMap;

use bollard::{Docker, query_parameters::ListContainersOptionsBuilder};

pub fn get_docker_client() -> Result<Docker, String> {
    Docker::connect_with_unix_defaults().map_err(|e| format!("Failed to connect to Docker: {}", e))
}

pub async fn collect_containers_to_backup(connection: &Docker) -> Vec<String> {
    let filters = HashMap::from([("label", vec!["bdf.enable"])]);
    let options = ListContainersOptionsBuilder::default()
        .filters(&filters)
        .build();

    let containers = connection.list_containers(Some(options)).await;

    if let Ok(containers) = containers {
        containers
            .into_iter()
            .filter_map(|container| {
                container.labels.and_then(|labels| {
                    if labels.get("bdf.enable").map_or(false, |v| v == "true") {
                        Some(container.names.unwrap()[0].clone().replace("/", ""))
                    } else {
                        None
                    }
                })
            })
            .collect()
    } else {
        vec![]
    }
}
