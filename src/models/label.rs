use serde::de;

use crate::models::extraction::ExtractionType;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Label {
    pub enabled: bool,
    pub extraction_type: ExtractionType,
    pub online: bool,
}

impl Label {
    pub fn new(enabled: bool, extraction_type: ExtractionType, online: bool) -> Self {
        Self {
            enabled,
            extraction_type,
            online,
        }
    }

    pub fn from_labels(labels: &HashMap<String, String>) -> Self {
        let enabled = labels.get("bdf.enable").map_or(false, |v| v == "true");
        let extraction_type = match labels.get("bdf.extraction_type").map(|s| s.as_str()) {
            Some("File") => ExtractionType::File,
            Some("TinyDB") => ExtractionType::TinyDB,
            Some("SQLite") => ExtractionType::SQLite,
            Some("Pocketbase") => ExtractionType::Pocketbase,
            _ => ExtractionType::File, // Default to File if not specified
        };
        let online = labels.get("bdf.online").map_or(false, |v| v == "true");

        Self {
            enabled,
            extraction_type,
            online,
        }
    }
}

pub fn label_keys() -> HashMap<String, String> {
    HashMap::from([
        ("enable".to_string(), "bdf.enable".to_string()),
        (
            "extraction_type".to_string(),
            "bdf.extraction_type".to_string(),
        ),
        ("online".to_string(), "bdf.online".to_string()),
    ])
}
