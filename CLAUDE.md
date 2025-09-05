# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

yuki is a declarative system configuration manager for Windows written in Rust, inspired by Nix and home-manager. It manages software installations across three package managers (Chocolatey, Scoop, and Winget) and system configuration files via a YAML manifest.

## Key Commands

Since the Rust implementation is not yet started, here are the planned commands:

### Setup and Build (once implemented)
```bash
# Build the project
cargo build

# Run tests
cargo test

# Run with manifest
cargo run -- apply manifest.example.yaml --dry-run

# Format code
cargo fmt

# Run linter
cargo clippy -- -W clippy::pedantic
```

## Architecture

The project uses a Rust workspace structure with these planned crates:

- `crates/yuki-cli/` - Main CLI binary using clap
- `crates/yuki-core/` - Core application logic and command implementations
- `crates/yuki-pm/` - Package manager abstraction layer with async traits
- `crates/yuki-config/` - Configuration file management system

### Core Design Principles

1. **Async-first**: All package manager operations use tokio for concurrent execution
2. **Type Safety**: Strong typing with serde for manifest parsing - never use raw strings
3. **Error Handling**: Use `anyhow` for application errors, `thiserror` for library errors
4. **Dry-Run First**: Every destructive operation must support `--dry-run` mode

## Manifest Structure

The YAML manifest defines system state with this structure:

```yaml
packages:
  chocolatey:
    prune: true  # Optional: remove unmanaged packages
    packages:
      - name: "git"
        version: "2.45.1"  # Optional

  scoop:
    prune: true
    packages:
      - name: "7zip"
        bucket: "main"  # Optional bucket specification

  winget:
    prune: false  # Winget doesn't support pruning
    packages:
      - name: "Microsoft.PowerToys"

configs:  # Future feature
  - source: "./configs/gitconfig"
    target: "~/.gitconfig"
```

## Implementation Status

Check `docs/TASKS.md` for detailed milestones. Currently:
- Milestone 1: Core structure and Scoop support (not started)
- Milestone 2: Chocolatey/Winget and list command (not started)
- Milestone 3: Update command and pruning (not started)

## Key Implementation Notes

1. **Package Manager Detection**: Always check if PM exists before operations
2. **Circuit Breaker**: Stop after 3 consecutive failures per PM
3. **Pruning Safety**: Only Scoop/Chocolatey support pruning, warn for Winget
4. **Concurrent Operations**: Use tokio for parallel PM operations but respect each PM's constraints
5. **Windows Paths**: Handle both Unix-style and Windows paths correctly
6. **Elevation**: Chocolatey often needs admin privileges - detect and inform user

## Testing Strategy

- Unit tests for manifest parsing and command construction
- Integration tests with mocked package managers
- Use `tempfile` for test isolation
- Property-based testing for configuration validation