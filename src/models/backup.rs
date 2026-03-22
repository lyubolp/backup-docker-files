use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Backup {
    id: String,
    created_at: u64,
    contents: Vec<String>,
    size: u64,
    repository: u32,
}

impl Backup {
    pub fn new(
        id: String,
        created_at: u64,
        contents: Vec<String>,
        size: u64,
        repository: u32,
    ) -> Self {
        Backup {
            id,
            created_at,
            contents,
            size,
            repository,
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

    pub fn repository(&self) -> u32 {
        self.repository
    }
}
