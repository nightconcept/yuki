package scoop

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestScoopManager_BuildInstallCommand(t *testing.T) {
	tests := []struct {
		name        string
		packageName string
		version     string
		expectedCmd string
	}{
		{
			name:        "install package without version",
			packageName: "git",
			version:     "",
			expectedCmd: "install git",
		},
		{
			name:        "install package with version",
			packageName: "nodejs",
			version:     "18.0.0",
			expectedCmd: "install nodejs@18.0.0",
		},
		{
			name:        "install package with complex name and version",
			packageName: "extras/7zip",
			version:     "23.01",
			expectedCmd: "install extras/7zip@23.01",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			sm := NewManager()
			actualCmd := sm.BuildInstallCommand(tt.packageName, tt.version)
			assert.Equal(t, tt.expectedCmd, actualCmd)
		})
	}
}

// Note: Testing RunScoopCommand directly would require mocking os/exec or having scoop installed.
// As per task 1.4, RunScoopCommand will be manually tested.
// A more comprehensive test suite might involve an interface for command execution
// that can be mocked, or using build tags to run integration tests.
