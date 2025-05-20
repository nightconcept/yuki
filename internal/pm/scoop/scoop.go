package scoop

import (
	"bytes"
	"fmt"
	"os/exec"
	"strings"
)

// ScoopManager handles operations related to Scoop.
type ScoopManager struct{}

// NewManager creates a new ScoopManager.
func NewManager() *ScoopManager {
	return &ScoopManager{}
}

// IsScoopInstalled checks if scoop.exe is available in the system's PATH.
func (sm *ScoopManager) IsScoopInstalled() bool {
	_, err := exec.LookPath("scoop")
	return err == nil
}

// BuildInstallCommand constructs the scoop install command string.
// It takes a packageName and an optional version.
// If version is provided, it appends "@version" to the package name.
func (sm *ScoopManager) BuildInstallCommand(packageName string, version string) string {
	if version != "" {
		return fmt.Sprintf("install %s@%s", packageName, version)
	}
	return fmt.Sprintf("install %s", packageName)
}

// RunScoopCommand executes a given scoop command and its arguments.
// It returns the standard output, standard error, and an error if the command execution fails
// or if scoop.exe is not found in PATH.
func (sm *ScoopManager) RunScoopCommand(args ...string) (string, string, error) {
	scoopPath, err := exec.LookPath("scoop")
	if err != nil {
		return "", "", fmt.Errorf("scoop.exe not found in PATH: %w", err)
	}

	cmd := exec.Command(scoopPath, args...)

	var stdout, stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err = cmd.Run()

	stdoutStr := strings.TrimSpace(stdout.String())
	stderrStr := strings.TrimSpace(stderr.String())

	if err != nil {
		// err already contains information about the exit code if that's the issue
		// It could also be other errors like I/O problems with pipes, etc.
		// We'll append stderr to the error message for more context if stderr is not empty.
		if stderrStr != "" {
			return stdoutStr, stderrStr, fmt.Errorf("error running scoop command '%s %s': %w. Stderr: %s", scoopPath, strings.Join(args, " "), err, stderrStr)
		}
		return stdoutStr, stderrStr, fmt.Errorf("error running scoop command '%s %s': %w", scoopPath, strings.Join(args, " "), err)
	}

	return stdoutStr, stderrStr, nil
}
