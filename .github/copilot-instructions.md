# Project Guidelines

## Overview

**bdf** (backup-docker-files) is a Rust CLI tool that automatically backs up files from Docker containers based on container labels. It connects to the Docker daemon, discovers containers with `bdf.*` labels, extracts specified files via TAR streaming, and stores backups with TOML metadata.

## Code Style

- Rust Edition 2024, formatted with default `rustfmt` settings
- Snake_case for functions/variables, PascalCase for types/enums, ALL_CAPS for constants
- Use explicit getter methods for struct field access (no `pub` fields)
- Derive `Debug`, `Clone`, `Serialize`, `Deserialize` on model structs as needed
- Error handling uses `Result<T, String>` — no `anyhow`/`thiserror` crates; use descriptive `format!()` error messages
- When multiple operations can fail, accumulate errors in `Vec<String>` and join with `"; "`

## Architecture

```
src/
  main.rs          — Entry point, async tokio runtime
  constants.rs     — Shared constants (paths, filenames)
  models/          — Domain structs (Backup, Container, Labels, Repository, ExtractionType)
  services/
    backup.rs      — Backup orchestration (init → extract → copy → finalize)
    discovery.rs   — Docker container discovery via Bollard
    extraction/    — File extraction strategies (file.rs, sqlite.rs)
      utils.rs     — Staging dir creation, directory walking
```

- **Models** are pure data with serialization; business logic lives in **services**
- Extraction strategies are separate modules under `services/extraction/`; each exposes an async `extract(container) -> Result<PathBuf, String>` function
- Docker interaction uses the `bollard` crate with Unix socket defaults

## Build and Test

```bash
cargo build          # Build
cargo test           # Run all tests
cargo run            # Run (requires Docker daemon)
```

## Conventions

- Async functions use `tokio` runtime (`#[tokio::main]`, `async fn`)
- Container file extraction works by downloading TAR streams from Docker and unpacking to a staging directory (`/tmp/bdf-staging`), then copying to the final backup location
- Containers can be paused during extraction when `bdf.online=true` label is set
- Docker labels schema: `bdf.enable`, `bdf.extraction_type`, `bdf.online`, `bdf.file_path` (single) or `bdf.file_path.N` (indexed, 0-based)
- Repository/backup metadata is persisted as TOML files
- UUIDs use v7 (time-ordered)
- Tests use `#[cfg(test)]` modules with `HashMap`-based setup for label parsing
