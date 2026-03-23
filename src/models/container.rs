use crate::models::label::Label;

#[derive(Debug, Clone)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub label: Label,
}

impl Container {
    pub fn new(id: String, name: String, label: Label) -> Self {
        Self { id, name, label }
    }
}
