# yuki - Task List

## Milestone 1: Core Rust CLI Structure, Manifest Parsing, and `apply` Command (Scoop only)

**Goal:** Establish the basic `yuki` Rust CLI application with workspace structure, implement strongly-typed manifest parsing with serde, and achieve a functional `apply` command that can install packages using **Scoop** as the first supported package manager. This validates the core async workflow and error handling.

- [x] **Task 1.1:** Setup Rust workspace project structure for `yuki`

  - [x] Initialize Cargo workspace (`cargo init --name yuki`)
  - [x] Create workspace structure (`/crates/yuki-cli`, `/crates/yuki-core`, `/crates/yuki-pm`, `/docs`)
  - [x] Configure workspace `Cargo.toml` with member crates
  - [x] Add core dependencies: `clap`, `serde`, `serde_yaml`, `tokio`, `anyhow`, `tracing`
  - [x] Verification: Project compiles. Basic `yuki --version` command works.

- [x] **Task 1.2:** Implement strongly-typed YAML manifest parsing with serde

  - [x] Define Rust structs for manifest structure in `yuki-core/src/manifest.rs`
  - [x] Implement serde derives for deserialization with validation
  - [x] Create manifest loading function with comprehensive error handling
  - [x] Add validation logic for manifest constraints and dependencies
  - [x] Verification: Unit tests pass for valid/invalid manifests. `yuki apply <manifest> --dry-run` loads and displays parsed data.

- [x] **Task 1.3:** Implement core async `apply` command logic in yuki-core

  - [x] Define `apply` command structure using `clap` in `yuki-cli`
  - [x] Implement async dispatcher logic in `yuki-core/src/apply.rs`
  - [x] Add structured logging with `tracing` for operation tracking  
  - [x] Implement `--dry-run` mode from the start
  - [x] Verification: `yuki apply <manifest> --dry-run` shows planned operations in correct order.

- [x] **Task 1.4:** Implement async **Scoop** package manager trait and implementation

  - [x] Define async `PackageManager` trait in `yuki-pm/src/traits.rs`
  - [x] Implement `ScoopManager` struct in `yuki-pm/src/scoop.rs`
  - [x] Add async command execution with `tokio::process::Command`
  - [x] Implement package availability detection and version handling
  - [x] Add comprehensive error handling with context
  - [ ] Verification: Unit tests for command construction and execution. Mock tests for different scenarios.

- [x] **Task 1.5:** Integrate **Scoop** installation into async `apply` command

  - [x] Integrate Scoop manager into `apply` command logic
  - [x] Implement async package installation with progress tracking
  - [x] Add structured error reporting with `tracing`
  - [x] Handle concurrent operations safely
  - [ ] Verification: `yuki apply <manifest_with_scoop_pkgs>` successfully installs packages with detailed logging.

- [x] **Task 1.6:** Implement package manager availability detection

  - [x] Add async PM detection before attempting operations
  - [x] Create informative error messages for missing PMs
  - [x] Implement graceful degradation when PM is unavailable
  - [ ] Verification: Test with missing `scoop.exe`; ensure proper error handling and user guidance.

- [x] **Task 1.7:** Implement failure tracking and circuit breaker pattern

  - [x] Add failure tracking per package manager
  - [x] Implement configurable failure threshold (default: 3 consecutive failures)
  - [x] Add circuit breaker to prevent cascade failures
  - [x] Provide clear feedback on failure patterns
  - [ ] Verification: Test with failing packages; ensure circuit breaker activates appropriately.

- [x] **Task 1.8:** Implement comprehensive operation summary and reporting

  - [x] Design structured result types for all operations
  - [x] Create detailed summary with success/failure/skip statistics
  - [ ] Add colorized output for better UX (when supported)
  - [x] Include timing information and performance metrics
  - [x] Verification: Summary provides actionable feedback for all operation outcomes.

## Milestone 2: Extend `apply` (Chocolatey/Winget), Implement `list` Command, and Enhanced Dry-Run

**Goal:** Add async install support for Chocolatey/Winget to `apply`. Implement comprehensive `list` command with parallel PM queries. Enhance `--dry-run` mode with detailed planning output.

- [x] **Task 2.1:** Implement async **Chocolatey** package manager

  - [x] Create `ChocolateyManager` struct implementing `PackageManager` trait
  - [x] Handle Chocolatey-specific command construction (`choco install`, `choco upgrade`)
  - [x] Implement elevated permission detection and handling
  - [x] Add Chocolatey-specific error parsing and reporting
  - [ ] Verification: Unit tests with mocked commands. Integration tests with actual Chocolatey.

- [x] **Task 2.2:** Implement async **Winget** package manager

  - [x] Create `WingetManager` struct implementing `PackageManager` trait  
  - [x] Handle Winget source agreements and interactive prompts
  - [x] Implement version specification and source handling
  - [x] Add Winget-specific output parsing for different formats
  - [ ] Verification: Unit tests and integration tests. Handle Winget's evolving CLI.

- [x] **Task 2.3:** Enhance `--dry-run` mode with detailed planning

  - [x] Implement dry-run mode as first-class feature in all PM implementations
  - [ ] Add dependency analysis and conflict detection in dry-run
  - [ ] Show detailed installation plan with version changes and dependencies
  - [ ] Include estimated operation times and resource requirements
  - [x] Verification: `yuki apply --dry-run` provides comprehensive planning output.

- [x] **Task 2.4:** Implement async `list` command with parallel PM queries

  - [x] Design `list` command structure with filtering and formatting options
  - [x] Implement concurrent querying of all package managers
  - [ ] Add package deduplication and conflict detection across PMs
  - [x] Verification: `yuki list` runs efficiently and shows consolidated results.

- [x] **Task 2.5:** Implement parallel **Scoop** package listing with caching

  - [x] Add async `list_installed` method to `ScoopManager`
  - [x] Implement output parsing for different Scoop list formats
  - [ ] Add local caching for performance on repeated calls
  - [ ] Verification: Unit tests for parsing. Performance tests for large package lists.

- [x] **Task 2.6:** Implement parallel **Chocolatey** package listing

  - [x] Add async `list_installed` method to `ChocolateyManager`
  - [x] Handle Chocolatey's verbose output and extract relevant information
  - [ ] Support different output formats (local vs. remote)
  - [ ] Verification: Unit tests with sample Chocolatey output formats.

- [x] **Task 2.7:** Implement parallel **Winget** package listing

  - [x] Add async `list_installed` method to `WingetManager`
  - [x] Handle Winget's JSON and table output formats
  - [ ] Support source filtering and package metadata extraction
  - [ ] Verification: Unit tests for different Winget output scenarios.

- [x] **Task 2.8:** Finalize consolidated list output with rich formatting

  - [x] Design user-friendly output format with tables and sections
  - [x] Add filtering options (by PM, by package name, by version)
  - [x] Implement export options (JSON, YAML for manifest generation)
  - [ ] Add package status indicators (installed, updatable, conflicts)
  - [x] Verification: `yuki list` provides actionable and well-formatted information.

## Milestone 3: Implement `update --all`, Package Pruning, and Configuration Management Foundation

**Goal:** Implement parallel `update --all` command. Add package pruning functionality with safety checks. Begin foundation for configuration file management.

- [x] **Task 3.1:** Enhance manifest parsing for advanced features

  - [x] Add `prune` field support with strong typing in manifest structs
  - [x] Implement validation rules for pruning (safety checks, warnings)
  - [ ] Add manifest schema versioning for backward compatibility
  - [x] Implement manifest validation with detailed error reporting
  - [x] Verification: Unit tests for all manifest features and edge cases.

- [x] **Task 3.2:** Implement parallel `update --all` command with progress tracking

  - [x] Design `update` command with concurrent PM operations
  - [ ] Add progress bars and real-time status updates
  - [x] Implement selective update options (by PM, by package pattern)
  - [ ] Add rollback capabilities for failed updates
  - [x] Verification: `yuki update --all` updates all PMs efficiently with clear progress.

- [x] **Task 3.3:** Implement **Scoop** update operations with dependency handling

  - [x] Add async `update_all` method with progress reporting
  - [x] Handle Scoop bucket updates and package updates separately
  - [ ] Implement dependency-aware update ordering
  - [ ] Add update conflict detection and resolution
  - [ ] Verification: Scoop updates handle complex scenarios correctly.

- [x] **Task 3.4:** Implement **Chocolatey** update operations with elevation

  - [x] Add async `update_all` method with admin privilege handling
  - [ ] Implement parallel package updates with dependency resolution
  - [ ] Handle Chocolatey-specific update scenarios (pinned packages, etc.)
  - [ ] Add update verification and rollback on failure
  - [ ] Verification: Chocolatey updates work reliably with proper permissions.

- [x] **Task 3.5:** Implement **Winget** update operations with source management

  - [x] Add async `update_all` method with source agreement handling
  - [ ] Implement update filtering and package selection
  - [ ] Handle Winget's evolving update mechanisms
  - [ ] Add support for different Winget sources and repositories
  - [ ] Verification: Winget updates work across different package sources.

- [x] **Task 3.6:** Implement package pruning with safety mechanisms

  - [x] Add pruning logic with dry-run support and confirmation prompts
  - [ ] Implement dependency analysis to prevent breaking system packages
  - [ ] Add pruning exclusion lists and safety rules
  - [ ] Create detailed pruning reports showing what will be removed
  - [x] Verification: Pruning works safely and provides clear feedback.

- [ ] **Task 3.7:** Begin configuration management foundation (yuki-config crate)

  - [ ] Create `yuki-config` crate with basic file management traits
  - [ ] Implement file copying, templating, and backup functionality
  - [ ] Add configuration validation and schema support
  - [ ] Design plugin architecture for different config types
  - [ ] Verification: Basic config file deployment works in test scenarios.
