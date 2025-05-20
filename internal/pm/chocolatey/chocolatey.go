package chocolatey

import (
	"bytes"
	"fmt"
	"os/exec"
	"strings"
)

// Overridable for testing
var chocoExecCommand = exec.Command
var chocoLookPath = exec.LookPath

// ErrChocolateyNotInstalled is returned when Chocolatey is not found in PATH.
var ErrChocolateyNotInstalled = fmt.Errorf("chocolatey not found in PATH")

// Manager handles operations related to Chocolatey.
type Manager struct{}

// NewManager creates a new Manager for Chocolatey.
func NewManager() *Manager {
	return &Manager{}
}

// IsInstalled checks if choco.exe is available in the system's PATH.
func (m *Manager) IsInstalled() bool {
	_, err := chocoLookPath("choco")
	return err == nil
}

// InstallPackage installs a package using Chocolatey.
// It constructs the command `choco install <packageName> [--version <version>] -y`.
// It returns the combined stdout/stderr output and an error if the installation fails
// or if Chocolatey is not installed.
func (m *Manager) InstallPackage(packageName string, version string) (string, error) {
	if !m.IsInstalled() {
		return "", ErrChocolateyNotInstalled
	}

	args := []string{"install", packageName, "-y", "--no-progress"}
	if version != "" {
		args = append(args, "--version", version)
	}

	cmd := chocoExecCommand("choco", args...)

	var outb, errb bytes.Buffer
	cmd.Stdout = &outb
	cmd.Stderr = &errb

	err := cmd.Run()

	stdout := strings.TrimSpace(outb.String())
	stderr := strings.TrimSpace(errb.String())
	output := stdout
	if stderr != "" {
		if output != "" {
			output += "\n"
		}
		output += "Stderr: " + stderr
	}

	if err != nil {
		// Append stderr to the error message for more context.
		return output, fmt.Errorf("error running choco command '%s': %w. Output: %s", strings.Join(cmd.Args, " "), err, output)
	}

	return output, nil
}
