use crate::models::container::Container;

pub mod file;
pub mod pocketbase;
pub mod sqlite;
pub mod utils;

pub(crate) trait Extractor {
    async fn extract(container: &Container) -> Result<String, String>;
}

pub async fn extract(container: &Container) -> Result<String, String> {
    unimplemented!()
}
// pub fn
