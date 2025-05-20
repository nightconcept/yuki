# yuki - Product Requirements Document (Prototype/MVP + M4)

## 1. Introduction

- **Project Idea:** `yuki` is a Command Line Interface (CLI) tool, written in Go, designed to simplify and unify software package management on Windows. It allows users to declaratively manage software installations from a YAML manifest across three prominent package managers: Chocolatey, Scoop, and Winget. `yuki` ensures the system's state matches the manifest by installing specified packages. Optionally, if configured with `prune: true` for Chocolatey and/or Scoop sections in the manifest, it will also remove packages managed by those PMs found on the system but not listed in the manifest. A `--dry-run` mode is available for all `apply` operations.
- **Problem/Need:** Windows users often leverage multiple package managers (Chocolatey, Scoop, Winget) to access a diverse range of software. Managing these installations, updates, and ensuring a clean, declarative state through distinct command syntaxes and workflows is inefficient and error-prone. `yuki` aims to solve this by providing a single, consistent interface to achieve a desired software environment defined in a manifest.
- **Prototype Goal:** The main goal for this multi-milestone prototype is to create a functional CLI tool that:
    1. Successfully parses a user-defined YAML manifest.
    2. Implements an `apply` command to install software from the manifest using Chocolatey, Scoop, and Winget. This command includes a `--dry-run` mode.
    3. Enhances the `apply` command with an optional `prune: true` setting within Chocolatey and Scoop manifest sections, enabling the removal of packages installed by these PMs but not specified in their respective manifest sections. Winget does not support pruning via `yuki`.
    4. Implements an `update --all` command to trigger system-wide updates across all three package managers.
    5. Implements a `list` command to display a consolidated view of packages installed by these managers.
    6. Handles errors gracefully and provides clear user feedback, including warnings for unsupported operations (e.g., `prune: true` for Winget).
    7. Serves as a practical tool for the primary user to validate its usability for declarative state management.

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

- **Primary Language(s):** Go
- **Key Frameworks/Libraries:** `urfave/cli`, `gopkg.in/yaml.v3`, `os/exec`.
- **Key APIs/Integrations:** CLI interaction with `choco.exe`, `scoop.exe`, `winget.exe` (for install, list, uninstall, upgrade all/update *).
- **High-Level Architectural Approach:**
    - ... (similar to previous PRD)
    - `apply` command logic to include `--dry-run` mode.
    - Logic to check `prune` flag per PM section in the manifest.
    - Conditional execution of pruning for Scoop/Chocolatey; warning for Winget if `prune: true`.
- **Critical Technical Decisions/Constraints:**
    - `--dry-run` flag for `apply` is essential for user safety and understanding, given no interactive confirmation for pruning.
    - Pruning logic for `apply` must differentiate behavior for Scoop/Chocolatey vs. Winget.
    - Default for `prune` field if omitted in a PM section is `false`.

## 4. Project Structure

A typical Go CLI project structure using `urfave/cli` would be suitable.

/yuki
  /cmd
    /yuki           # Main application package
      main.go
  /internal
    /app            # Core application logic, command handlers
      apply.go
      update.go
      list.go
      manifest.go   # Manifest parsing and structure
    /pm             # Package manager interaction layer
      /chocolatey
        chocolatey.go
      /scoop
        scoop.go
      /winget
        winget.go
      common.go     # Interfaces or shared utilities for PM interaction
    /utils          # General utility functions
  go.mod
  go.sum
  README.md
  /docs
    PRD.md
    TASKS.md
  manifest.example.yaml

  - `cmd/yuki/main.go`: Entry point, CLI definition using `urfave/cli`.
- `internal/app/`: Contains the core logic for each command (`apply`, `update`, `list`) and manifest handling.
- `internal/pm/`: Contains specific logic for interacting with each package manager (building commands, executing, parsing output).
- `manifest.example.yaml`: An example manifest file.

## 5. File Descriptions (If applicable)

- **`<user_manifest>.yaml`**:
    - **Purpose:** Defines desired software state. Controls pruning per PM group (Scoop/Chocolatey only).
    - **Format:** YAML.
    - **Key Contents/Structure:**

    ```yaml
chocolatey:
  prune: true # If true, remove unmanaged choco packages. Defaults to false if omitted.
  packages:
    - name: "git"
      version: "2.45.1"
    - name: "nodejs-lts"

scoop:
  prune: true # If true, remove unmanaged scoop packages. Defaults to false if omitted.
  packages:
    - name: "extras/7zip"
    - name: "sumatrapdf"
      version: "3.5.2"

winget:
  # prune: true # If set, yuki issues a warning; no pruning for Winget.
  # Defaults to false if omitted.
  packages:
    - name: "Microsoft.PowerToys"
    - name: "VideoLAN.VLC"
      version: "3.0.20"
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

## 7. Project-Specific Coding Rules (Optional)

- **Language Version:** Go 1.22+ (or latest stable at time of development)
- **Formatting:** `gofmt` (or `goimports`) must be used.
- **Linting:** `golangci-lint` with a sensible default configuration (e.g., `golangci-lint run`).
- **Key Principles:**
    - Clarity and Readability: Code should be easy to understand and maintain.
    - Modularity: Keep concerns separated (CLI definition, app logic, PM interaction).
    - Explicit Error Handling: Handle errors explicitly; avoid panics in library/app code. Use `errors.Is`, `errors.As` where appropriate.
    - Testability: Write code that is testable. Unit tests for core logic (manifest parsing, command construction, output parsing for `list`). Mocking `os/exec` or PM interaction interfaces will be essential.
- **Naming Conventions:** Follow standard Go naming conventions (e.g., `camelCase` for local variables and internal functions, `PascalCase` for exported identifiers).
- **Testing (for prototype):**
    - Unit tests for manifest parsing.
    - Unit tests for the logic of constructing commands for each PM.
    - Unit tests for parsing output from `list` commands (using example/canned outputs).
    - Basic integration tests for CLI command invocation (e.g., ensuring commands run, help is displayed). Testing actual package installations with all PMs can be complex for automated CI but should be done manually during development.
