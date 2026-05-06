use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::constants;
use crate::models::backup::Backup;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum MirrorType {
    Full,
    Partial { n: usize },
}

/// Where the mirror's backup files are physically stored.
/// Only `Local` is implemented; other variants are reserved for future remote backends.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum StorageBackend {
    Local { path: String },
}

impl StorageBackend {
    /// Returns the local path for `Local` backends; errors on remote variants.
    pub fn local_path(&self) -> Result<&str, String> {
        match self {
            StorageBackend::Local { path } => Ok(path.as_str()),
        }
    }
}

/// Lightweight descriptor stored inside a `Repository`'s metadata.
/// One entry per registered mirror.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MirrorConfig {
    backend: StorageBackend,
    mirror_type: MirrorType,
}

impl MirrorConfig {
    pub fn new(backend: StorageBackend, mirror_type: MirrorType) -> Self {
        MirrorConfig {
            backend,
            mirror_type,
        }
    }

    pub fn backend(&self) -> &StorageBackend {
        &self.backend
    }

    pub fn mirror_type(&self) -> &MirrorType {
        &self.mirror_type
    }
}

/// Full mirror state, persisted in `{location}/mirror_metadata.toml`.
#[derive(Serialize, Deserialize, Debug)]
pub struct Mirror {
    id: String,
    name: String,
    created_at: u64,
    location: String,
    source_repository: String,
    mirror_type: MirrorType,
    backups: Vec<Backup>,
    /// For `Partial { n }` mirrors: counts down from `n-1` to `0`.
    /// When `0`, the next backup is kept and the counter resets to `n-1`.
    /// Always `0` for `Full` mirrors (unused).
    backups_until_next_keep: usize,
}

impl Mirror {
    pub fn new(
        name: String,
        location: String,
        source_repository: String,
        mirror_type: MirrorType,
    ) -> Self {
        Mirror {
            id: Uuid::now_v7().to_string(),
            name,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            location,
            source_repository,
            mirror_type,
            backups: vec![],
            backups_until_next_keep: 0,
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

    pub fn source_repository(&self) -> &String {
        &self.source_repository
    }

    pub fn mirror_type(&self) -> &MirrorType {
        &self.mirror_type
    }

    pub fn backups(&self) -> &Vec<Backup> {
        &self.backups
    }

    pub fn backups_mut(&mut self) -> &mut Vec<Backup> {
        &mut self.backups
    }

    pub fn backups_until_next_keep(&self) -> usize {
        self.backups_until_next_keep
    }

    pub fn set_backups_until_next_keep(&mut self, value: usize) {
        self.backups_until_next_keep = value;
    }

    pub fn add_backup(&mut self, backup: Backup) {
        self.backups.push(backup);
    }

    /// Removes the backup with the given id from the in-memory list.
    /// Does NOT delete files — file deletion is handled by the service layer.
    pub fn remove_backup_by_id(&mut self, backup_id: &str) {
        self.backups.retain(|b| b.id() != backup_id);
    }
}

pub fn save_mirror(mirror: &Mirror) -> Result<(), String> {
    let metadata_path = format!("{}/{}", mirror.location(), constants::MIRROR_METADATA_FILE);

    fs::write(
        &metadata_path,
        toml::to_string(mirror).map_err(|e| format!("Failed to serialize mirror: {}", e))?,
    )
    .map_err(|e| format!("Failed to save mirror metadata: {}", e))
}

pub fn load_mirror(location: &str) -> Option<Mirror> {
    let metadata_path = format!("{}/{}", location, constants::MIRROR_METADATA_FILE);

    match fs::read_to_string(&metadata_path) {
        Ok(content) => toml::from_str(&content).ok(),
        Err(_) => None,
    }
}
