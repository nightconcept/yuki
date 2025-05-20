package main

import (
	"fmt"
	"log"
	"os"
	"strings"

	"github.com/nightconcept/yuki/internal/app"
	"github.com/nightconcept/yuki/internal/pm/scoop"
	"github.com/urfave/cli/v2"
)

// version is the application version, set at build time.
var version = "dev" // Default to "dev" if not set by ldflags

const maxConsecutiveScoopFailures = 3

// PackageResult holds the outcome of a package operation.
// #MARKER: PackageResult struct definition
type PackageResult struct {
	Name           string
	PackageManager string
	Status         string // e.g., SUCCESS, FAILURE, SKIPPED, INFO
	Message        string // Error message or additional info
}

// installScoopPackage handles the installation of a single Scoop package.
// It returns a PackageResult summarizing the outcome.
func installScoopPackage(
	pkg app.Package,
	sectionName string,
	scoopManager *scoop.ScoopManager,
	consecutiveScoopFailures *int,
	scoopSkippingActivated *bool,
) PackageResult {
	result := PackageResult{Name: pkg.Name, PackageManager: sectionName}

	if *scoopSkippingActivated {
		msg := fmt.Sprintf("Skipping Scoop package %s due to previous consecutive failures.", pkg.Name)
		fmt.Println("  ", msg)
		result.Status = "SKIPPED"
		result.Message = "Previous consecutive failures limit reached."
		return result
	}

	if *consecutiveScoopFailures >= maxConsecutiveScoopFailures {
		msg := fmt.Sprintf("WARN: Reached %d consecutive Scoop installation failures. Skipping remaining Scoop packages for this run.", maxConsecutiveScoopFailures)
		fmt.Println("  ", msg)
		*scoopSkippingActivated = true
		// Also skip the current package
		skipMsg := fmt.Sprintf("Skipping Scoop package %s due to previous consecutive failures.", pkg.Name)
		fmt.Println("  ", skipMsg)
		result.Status = "SKIPPED"
		result.Message = "Consecutive failure limit reached."
		return result
	}

	installArgsStr := scoopManager.BuildInstallCommand(pkg.Name, pkg.Version)
	args := strings.Fields(installArgsStr)

	fmt.Printf("  Attempting to install %s package: %s (Version: %s) using scoop...\n", sectionName, pkg.Name, pkg.Version)
	stdout, stderr, err := scoopManager.RunScoopCommand(args...)

	if err != nil {
		errMsg := fmt.Sprintf("ERROR installing %s: %v", pkg.Name, err)
		fmt.Println("    ", errMsg)
		(*consecutiveScoopFailures)++
		result.Status = "FAILURE"
		result.Message = err.Error()
		if stderr != "" {
			fmt.Printf("      Scoop Stderr: %s\n", stderr)
			result.Message += "; Stderr: " + stderr
		}
		if stdout != "" {
			fmt.Printf("      Scoop Stdout: %s\n", stdout)
			// Optionally add stdout to message if needed, but typically error is primary
		}
		return result
	}

	fmt.Printf("    SUCCESS installing %s.\n", pkg.Name)
	*consecutiveScoopFailures = 0 // Reset failure counter on success
	result.Status = "SUCCESS"
	if stdout != "" {
		fmt.Printf("      Scoop Stdout: %s\n", stdout)
	}
	if stderr != "" {
		fmt.Printf("      Scoop Stderr: %s\n", stderr)
		// Optionally add non-fatal stderr to message if considered relevant for summary
	}
	return result
}

// processManifestSection handles the processing of packages within a single manifest section.
// It returns a slice of PackageResult and an error if a fundamental issue occurs.
func processManifestSection(section app.PackageManagerSection, scoopManager *scoop.ScoopManager) ([]PackageResult, error) {
	var results []PackageResult

	// Skip if the section has no packages
	if len(section.Packages) == 0 {
		return results, nil
	}

	// Check for specific package manager requirements before processing its packages
	if section.Name == "scoop" {
		if !scoopManager.IsScoopInstalled() {
			msg := "Scoop is not installed or not found in PATH. Skipping Scoop packages."
			fmt.Println(msg)
			for _, pkg := range section.Packages { // Mark all packages in this section as skipped
				results = append(results, PackageResult{
					Name:           pkg.Name,
					PackageManager: section.Name,
					Status:         "SKIPPED",
					Message:        "Scoop not installed or not found in PATH.",
				})
			}
			return results, nil // Skip this entire section
		}
	}

	fmt.Printf("Processing %s packages...\n", section.Name)
	consecutiveScoopFailures := 0
	scoopSkippingActivated := false

	for _, pkg := range section.Packages {
		if section.Name == "scoop" {
			pkgResult := installScoopPackage(pkg, section.Name, scoopManager, &consecutiveScoopFailures, &scoopSkippingActivated)
			results = append(results, pkgResult)
		} else {
			// For other package managers, keep the old behavior (dry run print)
			var versionMsg string
			if pkg.Version != "" {
				versionMsg = fmt.Sprintf("Version: %s", pkg.Version)
				fmt.Printf("  - Would process %s package: %s, %s\n", section.Name, pkg.Name, versionMsg)
			} else {
				versionMsg = "(latest)"
				fmt.Printf("  - Would process %s package: %s %s\n", section.Name, pkg.Name, versionMsg)
			}
			results = append(results, PackageResult{
				Name:           pkg.Name,
				PackageManager: section.Name,
				Status:         "INFO",
				Message:        fmt.Sprintf("Processing not yet implemented for this PM. %s", versionMsg),
			})
		}
	}
	return results, nil
}

// handleApplyCommand is the main logic for the 'apply' command.
func handleApplyCommand(c *cli.Context) error {
	manifestPath := c.Args().First()
	if manifestPath == "" {
		return fmt.Errorf("manifest path argument is required")
	}

	fmt.Printf("Loading manifest: %s\n", manifestPath)
	manifest, err := app.LoadManifest(manifestPath)
	if err != nil {
		return fmt.Errorf("failed to load manifest: %w", err)
	}

	fmt.Println("Manifest loaded. Processing packages in manifest order...")

	scoopManager := scoop.NewManager()
	var allResults []PackageResult

	for _, section := range manifest.Sections {
		sectionResults, err := processManifestSection(section, scoopManager)
		if err != nil {
			// Log the error and continue to the next section, or decide to stop all processing.
			// For now, let's log and continue, as per current behavior.
			fmt.Printf("ERROR processing section %s: %v\n", section.Name, err)
			// Potentially add a general error result for the section if desired
			// For now, individual package errors are more granularly captured.
		}
		if sectionResults != nil {
			allResults = append(allResults, sectionResults...)
		}
	}

	fmt.Println("\n--- Apply Summary ---")
	if len(allResults) == 0 {
		fmt.Println("No packages were processed.")
	} else {
		for _, res := range allResults {
			fmt.Printf("Package: %s (%s) - Status: %s", res.Name, res.PackageManager, res.Status)
			if res.Message != "" {
				fmt.Printf(" - Message: %s", res.Message)
			}
			fmt.Println()
		}
	}
	fmt.Println("-------------------")

	fmt.Println("Finished processing manifest.")
	return nil
}

// The main function, where the program execution begins.
func main() {
	appCli := &cli.App{
		Name:    "yuki",
		Usage:   "Declarative package manager for Windows",
		Version: version,
		Action: func(c *cli.Context) error {
			_ = cli.ShowAppHelp(c)
			return nil
		},
		Commands: []*cli.Command{
			{
				Name:   "apply",
				Usage:  "Applies a manifest file",
				Action: handleApplyCommand, // Use the refactored handler
			},
		},
	}

	if err := appCli.Run(os.Args); err != nil {
		log.Fatal(err)
	}
}
