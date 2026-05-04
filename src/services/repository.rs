use crate::{
    models::{backup::Backup, repository::Repository},
    services::backup::remove_backup,
};

pub fn new_backup(repo: &mut Repository) -> Result<&mut Backup, String> {
    if repo.backups_mut().len() >= repo.rotation_limit() {
        rotate_backups(repo)?;
    }

    let backup = Backup::new(repo.location().to_string());
    repo.add_backup(backup);

    repo.backups_mut()
        .last_mut()
        .ok_or_else(|| "Failed to create backup".to_string())
}

fn rotate_backups(repo: &mut Repository) -> Result<(), String> {
    repo.backups_mut().sort_by_key(|b| b.created_at());

    while repo.backups_mut().len() >= repo.rotation_limit() {
        if let Some(oldest_backup) = repo.backups_mut().first() {
            remove_backup(oldest_backup)?;
            repo.backups_mut().remove(0);
        }
    }

    Ok(())
}
