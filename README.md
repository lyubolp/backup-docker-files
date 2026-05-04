# backup-docker-files
I need to backup files from Docker

## CLI reference

### Main commands

- Create a backup: `bdf create --repository <REPO_PATH>`
- List all backups: `bdf list --repository <REPO_PATH>`
- Restore a backup: `bdf restore --repository <REPO_PATH> --backup <BACKUP_ID>`

### Repo managment

- Create a repository: `bdf repo create --path <REPO_PATH>`
- Create a mirror: `bdf mirror create --repository <REPO_PATH> --path <MIRROR_PATH>`
- Create a partial mirror: `bdf mirror create --repository <REPO_PATH> --path <MIRROR_PATH> --retention <RETENTION>`
- List information about a repository: `bdf repo info --path <REPO_PATH>`
- Remove a repository: `bdf repo remove --repository <REPO_PATH>`
- Change rotation limit: `bdf repo edit --repository <REPO_PATH> --rotation <ROTATION_LIMIT>`
- Move a repository: `bdf repo edit --repository <REPO_PATH> --location <NEW_LOCATION>`
- Remove mirror: `bdf mirror remove --path <MIRROR_PATH>`

### Backup managment

- Remove a backup: `bdf backup remove --repository <REPO_PATH> --backup <BACKUP_ID>`
- List information about a backup: `bdf backup info --repository <REPO_PATH> --backup <BACKUP_ID>`

### Credentials managment

- Add credentials
- Edit credentials
- Remove credentials