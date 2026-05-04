use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = "backup-docker-files",
    version = "0.1",
    author = "Lyubolp",
    about = "A tool to backup Docker container files"
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new backup
    Create {
        /// Name of the repository to store the backup
        #[clap(short, long)]
        repository: String,
    },
    /// List all backups in a repository
    List {
        /// Name of the repository to list backups from
        #[clap(short, long)]
        repository: String,
    },
    /// Restore a backup from a repository
    Restore {
        /// Name of the repository to restore from
        #[clap(short, long)]
        repository: String,
        /// ID of the backup to restore
        #[clap(short, long)]
        backup: String,
    },
    /// Manage repositories
    Repo {
        #[clap(subcommand)]
        command: RepoCommands,
    },
    /// Manage mirrors
    Mirror {
        #[clap(subcommand)]
        command: MirrorCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum RepoCommands {
    /// Create a new repository
    Create {
        /// Path where the repository will be stored
        #[clap(short, long)]
        path: String,
    },
    /// Show information about a repository
    Info {
        /// Path to the repository
        #[clap(short, long)]
        path: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum MirrorCommands {
    /// Create a mirror of a repository
    Create {
        /// Name of the repository to mirror
        #[clap(short, long)]
        repository: String,
        /// Path where the mirror will be stored
        #[clap(short, long)]
        path: String,
        /// Number of backups to retain in the mirror
        #[clap(long)]
        retention: Option<usize>,
    },
}
