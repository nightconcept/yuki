# yuki - Product Requirements Document (Prototype/MVP + M4)

## 1. Introduction

- **Project Idea:** `yuki` is a Command Line Interface (CLI) tool, written in Rust, designed as a declarative system configuration manager for Windows, inspired by Nix and home-manager. It allows users to declaratively manage their entire Windows environment from a YAML manifest, including software installations across three prominent package managers (Chocolatey, Scoop, and Winget) and system configuration files. `yuki` ensures the system's state matches the manifest by installing specified packages and managing configuration files. Optionally, if configured with `prune: true` for package manager sections, it will also remove packages not listed in the manifest. A `--dry-run` mode is available for all operations.
- **Problem/Need:** Windows users often struggle with reproducible system configurations. Managing software installations across multiple package managers, maintaining configuration files, and ensuring a clean, declarative state is inefficient and error-prone. Unlike Unix-like systems with tools like Nix or Ansible, Windows lacks a comprehensive declarative configuration management solution. `yuki` aims to fill this gap by providing a single, consistent interface to achieve a desired system state defined in a manifest.
- **Prototype Goal:** The main goal for this multi-milestone prototype is to create a functional CLI tool that:
    1. Successfully parses a user-defined YAML manifest describing the desired system state.
    2. Implements an `apply` command to install software from the manifest using Chocolatey, Scoop, and Winget. This command includes a `--dry-run` mode.
    3. Enhances the `apply` command with an optional `prune: true` setting within package manager sections, enabling the removal of packages not specified in their respective manifest sections.
    4. Implements configuration file management to deploy and maintain system configuration files.
    5. Implements an `update --all` command to trigger system-wide updates across all package managers.
    6. Implements a `list` command to display a consolidated view of packages and managed configurations.
    7. Handles errors gracefully and provides clear user feedback with detailed logging.
    8. Serves as a foundation for a comprehensive Windows system configuration management tool.

## 2. Core Features / User Stories

- **Feature 1: Declarative Software State Management via Manifest (`apply` command)**

  - Description: Allows the user to define the desired state of software packages from a YAML manifest file. `yuki` will ensure that all packages listed in the manifest are installed (and updated to the specified version if provided). If a Chocolatey or Scoop section in the manifest is marked with `prune: true`, `yuki` will also remove packages managed by that specific PM which are found on the system but not listed in that section of the manifest. Winget sections do not support pruning; a warning is issued if `prune: true` is set for Winget. A `--dry-run` option shows intended changes without execution.
  - User Action(s):
    - `yuki apply <manifest_filepath.yaml>`
    - `yuki apply <manifest_filepath.yaml> --dry-run`
  - Outcome(s):
    - `yuki` parses the YAML manifest.
    - For each package entry in the manifest, `yuki` calls the specified package manager (Chocolatey, Scoop, or Winget) to ensure the package is installed (and at the correct version if specified).
    - **(Pruning Phase - only for Scoop/Chocolatey sections with `prune: true`):**
      - For each PM section (Scoop, Chocolatey) marked `prune: true`:
        - `yuki` retrieves a list of all packages currently installed by that specific PM.
        - Compares this "actual installed" list against the "desired" list for that PM from the manifest.
        - Identifies packages managed by this PM that are present on the system but not declared in its manifest section.
        - These identified "extra" packages for this PM are marked for uninstallation.
      - If a Winget section has `prune: true`, a warning is issued, and no pruning analysis is done for Winget packages.
    - **Execution (if not `--dry-run`):**
      - Installs/updates packages as per manifest.
      - Uninstalls packages marked for pruning (for Scoop/Chocolatey with `prune: true`).
    - **Dry Run (`--dry-run`):**
      - `yuki` reports all packages that _would be_ installed, updated to a specific version, or uninstalled (due to pruning), without making any changes.
    - Handles errors (missing PM, package install/uninstall failure, consecutive install failures).
    - Outputs a summary of actions taken or proposed (in dry run).
  - Command: `yuki apply <manifest_filepath> [--dry-run]`
  - Key Inputs: Path to a YAML manifest. Optional `--dry-run` flag.
  - Expected Output: Console messages indicating progress/proposed actions, errors, warnings (e.g., Winget prune attempt), and a final summary. If not dry run, system state for Scoop/Chocolatey (if `prune: true`) and Winget matches the manifest installs.
- **Feature 2: System-Wide Software Update (`update --all` command)** (Same as previous PRD)

- **Feature 3: Consolidated Listing of Installed Packages (`list` command)** (Same as previous PRD)

## 3. Technical Specifications

- **Primary Language:** Rust (edition 2024)
- **Key Dependencies:** `clap` (CLI framework), `serde` + `serde_yaml` (YAML parsing), `tokio` (async runtime), `anyhow` + `thiserror` (error handling), `tracing` (structured logging), `tempfile` (temporary file management)
- **Key APIs/Integrations:** 
  - Process execution with `choco.exe`, `scoop.exe`, `winget.exe`
  - File system operations for configuration management
  - Windows-specific APIs through `windows-rs` crate if needed
- **High-Level Architectural Approach:**
  - Modular design with separate crates/modules for each package manager
  - Async-first design for concurrent operations
  - Strong type safety with serde for configuration parsing
  - Structured logging throughout the application
  - `apply` command logic with `--dry-run` mode support
  - Configuration file templating and deployment system
  - State tracking for idempotent operations
- **Critical Technical Decisions/Constraints:**
  - `--dry-run` flag is essential for user safety before making system changes
  - Structured configuration with strong typing to prevent runtime errors
  - Async operations to handle multiple package managers concurrently
  - Comprehensive error handling with context for debugging
  - Configuration file management with backup and rollback capabilities

## 4. Project Structure

A modern Rust CLI project structure using workspace organization:

```
/yuki
  Cargo.toml              # Workspace root
  /crates
    /yuki-cli             # Main CLI binary crate
      Cargo.toml
      src/
        main.rs
        cli.rs            # CLI command definitions
        config.rs         # Configuration loading
    /yuki-core            # Core library crate
      Cargo.toml
      src/
        lib.rs
        manifest.rs       # Manifest parsing and types
        apply.rs          # Apply command logic
        list.rs           # List command logic  
        update.rs         # Update command logic
        state.rs          # State management
        error.rs          # Error types
    /yuki-pm              # Package manager abstraction crate
      Cargo.toml
      src/
        lib.rs
        traits.rs         # Common PM traits
        chocolatey.rs     # Chocolatey implementation
        scoop.rs          # Scoop implementation
        winget.rs         # Winget implementation
    /yuki-config          # Configuration file management crate
      Cargo.toml
      src/
        lib.rs
        deploy.rs         # File deployment logic
        template.rs       # Template processing
        backup.rs         # Backup/restore functionality
  /docs
    PRD.md
    TASKS.md
  /examples
    basic-manifest.yaml
    full-manifest.yaml
  /tests                  # Integration tests
    common/
    test_apply.rs
    test_config.rs
```

- `crates/yuki-cli/`: Main binary with CLI interface using `clap`
- `crates/yuki-core/`: Core application logic and command implementations
- `crates/yuki-pm/`: Package manager abstraction layer with async traits
- `crates/yuki-config/`: Configuration file management system
- `examples/`: Example manifest files demonstrating features

## 5. File Descriptions (If applicable)

- **`<user_manifest>.yaml`**:
  - **Purpose:** Defines desired system state including software packages and configuration files. Inspired by Nix/home-manager declarative approach.
  - **Format:** YAML with strong typing validation.
  - **Key Contents/Structure:**

    ```yaml
# System configuration manifest
system:
  name: "development-workstation"
  description: "My development environment"

# Package manager configurations
packages:
  chocolatey:
    prune: true # Remove unmanaged packages
    packages:
      - name: "git"
        version: "2.45.1"
      - name: "nodejs-lts"

  scoop:
    prune: true
    buckets:
      - "extras"
      - "main"
    packages:
      - name: "7zip"
        bucket: "main"
      - name: "sumatrapdf"
        version: "3.5.2"

  winget:
    prune: false # Winget pruning not recommended
    packages:
      - name: "Microsoft.PowerToys"
      - name: "VideoLAN.VLC"
        version: "3.0.20"

# Configuration file management (future feature)
configs:
  - source: "./configs/gitconfig"
    target: "~/.gitconfig"
    template: false
  - source: "./configs/vscode-settings.json"
    target: "%APPDATA%/Code/User/settings.json"
    backup: true
  - source: "./configs/powershell-profile.ps1"
    target: "$PROFILE"
    template: true
    variables:
      username: "{{ env.USERNAME }}"
    ```

## 6. Future Considerations / Out of Scope (for this prototype)

- **Out of Scope for Prototype:**
  - `search` command (to find packages across PMs).
  - `uninstall` command.
  - `args` field in the manifest for passing arbitrary/custom arguments to underlying PM install commands.
  - JSON manifest input format (focus on YAML for MVP).
  - `yuki` automatically installing missing package managers (Chocolatey, Scoop, Winget). It will only report if a required PM is not found.
  - Managing packages not explicitly listed for `apply` (e.g. no `yuki remove-not-in-manifest` type of command).
  - Interactive prompts during `apply` (beyond a simple yes/no for PM installation if that were in scope).
  - Advanced output parsing for all edge cases of PM outputs; MVP will focus on common/standard formats.
  - Pruning being the default behavior (it requires `prune: true` per PM section).
  - Interactive confirmation for pruning (replaced by `--dry-run` and explicit `prune: true` flags).
- **Potential Future Enhancements (Post-Prototype):**
  - Implement `search` and `uninstall` commands.
  - Support for JSON manifest format.
  - Option for `yuki` to offer to install missing package managers.
  - Configuration file for `yuki` itself (e.g., default behaviors, PM paths).
  - More robust error recovery and interactive conflict resolution.
  - Support for an `args` field in the manifest for more granular control over PM installations.
  - Parallel execution of package installations (within constraints of each PM) or PM operations (e.g., run all list commands concurrently).
  - Plugin system to support other package sources or custom script execution.
  - Standalone `yuki uninstall <package> [--manager <pm>]` command.
  - Global `prune: true` option in manifest with per-PM overrides.
  - `--force` flag to bypass certain safety checks (use with caution).

## 7. Project-Specific Coding Rules

- **Rust Edition:** 2024 (or latest stable)
- **Formatting:** `rustfmt` with default configuration
- **Linting:** `clippy` with pedantic lints enabled
- **Key Principles:**
  - **Type Safety**: Leverage Rust's type system for correctness; use strong types over primitives
  - **Error Handling**: Use `anyhow` for application errors, `thiserror` for library errors; avoid panics in library code
  - **Async First**: Design for concurrency; use `tokio` for async operations
  - **Modularity**: Clean separation of concerns with well-defined interfaces
  - **Documentation**: Comprehensive rustdoc comments for public APIs
  - **Performance**: Efficient resource usage; avoid unnecessary allocations
- **Naming Conventions:** Follow Rust standards (`snake_case` for functions/variables, `PascalCase` for types)
- **Dependencies:** Prefer well-maintained crates; minimize dependency count; pin versions for stability
- **Testing Strategy:**
  - Unit tests for core logic (manifest parsing, command construction)
  - Integration tests for CLI commands using temporary directories
  - Property-based testing for configuration validation
  - Mock package manager interactions for deterministic testing
  - Separate test fixtures and utilities in `tests/common/`
- **Error Context:** Always provide meaningful error messages with context for debugging
- **Logging:** Structured logging with `tracing`; different log levels for development vs production
