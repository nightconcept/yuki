# yuki - Task List

## Milestone 1: Core CLI Structure, Manifest Parsing, and `apply` Command (Scoop only)

**Goal:** Establish the basic `yuki` CLI application, implement manifest parsing, and achieve a functional `apply` command that can install packages from a manifest using only **Scoop** as the first supported package manager. This will validate the core workflow and error handling.

- [ ] **Task 1.1:** Setup Go project structure for `yuki`
    
    - [ ] Initialize Go module (`go mod init github.com/your-username/yuki`)
    - [ ] Create initial directory structure (`/cmd/yuki`, `/internal/app`, `/internal/pm/scoop`, `/internal/utils`, `/docs`)
    - [ ] Add `urfave/cli` and `go-yaml/yaml` dependencies.
    - [ ] Verification: Project compiles. Basic `yuki --version` command works.
- [ ] **Task 1.2:** Implement YAML manifest parsing
    
    - [ ] Define Go structs for the manifest structure (top-level PM keys, list of package objects with `name` and optional `version`).
    - [ ] Implement function to read and unmarshal the YAML manifest file (e.g., `manifest.yaml`).
    - [ ] Add error handling for file not found or malformed YAML.
    - [ ] Verification: Unit tests pass for parsing valid and invalid manifest examples. `yuki apply <manifest>` can successfully load and print parsed package details (dry run).
- [ ] **Task 1.3:** Implement core `apply` command logic (dispatcher)
    
    - [ ] Define the `apply` command structure in `urfave/cli`.
    - [ ] Implement logic to iterate through PM groups in manifest file order.
    - [ ] Implement logic to iterate through packages within each PM group in manifest order.
    - [ ] Verification: `yuki apply <manifest>` with a dummy manifest correctly logs the packages it would process in the correct order.
- [ ] **Task 1.4:** Implement **Scoop** package manager interaction for `install`
    
    - [ ] Create `internal/pm/scoop/scoop.go`.
    - [ ] Implement function to construct `scoop install <package_name>[@<version>]` command string.
    - [ ] Implement function to execute the `scoop` command using `os/exec`.
    - [ ] Capture stdout/stderr from `scoop`.
    - [ ] Basic error detection (e.g., `scoop.exe` not found, non-zero exit code).
    - [ ] Verification: Unit tests for command construction. Manually test installing a Scoop package using this module's function.
- [ ] **Task 1.5:** Integrate **Scoop** installation into `apply` command
    
    - [ ] In `apply` command logic, if `scoop` group is processed, call the Scoop interaction module for each package.
    - [ ] Implement basic error reporting for `apply` (package success/failure).
    - [ ] Verification: `yuki apply <manifest_with_scoop_pkgs>` successfully installs specified Scoop packages. Errors are reported.
- [ ] **Task 1.6:** Implement "Missing PM" detection for **Scoop** in `apply`
    
    - [ ] Before attempting `scoop` commands, check if `scoop.exe` is in PATH or executable.
    - [ ] If missing, report clearly and skip Scoop packages as per PRD.
    - [ ] Verification: Test `apply` with Scoop packages when `scoop.exe` is not accessible; ensure correct reporting and skipping.
- [ ] **Task 1.7:** Implement consecutive failure limit for **Scoop** in `apply`
    
    - [ ] Add logic to `apply` command to track consecutive installation failures for Scoop packages.
    - [ ] If 3 consecutive failures occur, skip remaining Scoop packages for that run and report.
    - [ ] Verification: Test with a manifest causing 3+ consecutive Scoop failures; ensure subsequent Scoop packages are skipped.
- [ ] **Task 1.8:** Basic end-of-`apply` summary
    
    - [ ] Collect results (success, failure, skip reason) for each attempted package installation.
    - [ ] Print a simple summary table/list at the end of the `apply` command.
    - [ ] Verification: Summary accurately reflects the outcome of an `apply` run with mixed results (for Scoop packages).

## Milestone 2: Extend `apply` for Chocolatey & Winget, Implement `list` Command

**Goal:** Add support for Chocolatey and Winget to the `apply` command, making it fully functional across all three PMs. Implement the `list` command with output parsing, starting with Scoop.

- [ ] **Task 2.1:** Implement **Chocolatey** package manager interaction for `install`
    
    - [ ] Create `internal/pm/chocolatey/chocolatey.go`.
    - [ ] Implement functions for `choco install <package_name> [--version <version>]` command construction and execution.
    - [ ] Implement "Missing PM" detection and reporting for `choco.exe`.
    - [ ] Verification: Unit tests. Manually test installing Chocolatey packages. Integrate into `apply` and test with a manifest. Consecutive failure logic for Chocolatey.
- [ ] **Task 2.2:** Implement **Winget** package manager interaction for `install`
    
    - [ ] Create `internal/pm/winget/winget.go`.
    - [ ] Implement functions for `winget install <package_name> --version <version> --accept-package-agreements --accept-source-agreements` command construction and execution.
    - [ ] Implement "Missing PM" detection and reporting for `winget.exe`.
    - [ ] Verification: Unit tests. Manually test installing Winget packages. Integrate into `apply` and test with a manifest. Consecutive failure logic for Winget.
- [ ] **Task 2.3:** Refine `apply` command for multi-PM error handling and summary
    
    - [ ] Ensure error handling (missing PM, consecutive failures) works independently for each PM.
    - [ ] Enhance summary to clearly show results per PM.
    - [ ] Verification: `yuki apply <manifest_with_all_pms>` handles various scenarios correctly.
- [ ] **Task 2.4:** Implement `list` command structure
    
    - [ ] Define `list` command in `urfave/cli`.
    - [ ] Add logic to call PM interaction modules for listing.
    - [ ] Verification: `yuki list` runs without error (initially can just print "listing...").
- [ ] **Task 2.5:** Implement **Scoop** `list` interaction and parsing
    
    - [ ] Function in `internal/pm/scoop/scoop.go` to run `scoop list`.
    - [ ] Parse `scoop list` output to extract package name, version, and source/bucket.
    - [ ] Verification: Unit tests for parsing example `scoop list` output. `yuki list` correctly shows Scoop packages.
- [ ] **Task 2.6:** Implement **Chocolatey** `list` interaction and parsing
    
    - [ ] Function in `internal/pm/chocolatey/chocolatey.go` to run `choco list --local-only`.
    - [ ] Parse `choco list` output to extract package name and version.
    - [ ] Verification: Unit tests for parsing. `yuki list` correctly shows Chocolatey packages, sectioned.
- [ ] **Task 2.7:** Implement **Winget** `list` interaction and parsing
    
    - [ ] Function in `internal/pm/winget/winget.go` to run `winget list --accept-source-agreements`.
    - [ ] Parse `winget list` output (note: Winget's output can be verbose; aim for key fields like Name, ID, Version).
    - [ ] Verification: Unit tests for parsing. `yuki list` correctly shows Winget packages, sectioned.
- [ ] **Task 2.8:** Finalize `list` command output formatting
    
    - [ ] Ensure output is clearly sectioned by PM.
    - [ ] Display columns: Package Name, Version, Source (if any), Managing PM.
    
    - [ ] Verification: `yuki list` output is clean, well-formatted, and accurate for all three PMs.

## Milestone 3: Implement `update --all` Command and Refinements

**Goal:** Implement the `update --all` command (starting with Scoop) and add final polishes, documentation, and prepare for initial user testing.

- [ ] **Task 3.1:** Implement `update -a / --all` command structure
    
    - [ ] Define `update` command with `-a`/`--all` flag in `urfave/cli`.
    - [ ] Verification: `yuki update -a` runs.
- [ ] **Task 3.2:** Implement **Scoop** `update all` interaction
    
    - [ ] Function in `internal/pm/scoop/scoop.go` to run `scoop update *`.
    - [ ] Capture and display output.
    - [ ] Verification: `yuki update -a` correctly triggers `scoop update *`.
- [ ] **Task 3.3:** Implement **Chocolatey** `update all` interaction
    
    - [ ] Function in `internal/pm/chocolatey/chocolatey.go` to run `choco upgrade all -y` (or non-interactive equivalent).
    - [ ] Capture and display output.
    - [ ] Verification: `yuki update -a` correctly triggers `choco upgrade all`.
- [ ] **Task 3.4:** Implement **Winget** `update all` interaction
    
    - [ ] Function in `internal/pm/winget/winget.go` to run `winget upgrade --all --accept-package-agreements --accept-source-agreements` (or similar non-interactive flags).
    - [ ] Capture and display output.
    - [ ] Verification: `yuki update -a` correctly triggers `winget upgrade --all`.
- [ ] **Task 3.5:** Add basic logging/verbose output option
    
    - [ ] Implement a global `--verbose` flag.
    - [ ] Add more detailed logging (e.g., exact commands being run) when verbose is enabled.
    - [ ] Verification: `--verbose` flag provides more detailed operational output.
- [ ] **Task 3.6:** Create `manifest.example.yaml`
    
    - [ ] Create a well-commented example manifest file showcasing features.
    - [ ] Verification: Example manifest is clear and usable.
- [ ] **Task 3.7:** Write/Update `README.md`
    
    - [ ] Include project description (`yuki`), features, installation instructions (how to build/get `yuki.exe`), usage examples for all commands, and manifest format.
    - [ ] Verification: README is comprehensive and clear for a new user.
- [ ] **Task 3.8:** Manual end-to-end testing of all features
    
    - [ ] Test `apply` with a comprehensive manifest (all PMs, versions, no versions, errors).
    - [ ] Test `list` on a system with packages from all three PMs.
    - [ ] Test `update -a`.
    - [ ] Verification: All features work as per PRD.

## Additional Tasks / Backlog (Post-MVP)

[Tasks identified but not for the initial MVP release, derived from "Future Considerations" in PRD.]

- [ ] Implement `search` command
    - [ ] Verification: `yuki search <packagename>` queries all PMs.
- [ ] Implement `uninstall` command (likely manifest-driven)
    - [ ] Verification: `yuki uninstall --file <manifest>` or `yuki uninstall <packagename> --manager <pm>`
- [ ] Add support for JSON manifest input.
- [ ] Feature: Offer to install missing PMs during `apply`.
- [ ] Configuration file for `yuki` itself (e.g., default flags, PM paths).
- [ ] Add `args` field support to manifest for custom PM arguments.
- [ ] Investigate parallel execution for PM operations.
- [ ] More sophisticated output parsing for `list` and `update` to handle more edge cases or provide richer data.