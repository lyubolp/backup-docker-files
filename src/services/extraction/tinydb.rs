use crate::models::container::Container;

use super::Extractor;

pub(crate) struct TinyDbExtractor;

impl Extractor for TinyDbExtractor {
    async fn extract(container: &Container) -> Result<String, String> {
        unimplemented!()
    }
}
