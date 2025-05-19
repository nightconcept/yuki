# yuki - Product Requirements Document (Prototype/MVP)

## 1. Introduction

- **Project Idea:** `yuki` is a Command Line Interface (CLI) tool, written in Go, designed to simplify and unify software package management on Windows. It allows users to declaratively manage software installations from a YAML manifest across three prominent package managers: Chocolatey, Scoop, and Winget. It also provides unified commands to update all software managed by these PMs and to list installed software across them.
- **Problem/Need:** Windows users often leverage multiple package managers (Chocolatey, Scoop, Winget) to access a diverse range of software. Managing these installations and updates through distinct command syntaxes and workflows is inefficient, particularly when setting up new machines or maintaining a consistent software environment based on a declarative configuration. `yuki` aims to solve this by providing a single, consistent interface for these common tasks.
- **Prototype Goal:** The main goal for this MVP is to create a functional CLI tool that:
    1. Successfully parses a user-defined YAML manifest specifying packages for Chocolatey, Scoop, and Winget.
    2. Implements an `apply` command to install software (including specific versions if requested) from the manifest using the designated package managers.
    3. Implements an `update --all` command to trigger system-wide updates across all three package managers.
    4. Implements a `list` command to display a consolidated view of packages installed by these managers.
    5. Handles errors gracefully and provides clear user feedback.
    6. Serves as a practical tool for the primary user (developer setting up/maintaining their PC) to validate its usability and approach.

## 2. Core Features / User Stories

- **Feature 1: Declarative Software Installation via Manifest (`apply` command)**
    
    - Description: Allows the user to install a list of software packages from a YAML manifest file. The manifest specifies the package name, the package manager to use, and an optional version.
    - User Action(s): User runs `yuki apply <manifest_filepath.yaml>`
    - Outcome(s):
        - `yuki` parses the YAML manifest.
        - For each package entry, `yuki` calls the specified package manager (Chocolatey, Scoop, or Winget) to install the package.
        - If a version is specified in the manifest, `yuki` attempts to install that specific version. Otherwise, the latest stable version is installed.
        - `yuki` processes package manager groups (e.g., all `chocolatey` packages, then all `scoop` packages) in the order they appear in the manifest file. Packages within each group are also processed in order.
        - Provides feedback on the success or failure of each installation.
        - Handles errors as defined (missing PM, package install failure, consecutive failure limit).
        - Outputs a summary of actions taken.
    - Command: `yuki apply <manifest_filepath>`
    - Key Inputs: Path to a valid YAML manifest file.
    - Expected Output: Console messages indicating progress, errors, and a final summary. Software listed in the manifest is installed.
- **Feature 2: System-Wide Software Update (`update --all` command)**
    
    - Description: Allows the user to update all packages managed by Chocolatey, Scoop, and Winget on their system using a single command. This command operates independently of any manifest file.
    - User Action(s): User runs `yuki update -a` or `yuki update --all`.
    - Outcome(s):
        - `yuki` executes the native "update all" commands for Chocolatey (e.g., `choco upgrade all -y`).
        - `yuki` executes the native "update all" commands for Scoop (e.g., `scoop update *`).
        - `yuki` executes the native "update all" commands for Winget (e.g., `winget upgrade --all --accept-package-agreements --accept-source-agreements`).
        - Displays output/summary from these operations.
    - Command: `yuki update -a` (alias `--all`)
    - Key Inputs: None (beyond the command itself).
    - Expected Output: Console messages indicating which package managers are being updated and their respective outputs or a summary.
- **Feature 3: Consolidated Listing of Installed Packages (`list` command)**
    
    - Description: Allows the user to see a consolidated list of software packages installed across Chocolatey, Scoop, and Winget. This command operates independently of any manifest file.
    - User Action(s): User runs `yuki list`.
    - Outcome(s):
        - `yuki` executes the native "list installed" commands for Chocolatey, Scoop, and Winget.
        - Parses the output from each package manager.
        - Displays a formatted list, clearly sectioned by package manager (Chocolatey, Scoop, Winget).
        - For each package, shows: Package Name, Installed Version, Source/Bucket (if applicable, e.g., Scoop's bucket), and Managing PM.
    - Command: `yuki list`
    - Key Inputs: None.
    - Expected Output: Formatted console output listing installed packages.

## 3. Technical Specifications

- **Primary Language(s):** Go (latest stable version, e.g., 1.22+ as of May 2025)
- **Key Frameworks/Libraries:**
    - CLI Framework: `urfave/cli` (v2 or latest stable)
    - YAML parsing: `gopkg.in/yaml.v3`
    - OS command execution: standard `os/exec`
- **Database (if any):** None for MVP.
- **Key APIs/Integrations (if any):** Direct CLI interaction with:
    - `choco.exe` (Chocolatey)
    - `scoop.exe` (Scoop)
    - `winget.exe` (Winget)
- **Deployment Target (if applicable for prototype):** Local executable for Windows (`yuki.exe`).
- **High-Level Architectural Approach:**
    - Modular CLI application built with `urfave/cli`, with commands for `apply`, `update`, and `list`.
    - Separate Go packages/modules for interacting with each underlying package manager (e.g., `internal/chocolatey`, `internal/scoop`, `internal/winget`). These modules will be responsible for:
        - Constructing the correct CLI arguments for the specific package manager.
        - Executing the command using `os/exec`.
        - Parsing the output (stdout, stderr) from the package manager CLIs into structured data where necessary (especially for the `list` command).
    - A core orchestration layer for the `apply` command to manage manifest parsing, package processing order, and error handling logic (including consecutive failure tracking).
    - YAML parsing module for the manifest file.
- **Critical Technical Decisions/Constraints:**
    - Must be able to locate and execute the `.exe` files for Chocolatey, Scoop, and Winget. Assumes they are in the system's PATH.
    - Output parsing from third-party CLIs can be fragile. Initial implementation should target known output formats of current stable versions of these managers.
    - Error handling needs to distinguish between `yuki` application errors, underlying PM not found, and errors reported _from_ the underlying PMs.
    - For the `apply` command, maintain state for consecutive failures per package manager within a single run.

## 4. Project Structure (Optional)

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

- **`<user_manifest>.yaml`** (User-provided, e.g., `my_setup.yaml`):
    - **Purpose:** Defines the desired state of software to be installed by the `yuki apply` command.
    - **Format:** YAML.
    - **Key Contents/Structure:**

    ```yaml
    # Example: my_setup.yaml
chocolatey:
  - name: "git"
    version: "2.45.1" # Optional: specific version
  - name: "nodejs-lts" # Installs latest LTS

scoop:
  - name: "extras/7zip" # Bucket prefix included in name if not in main
  - name: "sumatrapdf"
    version: "3.5.2" # Optional

winget:
  - name: "Microsoft.PowerToys"
  - name: "VideoLAN.VLC"
    version: "3.0.20" # Optional
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
- **Potential Future Enhancements (Post-Prototype):**
    - Implement `search` and `uninstall` commands.
    - Support for JSON manifest format.
    - Option for `yuki` to offer to install missing package managers.
    - Configuration file for `yuki` itself (e.g., default behaviors, PM paths).
    - More robust error recovery and interactive conflict resolution.
    - Support for an `args` field in the manifest for more granular control over PM installations.
    - Parallel execution of package installations (within constraints of each PM) or PM operations (e.g., run all list commands concurrently).
    - Plugin system to support other package sources or custom script execution.

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
