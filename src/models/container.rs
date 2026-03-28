use crate::models::labels::Labels;

#[derive(Debug, Clone)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub labels: Labels,
}

impl Container {
    pub fn new(id: String, name: String, labels: Labels) -> Self {
        Self {
            id,
            name,
            labels,
        }
    }
}
