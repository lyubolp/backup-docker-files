use fs_extra::dir::{CopyOptions, copy};

use crate::models::backup::Backup;
use crate::models::mirror::{Mirror, MirrorConfig, MirrorType, StorageBackend, save_mirror};

/// Returns true if the next backup should be stored in this mirror.
pub fn should_keep_backup(mirror: &Mirror) -> bool {
    match mirror.mirror_type() {
        MirrorType::Full => true,
        MirrorType::Partial { .. } => mirror.backups_until_next_keep() == 0,
    }
}

/// Advances the partial-mirror counter after a keep/skip decision.
/// For `Full` mirrors this is a no-op.
pub fn advance_counter(mirror: &mut Mirror) {
    match mirror.mirror_type().clone() {
        MirrorType::Full => {}
        MirrorType::Partial { n } => {
            if mirror.backups_until_next_keep() == 0 {
                // Just kept one — reset to n-1 (skip the next n-1 backups)
                mirror.set_backups_until_next_keep(n - 1);
            } else {
                mirror.set_backups_until_next_keep(mirror.backups_until_next_keep() - 1);
            }
        }
    }
}

/// Resolves the destination directory path for a given backup within a mirror.
/// Only `StorageBackend::Local` is implemented; other variants return an error.
pub fn resolve_destination(config: &MirrorConfig, backup_id: &str) -> Result<String, String> {
    match config.backend() {
        StorageBackend::Local { path } => Ok(format!("{}/{}", path, backup_id)),
    }
}

/// Copies all files from `backup.root()` into `dest` using a full recursive copy.
/// Isolated so that a future remote backend only needs to replace this function.
pub fn copy_backup(backup: &Backup, dest: &str) -> Result<(), String> {
    std::fs::create_dir_all(dest)
        .map_err(|e| format!("Failed to create mirror backup dir '{}': {}", dest, e))?;

    let options = CopyOptions {
        content_only: true,
        ..Default::default()
    };

    copy(backup.root(), dest, &options)
        .map(|_| ())
        .map_err(|e| format!("Failed to copy backup to mirror '{}': {}", dest, e))
}

/// Deletes a backup's files from the mirror storage backend.
/// For `Local`: removes the `{path}/{backup_id}/` directory.
pub fn remove_from_mirror(backup_id: &str, config: &MirrorConfig) -> Result<(), String> {
    match config.backend() {
        StorageBackend::Local { path } => {
            let dir = format!("{}/{}", path, backup_id);
            if std::path::Path::new(&dir).exists() {
                std::fs::remove_dir_all(&dir)
                    .map_err(|e| format!("Failed to remove backup '{}' from mirror: {}", dir, e))?;
            }
            Ok(())
        }
    }
}

/// Distributes a completed backup to a mirror.
/// Checks the keep/skip policy, advances the counter, and — if keeping —
/// copies the files and records a mirrored `Backup` entry in the mirror's metadata.
pub fn distribute_backup(
    backup: &Backup,
    mirror: &mut Mirror,
    config: &MirrorConfig,
) -> Result<(), String> {
    let keep = should_keep_backup(mirror);
    advance_counter(mirror);

    if keep {
        let dest = resolve_destination(config, backup.id())?;
        copy_backup(backup, &dest)?;
        let mirrored_backup = backup.mirror_to(dest);
        mirror.add_backup(mirrored_backup);
        save_mirror(mirror)?;
    }

    Ok(())
}
