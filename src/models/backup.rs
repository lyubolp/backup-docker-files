use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
pub struct Backup {
    id: String,
    created_at: u64,
    contents: Vec<String>,
    size: u64,
    root: String,
    is_completed: bool,
}

impl Backup {
    pub fn new(root: String) -> Self {
        Backup {
            id: Uuid::now_v7().to_string(),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            contents: vec![],
            size: 0,
            root,
            is_completed: false,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    pub fn contents(&self) -> &Vec<String> {
        &self.contents
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn root(&self) -> &String {
        &self.root
    }

    pub fn is_completed(&self) -> bool {
        self.is_completed
    }

    pub fn complete(&mut self, contents: Vec<String>, size: u64) {
        self.contents = contents;
        self.size = size;
        self.is_completed = true;
    }
}
