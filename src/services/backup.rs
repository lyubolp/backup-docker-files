use crate::models::backup::{self, Backup};
use std::fs;

pub fn init_backup(backup: &Backup) {
    let root_dir = backup.root();

    fs::create_dir_all(root_dir).expect("Failed to create backup root directory");
}
