pub mod file;
pub mod sqlite;
pub mod utils;

use crate::models::container::Container;
use crate::models::repo::Repository;

trait Extractor {
    fn extract(&self, container: &Container, repository: &Repository) -> Result<(), String>;
}
