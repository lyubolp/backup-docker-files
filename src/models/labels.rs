use serde::de;

use crate::models::extraction::ExtractionType;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Labels {
    pub enabled: bool,
    pub extraction_type: ExtractionType,
    pub online: bool,
    pub file_paths: Vec<String>,
    pub other: Option<HashMap<String, String>>,
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

impl Labels {
    pub fn new(
        enabled: bool,
        extraction_type: ExtractionType,
        online: bool,
        file_paths: Vec<String>,
        other: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            enabled,
            extraction_type,
            online,
            file_paths,
            other,
        }
    }

    pub fn from_labels(labels: &HashMap<String, String>) -> Result<Self, String> {
        if !Self::are_required_keys_present(labels) {
            return Err("Missing required label keys".to_string());
        }

        let label_keys = label_keys();

        let enabled = labels
            .get(&label_keys["enable"])
            .map_or(false, |v| v == "true");

        let extraction_type = match labels
            .get(&label_keys["extraction_type"])
            .map(|s| s.as_str())
        {
            Some("File") => ExtractionType::File,
            Some("TinyDB") => ExtractionType::TinyDB,
            Some("SQLite") => ExtractionType::SQLite,
            Some("Pocketbase") => ExtractionType::Pocketbase,
            _ => ExtractionType::File, // Default to File if not specified
        };

        let online = labels
            .get(&label_keys["online"])
            .map_or(false, |v| v == "true");

        let file_paths = labels
            .iter()
            .filter(|(k, _)| k.starts_with("bdf.file_path"))
            .map(|(_, v)| v.clone())
            .collect();

        let other = Some(
            labels
                .iter()
                .filter(|(k, _)| !label_keys.values().any(|v| v == *k))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        );

        Ok(Self {
            enabled,
            extraction_type,
            online,
            file_paths,
            other,
        })
    }

    fn are_required_keys_present(labels: &HashMap<String, String>) -> bool {
        let required_keys = label_keys();
        required_keys.values().all(|key| labels.contains_key(key))
    }
}
