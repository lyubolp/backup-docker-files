use serde::{Deserialize, Serialize};

use uuid::Uuid;

use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::constants;
use crate::models::backup::Backup;

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    id: String,
    name: String,
    created_at: u64,
    location: String,
    rotation_limit: usize,
    backups: Vec<Backup>,
}

impl Repository {
    pub fn new(name: String, location: String, rotation_limit: usize) -> Self {
        Repository {
            id: Uuid::now_v7().to_string(),
            name,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            location,
            rotation_limit,
            backups: vec![],
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    pub fn location(&self) -> &String {
        &self.location
    }

    pub fn rotation_limit(&self) -> usize {
        self.rotation_limit
    }

    pub fn new_backup(&self) -> Backup {
        if self.backups.len() >= self.rotation_limit {
            unimplemented!("Rotation logic not implemented yet");
        }

        unimplemented!("Backup creation logic not implemented yet");
    }

    fn rotate(&self) {
        unimplemented!("Rotation logic not implemented yet");
    }
}

pub fn save_repository(repo: &Repository) -> Result<(), String> {
    let metadata_path = format!("{}/{}", repo.location(), constants::REPO_METADATA_FILE);

    fs::write(
        &metadata_path,
        toml::to_string(repo).expect("Failed to serialize repository"),
    )
    .map_err(|e| format!("Failed to save repository: {}", e))
}

pub fn load_repository(location: &str) -> Option<Repository> {
    let metadata_path = format!("{}/{}", location, constants::REPO_METADATA_FILE);

    match fs::read_to_string(&metadata_path) {
        Ok(content) => toml::from_str(&content).ok(),
        Err(_) => None,
    }
}
