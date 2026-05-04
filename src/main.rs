mod cli;
mod clients;
mod constants;
mod models;
mod services;

use clap::Parser;
use std::fs;

use crate::{
    cli::{Cli, Commands, MirrorCommands, RepoCommands},
    models::backup::Backup,
    models::repository::{Repository, load_repository, save_repository},
    services::backup::create_backup,
    services::discovery::{collect_containers_to_backup, get_docker_client},
    services::repository,
};

const DEFAULT_ROTATION_LIMIT: usize = 10;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { repository } => {
            if let Err(e) = cmd_create(repository).await {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }

        Commands::List { repository } => {
            let repo = match load_repository(&repository) {
                Some(r) => r,
                None => {
                    eprintln!("Repository not found at path: {}", repository);
                    std::process::exit(1);
                }
            };
            let backups = repo.backups();
            if backups.is_empty() {
                println!("No backups found in repository '{}'.", repo.name());
            } else {
                println!("Backups in repository '{}':", repo.name());
                for backup in backups {
                    println!("  - {:?}", backup);
                }
            }
        }

        Commands::Restore { repository, backup } => {
            eprintln!(
                "Restore not yet implemented (repository={}, backup={}).",
                repository, backup
            );
            std::process::exit(1);
        }

        Commands::Repo { command } => match command {
            RepoCommands::Create { path } => {
                if let Err(e) = fs::create_dir_all(&path) {
                    eprintln!("Failed to create directory '{}': {}", path, e);
                    std::process::exit(1);
                }
                let name = std::path::Path::new(&path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(&path)
                    .to_string();
                let repo = Repository::new(name, path.clone(), DEFAULT_ROTATION_LIMIT);
                if let Err(e) = save_repository(&repo) {
                    eprintln!("Failed to save repository metadata: {}", e);
                    std::process::exit(1);
                }
                println!("Repository created at '{}'.", path);
            }
            RepoCommands::Info { path } => {
                let repo = match load_repository(&path) {
                    Some(r) => r,
                    None => {
                        eprintln!("Repository not found at path: {}", path);
                        std::process::exit(1);
                    }
                };
                println!("Repository: {}", repo.name());
                println!("  ID:             {}", repo.id());
                println!("  Location:       {}", repo.location());
                println!("  Created at:     {}", repo.created_at());
                println!("  Rotation limit: {}", repo.rotation_limit());
                println!("  Backups:        {}", repo.backups().len());
            }
        },

        Commands::Mirror { command } => match command {
            MirrorCommands::Create {
                repository,
                path,
                retention,
            } => {
                eprintln!(
                    "Mirror creation not yet implemented (repository={}, path={}, retention={:?}).",
                    repository, path, retention
                );
                std::process::exit(1);
            }
        },
    }
}

async fn cmd_create(repository_name: String) -> Result<(), String> {
    let mut repo = load_repository(&repository_name)
        .ok_or_else(|| format!("Repository not found at path: {}", repository_name))?;

    let docker_client = get_docker_client()?;
    let containers = collect_containers_to_backup(&docker_client).await;

    let mut backup = repository::new_backup(&mut repo)?;
    create_backup(&mut backup, &containers).await?;

    save_repository(&repo)?;

    println!("Backup created successfully.");
    Ok(())
}
