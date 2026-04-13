use crate::models::container::Container;
use crate::services::extraction::file::FileExtractor;

use super::Extractor;

pub(crate) struct SqliteExtractor;

impl Extractor for SqliteExtractor {
    async fn extract(container: &Container) -> Result<String, String> {
        // When extracting SQLite databases, we need to copy any "-whl" and "-shm".
        // This should not be specified in the labels, so we will just add a glob pattern to the label.

        let mut extended_container = container.clone();
        extended_container.labels.file_paths[0] =
            format!("{}*", extended_container.labels.file_paths[0]);

        FileExtractor::extract(container).await
    }
}
