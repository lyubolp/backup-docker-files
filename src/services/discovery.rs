use std::collections::HashMap;

use bollard::{Docker, query_parameters::ListContainersOptionsBuilder};

use crate::models::container::Container;
use crate::models::labels::{Labels, label_keys};

pub fn get_docker_client() -> Result<Docker, String> {
    Docker::connect_with_unix_defaults().map_err(|e| format!("Failed to connect to Docker: {}", e))
}

pub async fn collect_containers_to_backup(connection: &Docker) -> Vec<Container> {
    let filters = HashMap::from([("label", vec!["bdf.enable"])]);
    let options = ListContainersOptionsBuilder::default()
        .filters(&filters)
        .build();

    let containers = connection.list_containers(Some(options)).await;

    let label_keys = label_keys();
    if let Ok(containers) = containers {
        containers
            .into_iter()
            .filter_map(|container| {
                container.labels.and_then(|labels| {
                    if label_keys.iter().all(|(_, key)| labels.contains_key(key)) {
                        let label = Labels::from_labels(&labels);
                        Some(Container::new(
                            container.id.unwrap(),
                            container.names.unwrap()[0].clone(),
                            label.unwrap(),
                        ))
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
