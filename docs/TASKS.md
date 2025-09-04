# yuki - Task List

## Milestone 1: Core Rust CLI Structure, Manifest Parsing, and `apply` Command (Scoop only)

**Goal:** Establish the basic `yuki` Rust CLI application with workspace structure, implement strongly-typed manifest parsing with serde, and achieve a functional `apply` command that can install packages using **Scoop** as the first supported package manager. This validates the core async workflow and error handling.

- [ ] **Task 1.1:** Setup Rust workspace project structure for `yuki`

  - [ ] Initialize Cargo workspace (`cargo init --name yuki`)
  - [ ] Create workspace structure (`/crates/yuki-cli`, `/crates/yuki-core`, `/crates/yuki-pm`, `/docs`)
  - [ ] Configure workspace `Cargo.toml` with member crates
  - [ ] Add core dependencies: `clap`, `serde`, `serde_yaml`, `tokio`, `anyhow`, `tracing`
  - [ ] Verification: Project compiles. Basic `yuki --version` command works.

- [ ] **Task 1.2:** Implement strongly-typed YAML manifest parsing with serde

  - [ ] Define Rust structs for manifest structure in `yuki-core/src/manifest.rs`
  - [ ] Implement serde derives for deserialization with validation
  - [ ] Create manifest loading function with comprehensive error handling
  - [ ] Add validation logic for manifest constraints and dependencies
  - [ ] Verification: Unit tests pass for valid/invalid manifests. `yuki apply <manifest> --dry-run` loads and displays parsed data.

- [ ] **Task 1.3:** Implement core async `apply` command logic in yuki-core

  - [ ] Define `apply` command structure using `clap` in `yuki-cli`
  - [ ] Implement async dispatcher logic in `yuki-core/src/apply.rs`
  - [ ] Add structured logging with `tracing` for operation tracking  
  - [ ] Implement `--dry-run` mode from the start
  - [ ] Verification: `yuki apply <manifest> --dry-run` shows planned operations in correct order.

- [ ] **Task 1.4:** Implement async **Scoop** package manager trait and implementation

  - [ ] Define async `PackageManager` trait in `yuki-pm/src/traits.rs`
  - [ ] Implement `ScoopManager` struct in `yuki-pm/src/scoop.rs`
  - [ ] Add async command execution with `tokio::process::Command`
  - [ ] Implement package availability detection and version handling
  - [ ] Add comprehensive error handling with context
  - [ ] Verification: Unit tests for command construction and execution. Mock tests for different scenarios.

- [ ] **Task 1.5:** Integrate **Scoop** installation into async `apply` command

  - [ ] Integrate Scoop manager into `apply` command logic
  - [ ] Implement async package installation with progress tracking
  - [ ] Add structured error reporting with `tracing`
  - [ ] Handle concurrent operations safely
  - [ ] Verification: `yuki apply <manifest_with_scoop_pkgs>` successfully installs packages with detailed logging.

- [ ] **Task 1.6:** Implement package manager availability detection

  - [ ] Add async PM detection before attempting operations
  - [ ] Create informative error messages for missing PMs
  - [ ] Implement graceful degradation when PM is unavailable
  - [ ] Verification: Test with missing `scoop.exe`; ensure proper error handling and user guidance.

- [ ] **Task 1.7:** Implement failure tracking and circuit breaker pattern

  - [ ] Add failure tracking per package manager
  - [ ] Implement configurable failure threshold (default: 3 consecutive failures)
  - [ ] Add circuit breaker to prevent cascade failures
  - [ ] Provide clear feedback on failure patterns
  - [ ] Verification: Test with failing packages; ensure circuit breaker activates appropriately.

- [ ] **Task 1.8:** Implement comprehensive operation summary and reporting

  - [ ] Design structured result types for all operations
  - [ ] Create detailed summary with success/failure/skip statistics
  - [ ] Add colorized output for better UX (when supported)
  - [ ] Include timing information and performance metrics
  - [ ] Verification: Summary provides actionable feedback for all operation outcomes.

## Milestone 2: Extend `apply` (Chocolatey/Winget), Implement `list` Command, and Enhanced Dry-Run

**Goal:** Add async install support for Chocolatey/Winget to `apply`. Implement comprehensive `list` command with parallel PM queries. Enhance `--dry-run` mode with detailed planning output.

- [ ] **Task 2.1:** Implement async **Chocolatey** package manager

  - [ ] Create `ChocolateyManager` struct implementing `PackageManager` trait
  - [ ] Handle Chocolatey-specific command construction (`choco install`, `choco upgrade`)
  - [ ] Implement elevated permission detection and handling
  - [ ] Add Chocolatey-specific error parsing and reporting
  - [ ] Verification: Unit tests with mocked commands. Integration tests with actual Chocolatey.

- [ ] **Task 2.2:** Implement async **Winget** package manager

  - [ ] Create `WingetManager` struct implementing `PackageManager` trait  
  - [ ] Handle Winget source agreements and interactive prompts
  - [ ] Implement version specification and source handling
  - [ ] Add Winget-specific output parsing for different formats
  - [ ] Verification: Unit tests and integration tests. Handle Winget's evolving CLI.

- [ ] **Task 2.3:** Enhance `--dry-run` mode with detailed planning

  - [ ] Implement dry-run mode as first-class feature in all PM implementations
  - [ ] Add dependency analysis and conflict detection in dry-run
  - [ ] Show detailed installation plan with version changes and dependencies
  - [ ] Include estimated operation times and resource requirements
  - [ ] Verification: `yuki apply --dry-run` provides comprehensive planning output.

- [ ] **Task 2.4:** Implement async `list` command with parallel PM queries

  - [ ] Design `list` command structure with filtering and formatting options
  - [ ] Implement concurrent querying of all package managers
  - [ ] Add package deduplication and conflict detection across PMs
  - [ ] Verification: `yuki list` runs efficiently and shows consolidated results.

- [ ] **Task 2.5:** Implement parallel **Scoop** package listing with caching

  - [ ] Add async `list_installed` method to `ScoopManager`
  - [ ] Implement output parsing for different Scoop list formats
  - [ ] Add local caching for performance on repeated calls
  - [ ] Verification: Unit tests for parsing. Performance tests for large package lists.

- [ ] **Task 2.6:** Implement parallel **Chocolatey** package listing

  - [ ] Add async `list_installed` method to `ChocolateyManager`
  - [ ] Handle Chocolatey's verbose output and extract relevant information
  - [ ] Support different output formats (local vs. remote)
  - [ ] Verification: Unit tests with sample Chocolatey output formats.

- [ ] **Task 2.7:** Implement parallel **Winget** package listing

  - [ ] Add async `list_installed` method to `WingetManager`
  - [ ] Handle Winget's JSON and table output formats
  - [ ] Support source filtering and package metadata extraction
  - [ ] Verification: Unit tests for different Winget output scenarios.

- [ ] **Task 2.8:** Finalize consolidated list output with rich formatting

  - [ ] Design user-friendly output format with tables and sections
  - [ ] Add filtering options (by PM, by package name, by version)
  - [ ] Implement export options (JSON, YAML for manifest generation)
  - [ ] Add package status indicators (installed, updatable, conflicts)
  - [ ] Verification: `yuki list` provides actionable and well-formatted information.

## Milestone 3: Implement `update --all`, Package Pruning, and Configuration Management Foundation

**Goal:** Implement parallel `update --all` command. Add package pruning functionality with safety checks. Begin foundation for configuration file management.

- [ ] **Task 3.1:** Enhance manifest parsing for advanced features

  - [ ] Add `prune` field support with strong typing in manifest structs
  - [ ] Implement validation rules for pruning (safety checks, warnings)
  - [ ] Add manifest schema versioning for backward compatibility
  - [ ] Implement manifest validation with detailed error reporting
  - [ ] Verification: Unit tests for all manifest features and edge cases.

- [ ] **Task 3.2:** Implement parallel `update --all` command with progress tracking

  - [ ] Design `update` command with concurrent PM operations
  - [ ] Add progress bars and real-time status updates
  - [ ] Implement selective update options (by PM, by package pattern)
  - [ ] Add rollback capabilities for failed updates
  - [ ] Verification: `yuki update --all` updates all PMs efficiently with clear progress.

- [ ] **Task 3.3:** Implement **Scoop** update operations with dependency handling

  - [ ] Add async `update_all` method with progress reporting
  - [ ] Handle Scoop bucket updates and package updates separately
  - [ ] Implement dependency-aware update ordering
  - [ ] Add update conflict detection and resolution
  - [ ] Verification: Scoop updates handle complex scenarios correctly.

- [ ] **Task 3.4:** Implement **Chocolatey** update operations with elevation

  - [ ] Add async `update_all` method with admin privilege handling
  - [ ] Implement parallel package updates with dependency resolution
  - [ ] Handle Chocolatey-specific update scenarios (pinned packages, etc.)
  - [ ] Add update verification and rollback on failure
  - [ ] Verification: Chocolatey updates work reliably with proper permissions.

- [ ] **Task 3.5:** Implement **Winget** update operations with source management

  - [ ] Add async `update_all` method with source agreement handling
  - [ ] Implement update filtering and package selection
  - [ ] Handle Winget's evolving update mechanisms
  - [ ] Add support for different Winget sources and repositories
  - [ ] Verification: Winget updates work across different package sources.

- [ ] **Task 3.6:** Implement package pruning with safety mechanisms

  - [ ] Add pruning logic with dry-run support and confirmation prompts
  - [ ] Implement dependency analysis to prevent breaking system packages
  - [ ] Add pruning exclusion lists and safety rules
  - [ ] Create detailed pruning reports showing what will be removed
  - [ ] Verification: Pruning works safely and provides clear feedback.

- [ ] **Task 3.7:** Begin configuration management foundation (yuki-config crate)

  - [ ] Create `yuki-config` crate with basic file management traits
  - [ ] Implement file copying, templating, and backup functionality
  - [ ] Add configuration validation and schema support
  - [ ] Design plugin architecture for different config types
  - [ ] Verification: Basic config file deployment works in test scenarios.
