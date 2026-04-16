use crate::models::container::Container;

pub mod file;
pub mod pocketbase;
pub mod sqlite;
pub mod utils;

use crate::models::extraction::ExtractionType;
use crate::services::extraction::file::FileExtractor;
use crate::services::extraction::pocketbase::PocketbaseExtractor;
use crate::services::extraction::sqlite::SqliteExtractor;

pub(crate) trait Extractor {
    async fn extract(container: &Container) -> Result<String, String>;
}

pub async fn extract(container: &Container) -> Result<String, String> {
    match container.labels.extraction_type {
        ExtractionType::File => FileExtractor::extract(container).await,
        ExtractionType::Pocketbase => PocketbaseExtractor::extract(container).await,
        ExtractionType::SQLite => SqliteExtractor::extract(container).await,
    }
}
