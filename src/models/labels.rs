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
            Some("file") => ExtractionType::File,
            Some("tinydb") => ExtractionType::TinyDB,
            Some("sqlite") => ExtractionType::SQLite,
            Some("pocketbase") => ExtractionType::Pocketbase,
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
                .filter(|(k, _)| k.starts_with("bdf.") && !label_keys.values().any(|v| v == *k))
                .filter(|(k, _)| !k.starts_with("bdf.file_path"))
                .map(|(k, v)| {
                    (
                        k.clone().strip_prefix("bdf.").unwrap().to_string(),
                        v.clone(),
                    )
                })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_file_path() {
        let expected_file_path = "/data/file1.txt".to_string();
        let mut labels_map = HashMap::new();
        labels_map.insert("bdf.enable".to_string(), "true".to_string());
        labels_map.insert("bdf.extraction_type".to_string(), "File".to_string());
        labels_map.insert("bdf.online".to_string(), "false".to_string());
        labels_map.insert("bdf.file_path".to_string(), expected_file_path.clone());

        let labels = Labels::from_labels(&labels_map).unwrap();

        assert_eq!(labels.file_paths, vec![expected_file_path]);
    }

    #[test]
    fn test_multiple_file_paths() {
        let expected_file_paths =
            vec!["/data/file1.txt".to_string(), "/data/file2.txt".to_string()];
        let mut labels_map = HashMap::new();
        labels_map.insert("bdf.enable".to_string(), "true".to_string());
        labels_map.insert("bdf.extraction_type".to_string(), "File".to_string());
        labels_map.insert("bdf.online".to_string(), "false".to_string());
        labels_map.insert(
            "bdf.file_path.0".to_string(),
            expected_file_paths[0].clone(),
        );
        labels_map.insert(
            "bdf.file_path.1".to_string(),
            expected_file_paths[1].clone(),
        );

        let labels = Labels::from_labels(&labels_map).unwrap();

        assert_eq!(labels.file_paths, expected_file_paths);
    }
}
