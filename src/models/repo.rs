use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    id: u32,
    name: String,
    created_at: u64,
    location: String,
    rotation_limit: u32,
}

impl Repository {
    pub fn new(
        id: u32,
        name: String,
        created_at: u64,
        location: String,
        rotation_limit: u32,
    ) -> Self {
        Repository {
            id,
            name,
            created_at,
            location,
            rotation_limit,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
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

    pub fn rotation_limit(&self) -> u32 {
        self.rotation_limit
    }
}
