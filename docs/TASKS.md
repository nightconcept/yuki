# yuki - Task List

## Milestone 1: Core CLI Structure, Manifest Parsing, and `apply` Command (Scoop only)

**Goal:** Establish the basic `yuki` CLI application, implement manifest parsing, and achieve a functional `apply` command that can install packages from a manifest using only **Scoop** as the first supported package manager. This will validate the core workflow and error handling.

- [x] **Task 1.1:** Setup Go project structure for `yuki`
    
    - [x] Initialize Go module (`go mod init github.com/your-username/yuki`)
    - [x] Create initial directory structure (`/cmd/yuki`, `/internal/app`, `/internal/pm/scoop`, `/internal/utils`, `/docs`)
    - [x] Add `urfave/cli` and `go-yaml/yaml` dependencies.
    - [x] Verification: Project compiles. Basic `yuki --version` command works.

- [x] **Task 1.2:** Implement YAML manifest parsing
    
    - [x] Define Go structs for the manifest structure (top-level PM keys, list of package objects with `name` and optional `version`).
    - [x] Implement function to read and unmarshal the YAML manifest file (e.g., `manifest.yaml`).
    - [x] Add error handling for file not found or malformed YAML.
    - [x] Verification: Unit tests pass for parsing valid and invalid manifest examples. `yuki apply <manifest>` can successfully load and print parsed package details (dry run).

- [x] **Task 1.3:** Implement core `apply` command logic (dispatcher)
    
    - [x] Define the `apply` command structure in `urfave/cli`.
    - [x] Implement logic to iterate through PM groups in manifest file order.
    - [x] Implement logic to iterate through packages within each PM group in manifest order.
    - [x] Verification: `yuki apply <manifest>` with a dummy manifest correctly logs the packages it would process in the correct order.

- [x] **Task 1.4:** Implement **Scoop** package manager interaction for `install`
    
    - [x] Create `internal/pm/scoop/scoop.go`.
    - [x] Implement function to construct `scoop install <package_name>[@<version>]` command string.
    - [x] Implement function to execute the `scoop` command using `os/exec`.
    - [x] Capture stdout/stderr from `scoop`.
    - [x] Basic error detection (e.g., `scoop.exe` not found, non-zero exit code).
    - [x] Verification: Unit tests for command construction. Manually test installing a Scoop package using this module's function.

- [x] **Task 1.5:** Integrate **Scoop** installation into `apply` command
    
    - [x] In `apply` command logic, if `scoop` group is processed, call the Scoop interaction module for each package.
    - [x] Implement basic error reporting for `apply` (package success/failure).
    - [x] Verification: `yuki apply <manifest_with_scoop_pkgs>` successfully installs specified Scoop packages. Errors are reported.

- [x] **Task 1.6:** Implement "Missing PM" detection for **Scoop** in `apply`
    
    - [x] Before attempting `scoop` commands, check if `scoop.exe` is in PATH or executable.
    - [x] If missing, report clearly and skip Scoop packages as per PRD.
    - [x] Verification: Test `apply` with Scoop packages when `scoop.exe` is not accessible; ensure correct reporting and skipping.

- [x] **Task 1.7:** Implement consecutive failure limit for **Scoop** in `apply`
    
    - [x] Add logic to `apply` command to track consecutive installation failures for Scoop packages.
    - [x] If 3 consecutive failures occur, skip remaining Scoop packages for that run and report.
    - [x] Verification: Test with a manifest causing 3+ consecutive Scoop failures; ensure subsequent Scoop packages are skipped.

- [x] **Task 1.8:** Basic end-of-`apply` summary
    
    - [x] Collect results (success, failure, skip reason) for each attempted package installation.
    - [x] Print a simple summary table/list at the end of the `apply` command.
    - [x] Verification: Summary accurately reflects the outcome of an `apply` run with mixed results (for Scoop packages).

## Milestone 2: Extend `apply` (Installs for Choco/Winget), Implement `list` & `--dry-run` for `apply`

**Goal:** Add install support for Chocolatey/Winget to `apply`. Implement `list` command. Crucially, implement `--dry-run` for the `apply` command's install logic.

- [ ] **Task 2.1:** Implement **Chocolatey** PM interaction for `InstallPackage`
    
    - [ ] In `internal/pm/chocolatey/chocolatey.go` (create file), add `InstallPackage`.
    - [ ] Construct/execute `choco install <pkg> [--version <v>]`. "Missing PM" detection.
    - [ ] Verification: Unit tests. Integrate into `apply`. Consecutive failure logic for Choco installs.
- [ ] **Task 2.2:** Implement **Winget** PM interaction for `InstallPackage`
    
    - [ ] In `internal/pm/winget/winget.go` (create file), add `InstallPackage`.
    - [ ] Construct/execute `winget install <pkg> --version <v> --accept-...`. "Missing PM" detection.
    - [ ] Verification: Unit tests. Integrate into `apply`. Consecutive failure logic for Winget installs.
- [ ] **Task 2.3:** Implement `--dry-run` flag and logic for `apply` (Install/Update part)
    
    - [ ] Add `--dry-run` flag to `apply` command in `urfave/cli`.
    - [ ] Modify `apply` execution flow: if `--dry-run`, log intended install/update actions instead of executing.
    - [ ] Ensure PM interaction modules can signal intended actions without executing.
    - [ ] Verification: `yuki apply --dry-run <manifest>` shows packages that _would be_ installed/version-changed, without actually installing them.
- [ ] **Task 2.4:** Implement `list` command structure and PM interaction methods
    
    - [ ] Define `list` command. Add `ListInstalledPackages` method to PM modules.
    - [ ] Verification: `yuki list` runs.
- [ ] **Task 2.5:** Implement **Scoop** `ListInstalledPackages` method and parsing
    
    - [ ] In `scoop.go`, implement `ListInstalledPackages` (runs `scoop list`), parse output.
    - [ ] Verification: Unit tests. `yuki list` shows Scoop packages.
- [ ] **Task 2.6:** Implement **Chocolatey** `ListInstalledPackages` method and parsing
    
    - [ ] In `chocolatey.go`, implement `ListInstalledPackages` (runs `choco list --local-only`), parse.
    - [ ] Verification: Unit tests. `yuki list` shows Choco packages.
- [ ] **Task 2.7:** Implement **Winget** `ListInstalledPackages` method and parsing
    
    - [ ] In `winget.go`, implement `ListInstalledPackages` (runs `winget list --accept-source-agreements`), parse.
    - [ ] Verification: Unit tests. `yuki list` shows Winget packages.
- [ ] **Task 2.8:** Finalize `list` command output formatting
    
    - [ ] Consolidate lists, section by PM, format neatly.
    
    - [ ] Verification: `yuki list` output is clean, accurate.

## Milestone 3: Implement `update --all` Command, Manifest `prune` Field Parsing, and Refinements

**Goal:** Implement `update --all`. Enhance manifest parsing to include the `prune` field per PM group. Add general polishes.

- [ ] **Task 3.1:** Enhance YAML manifest parsing for `prune` field
    
    - [ ] Update Go structs for manifest to include `Prune bool` field within each PM group.
    - [ ] Ensure `prune` defaults to `false` if omitted.
    - [ ] Verification: Unit tests for parsing manifests with and without `prune` flags. Parsed data is correct.
- [ ] **Task 3.2:** Implement `update -a / --all` command structure
    
    - [ ] Define `update` command. Add `UpdateAllPackages` method to PM modules.
    - [ ] Verification: `yuki update -a` runs.
- [ ] **Task 3.3:** Implement **Scoop** `UpdateAllPackages` method
    
    - [ ] In `scoop.go`, implement (runs `scoop update *`). Capture/display output.
    - [ ] Verification: `update -a` triggers `scoop update *`.
- [ ] **Task 3.4:** Implement **Chocolatey** `UpdateAllPackages` method
    
    - [ ] In `chocolatey.go`, implement (runs `choco upgrade all -y`). Capture/display.
    - [ ] Verification: `update -a` triggers `choco upgrade all`.
- [ ] **Task 3.5:** Implement **Winget** `UpdateAllPackages` method
    
    - [ ] In `winget.go`, implement (runs `winget upgrade --all --accept-...`). Capture/display.
    - [ ] Verification: `update -a` triggers `winget upgrade --all`.
- [ ] **Task 3.6:** Add basic logging/verbose output option (`--verbose` flag)
    
    - [ ] Implement global flag and conditional detailed logging.
    - [ ] Verification: Provides more operational output.
- [ ] **Task 3.7:** Create `manifest.example.yaml` showing `prune` usage
    
    - [ ] Well-commented example file, including `prune: true/false` and comments about Winget.
    - [ ] Verification: Clear and usable.
- [ ] **Task 3.8:** Write/Update `README.md`
    
    - [ ] Include `prune` feature description, `--dry-run` usage.
    - [ ] Verification: Comprehensive and clear.

## Milestone 4: Implement Pruning (Uninstall Unmanaged based on `prune` flag) in `apply` Command

**Goal:** Enhance `apply` to identify and remove packages for Scoop/Chocolatey if their manifest section has `prune: true`. Implement Winget warning for `prune: true`. Ensure `--dry-run` shows pruning actions.

- [ ] **Task 4.1:** Design `apply` command flow for conditional pruning phase
    
    - [ ] Define steps: post-install/update phase, check `prune` flag for Choco/Scoop sections. If `prune: true` for Winget, prepare warning.
    - [ ] Verification: Flowchart or sequence diagram updated.
- [ ] **Task 4.2:** Implement `UninstallPackage` method for **Scoop**
    
    - [ ] In `scoop.go`, add `UninstallPackage` (runs `scoop uninstall <pkg>`). Error handling.
    - [ ] Verification: Unit tests. Manually test uninstalling.
- [ ] **Task 4.3:** Implement `UninstallPackage` method for **Chocolatey**
    
    - [ ] In `chocolatey.go`, add `UninstallPackage` (runs `choco uninstall <pkg> -y`). Error handling.
    - [ ] Verification: Unit tests. Manually test uninstalling.
- [ ] **Task 4.4:** Implement (stubbed/logging) `UninstallPackage` for **Winget**
    
    - [ ] In `winget.go`, add `UninstallPackage` but it should effectively be a no-op for pruning, or only log if called inappropriately for this feature. The main pruning logic will skip calling this for Winget.
    - [ ] Verification: No actual uninstall occurs.
- [ ] **Task 4.5:** Implement state comparison logic for pruning
    
    - [ ] In `internal/app/state.go` (create file) or similar: function to take manifest packages (for a specific PM) and list of all installed packages _by that PM_, return list of packages to prune for _that PM_.
    - [ ] Verification: Unit tests with various manifest/installed scenarios for a single PM.
- [ ] **Task 4.6:** Integrate pruning logic into `apply` command (Scoop & Chocolatey)
    
    - [ ] After install/update phase, for each Choco/Scoop section:
        - If `prune: true` is set in its manifest section:
            - Call `ListInstalledPackages` for that PM.
            - Call state comparison logic to get packages to prune for that PM.
            - If not `--dry-run`, call respective PM `UninstallPackage` method for each.
            - If `--dry-run`, log packages that _would be_ uninstalled.
    - [ ] Verification: `apply` with `prune: true` for Scoop/Choco correctly identifies and (if not dry run) attempts to uninstall packages. Dry run shows correct proposed uninstalls.
- [ ] **Task 4.7:** Implement Winget `prune: true` warning
    
    - [ ] In `apply` command, if a Winget section has `prune: true` in the manifest, issue a clear warning message that pruning is not supported for Winget and no action will be taken.
    - [ ] Ensure no pruning logic is triggered for Winget packages.
    - [ ] Verification: Warning is displayed when `winget: { prune: true, ... }` is in manifest. No Winget packages are marked for pruning.
- [ ] **Task 4.8:** Refine error handling and summary for `apply` (with pruning)
    
    - [ ] Ensure uninstall errors are caught and reported.
    - [ ] Update final summary to include packages successfully/unsuccessfully uninstalled (or proposed for uninstall in dry run).
    - [ ] Verification: `apply` handles uninstall errors; summary is comprehensive for all operations.
- [ ] **Task 4.9:** Thorough end-to-end testing of `apply` with pruning and `--dry-run`
    
    - [ ] Test with various manifests: `prune: true/false` for Scoop/Choco, `prune: true` for Winget.
    - [ ] Test `--dry-run` with all pruning scenarios.
    - [ ] Test actual pruning runs.
    - [ ] Verification: `apply` command behaves as a declarative state manager correctly and safely according to `prune` flags and `--dry-run`.

## Additional Tasks / Backlog (Post-MVP M1-M4)

- [ ] Implement standalone `search` command.
- [ ] Implement standalone `uninstall` command (e.g., `yuki uninstall <packagename> --manager <pm>`).
- [ ] Add support for JSON manifest input.
- [ ] Feature: Offer to install missing PMs during `apply`.
- [ ] `--force` flag for `apply` to bypass certain safety checks (e.g., if we re-introduce any other than the `prune` logic).
- [ ] Configuration file for `yuki` itself.
- [ ] Add `args` field support to manifest for custom PM arguments.
- [ ] Investigate parallel execution for PM operations.