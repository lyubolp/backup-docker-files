/*
1. Get superuser token - POST /api/collections/_superusers/auth-with-password
2. Create backup - POST /api/backups
3. Get backups - GET /api/backups
4. Parse key from backups
5. Get file access token - POST /api/files/token
6. Download backup file - GET http://0.0.0.0:8080/api/backups/<backup_key>?token=<file_access_token>
*/

use crate::models::container::Container;

use super::Extractor;

pub(crate) struct PocketbaseExtractor;

impl Extractor for PocketbaseExtractor {
    async fn extract(container: &Container) -> Result<String, String> {
        unimplemented!()
    }
}
