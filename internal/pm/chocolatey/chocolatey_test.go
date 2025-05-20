package chocolatey

import (
	"errors"
	"fmt"
	"os"
	"os/exec"
	"strings"
	"testing"
)

// Mock execCommand for testing specific to this package's vars
var mockChocoExecCommand func(command string, args ...string) *exec.Cmd
var mockChocoLookPath func(file string) (string, error)

// Store original functions from our package to restore them after tests
var originalPkgExecCommand = chocoExecCommand
var originalPkgLookPath = chocoLookPath

// Store original standard library exec functions for test helper process & fallbacks
var stdLibExecCommand = exec.Command
var stdLibLookPath = exec.LookPath

// Helper to setup mocks for our package's command execution vars
func setupMocks() {
	chocoExecCommand = mockChocoExecCommand
	chocoLookPath = mockChocoLookPath
}

// Helper to teardown mocks
func teardownMocks() {
	chocoExecCommand = originalPkgExecCommand
	chocoLookPath = originalPkgLookPath
}

// TestMain manages setup and teardown of mocks for all tests in this package
func TestMain(m *testing.M) {
	setupMocks()
	code := m.Run()
	teardownMocks()
	os.Exit(code)
}

func TestManager_InstallPackage(t *testing.T) {
	tests := []struct {
		name            string
		packageName     string
		version         string
		setupMock       func()
		expectedOutput  string
		expectedError   error
		expectedCmdArgs []string // To verify command arguments
	}{
		{
			name:        "choco not installed",
			packageName: "testpkg",
			setupMock: func() {
				mockChocoLookPath = func(file string) (string, error) {
					if file == "choco" {
						return "", errors.New("not found")
					}
					return stdLibLookPath(file)
				}
				mockChocoExecCommand = func(command string, args ...string) *exec.Cmd {
					return stdLibExecCommand(command, args...)
				}
			},
			expectedError: ErrChocolateyNotInstalled,
		},
		{
			name:        "install specific version successfully",
			packageName: "pkg1",
			version:     "1.2.3",
			setupMock: func() {
				mockChocoLookPath = func(file string) (string, error) { return "/fake/choco", nil }
				mockChocoExecCommand = func(command string, args ...string) *exec.Cmd {
					assertStringsEqual(t, "choco", command)
					cs := []string{"install", "pkg1", "-y", "--no-progress", "--version", "1.2.3"}
					assertSlicesEqual(t, cs, args)
					cmd := stdLibExecCommand(os.Args[0], "-test.run=TestHelperProcess")
					cmd.Env = append(os.Environ(), "GO_WANT_HELPER_PROCESS=1")
					return cmd
				}
				if err := os.Setenv("GO_WANT_HELPER_PROCESS_EXIT_CODE", "0"); err != nil {
					t.Fatalf("Failed to set GO_WANT_HELPER_PROCESS_EXIT_CODE: %v", err)
				}
				if err := os.Setenv("GO_WANT_HELPER_PROCESS_STDOUT", "Package installed successfully"); err != nil {
					t.Fatalf("Failed to set GO_WANT_HELPER_PROCESS_STDOUT: %v", err)
				}
				if err := os.Setenv("GO_WANT_HELPER_PROCESS_STDERR", ""); err != nil {
					t.Fatalf("Failed to set GO_WANT_HELPER_PROCESS_STDERR: %v", err)
				}
			},
			expectedOutput: "Package installed successfully",
			expectedError:  nil,
		},
		{
			name:        "install latest version successfully",
			packageName: "pkg2",
			version:     "",
			setupMock: func() {
				mockChocoLookPath = func(file string) (string, error) { return "/fake/choco", nil }
				mockChocoExecCommand = func(command string, args ...string) *exec.Cmd {
					assertStringsEqual(t, "choco", command)
					cs := []string{"install", "pkg2", "-y", "--no-progress"}
					assertSlicesEqual(t, cs, args)
					cmd := stdLibExecCommand(os.Args[0], "-test.run=TestHelperProcess")
					cmd.Env = append(os.Environ(), "GO_WANT_HELPER_PROCESS=1")
					return cmd
				}
				if err := os.Setenv("GO_WANT_HELPER_PROCESS_EXIT_CODE", "0"); err != nil {
					t.Fatalf("Failed to set GO_WANT_HELPER_PROCESS_EXIT_CODE: %v", err)
				}
				if err := os.Setenv("GO_WANT_HELPER_PROCESS_STDOUT", "Latest installed"); err != nil {
					t.Fatalf("Failed to set GO_WANT_HELPER_PROCESS_STDOUT: %v", err)
				}
				if err := os.Setenv("GO_WANT_HELPER_PROCESS_STDERR", "Some warning on stderr"); err != nil {
					t.Fatalf("Failed to set GO_WANT_HELPER_PROCESS_STDERR: %v", err)
				}
			},
			expectedOutput: "Latest installed\nStderr: Some warning on stderr",
			expectedError:  nil,
		},
		{
			name:        "install fails with error",
			packageName: "failpkg",
			version:     "1.0",
			setupMock: func() {
				mockChocoLookPath = func(file string) (string, error) { return "/fake/choco", nil }
				mockChocoExecCommand = func(command string, args ...string) *exec.Cmd {
					assertStringsEqual(t, "choco", command)
					cs := []string{"install", "failpkg", "-y", "--no-progress", "--version", "1.0"}
					assertSlicesEqual(t, cs, args)
					cmd := stdLibExecCommand(os.Args[0], "-test.run=TestHelperProcess")
					cmd.Env = append(os.Environ(), "GO_WANT_HELPER_PROCESS=1")
					return cmd
				}
				if err := os.Setenv("GO_WANT_HELPER_PROCESS_EXIT_CODE", "1"); err != nil {
					t.Fatalf("Failed to set GO_WANT_HELPER_PROCESS_EXIT_CODE: %v", err)
				}
				if err := os.Setenv("GO_WANT_HELPER_PROCESS_STDOUT", "Attempting install..."); err != nil {
					t.Fatalf("Failed to set GO_WANT_HELPER_PROCESS_STDOUT: %v", err)
				}
				if err := os.Setenv("GO_WANT_HELPER_PROCESS_STDERR", "Error: Package not found"); err != nil {
					t.Fatalf("Failed to set GO_WANT_HELPER_PROCESS_STDERR: %v", err)
				}
			},
			expectedOutput: "Attempting install...\nStderr: Error: Package not found",
			expectedError:  fmt.Errorf("error running choco command 'choco install failpkg -y --no-progress --version 1.0': exit status 1. Output: Attempting install...\nStderr: Error: Package not found"),
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			tt.setupMock()                                        // Apply specific mocks for this test case
			defer func() {
				if err := os.Unsetenv("GO_WANT_HELPER_PROCESS_EXIT_CODE"); err != nil {
					t.Logf("Warning: failed to unset GO_WANT_HELPER_PROCESS_EXIT_CODE: %v", err)
				}
			}()
			defer func() {
				if err := os.Unsetenv("GO_WANT_HELPER_PROCESS_STDOUT"); err != nil {
					t.Logf("Warning: failed to unset GO_WANT_HELPER_PROCESS_STDOUT: %v", err)
				}
			}()
			defer func() {
				if err := os.Unsetenv("GO_WANT_HELPER_PROCESS_STDERR"); err != nil {
					t.Logf("Warning: failed to unset GO_WANT_HELPER_PROCESS_STDERR: %v", err)
				}
			}()

			mgr := NewManager()
			output, err := mgr.InstallPackage(tt.packageName, tt.version)

			if tt.expectedError != nil {
				if err == nil {
					t.Errorf("expected error \"%v\", got nil", tt.expectedError)
				} else if !strings.Contains(err.Error(), tt.expectedError.Error()) {
					t.Errorf("expected error containing \"%v\", got \"%v\"", tt.expectedError, err)
				}
			} else if err != nil {
				t.Errorf("expected no error, got \"%v\"", err)
			}

			if output != tt.expectedOutput {
				t.Errorf("expected output \"%s\", got \"%s\"", tt.expectedOutput, output)
			}
		})
	}
}

// TestHelperProcess isn't a real test. It's used as a helper subprocess for TestManager_InstallPackage.
// It mimics the behavior of an external command.
func TestHelperProcess(t *testing.T) {
	if os.Getenv("GO_WANT_HELPER_PROCESS") != "1" {
		return
	}
	// Print stdout and stderr from environment variables
	if _, err := fmt.Fprint(os.Stdout, os.Getenv("GO_WANT_HELPER_PROCESS_STDOUT")); err != nil {
		// If TestHelperProcess itself has an error writing to its stdout, print to its stderr and exit
		_, _ = fmt.Fprintf(os.Stderr, "TestHelperProcess: error writing to stdout: %v\n", err)
		os.Exit(1)
	}
	if _, err := fmt.Fprint(os.Stderr, os.Getenv("GO_WANT_HELPER_PROCESS_STDERR")); err != nil {
		// If TestHelperProcess has an error writing to its own stderr, not much more we can do.
		// We can try to print it, but it might also fail. Exit with a different code.
		_, _ = fmt.Fprintf(os.Stderr, "TestHelperProcess: error writing to stderr: %v\n", err)
		os.Exit(2) // Different exit code to distinguish this failure
	}
	// Get the desired exit code from an environment variable
	codeStr := os.Getenv("GO_WANT_HELPER_PROCESS_EXIT_CODE")
	if codeStr == "" {
		codeStr = "0" // Default to 0 if not set
	}
	codeVal := 0
	_, err := fmt.Sscan(codeStr, &codeVal)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Invalid exit code for helper: %s, error: %v\n", codeStr, err)
		os.Exit(1)
	}
	os.Exit(codeVal)
}

// assertStringsEqual checks if two strings are equal and fails the test if not.
func assertStringsEqual(t *testing.T, expected, actual string) {
	t.Helper()
	if expected != actual {
		t.Errorf("Expected string '%s', but got '%s'", expected, actual)
	}
}

// assertSlicesEqual checks if two string slices are equal and fails the test if not.
func assertSlicesEqual(t *testing.T, expected, actual []string) {
	t.Helper()
	if len(expected) != len(actual) {
		t.Errorf("Expected slice length %d, but got %d. Expected: %v, Actual: %v", len(expected), len(actual), expected, actual)
		return
	}
	for i := range expected {
		if expected[i] != actual[i] {
			t.Errorf("Expected slice element at index %d to be '%s', but got '%s'. Expected: %v, Actual: %v", i, expected[i], actual[i], expected, actual)
			return
		}
	}
}
