package app

import (
	"os"
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestLoadManifest(t *testing.T) {
	tests := []struct {
		name        string
		setup       func(t *testing.T) string
		expected    *Manifest
		expectError string
	}{
		{
			name: "valid manifest with all package managers",
			setup: func(t *testing.T) string {
				tempDir := t.TempDir()
				content := `chocolatey:
  - name: "git"
    version: "2.45.1"
  - name: "nodejs-lts"

scoop:
  - name: "extras/7zip"
  - name: "sumatrapdf"
    version: "3.5.2"

winget:
  - name: "Microsoft.PowerToys"
  - name: "VideoLAN.VLC"
    version: "3.0.20"`

				filePath := filepath.Join(tempDir, "manifest.yaml")
				require.NoError(t, os.WriteFile(filePath, []byte(content), 0644))
				return filePath
			},
			expected: &Manifest{
				Sections: []PackageManagerSection{
					{
						Name: "chocolatey",
						Packages: []Package{
							{Name: "git", Version: "2.45.1"},
							{Name: "nodejs-lts", Version: ""},
						},
					},
					{
						Name: "scoop",
						Packages: []Package{
							{Name: "extras/7zip", Version: ""},
							{Name: "sumatrapdf", Version: "3.5.2"},
						},
					},
					{
						Name: "winget",
						Packages: []Package{
							{Name: "Microsoft.PowerToys", Version: ""},
							{Name: "VideoLAN.VLC", Version: "3.0.20"},
						},
					},
				},
			},
		},
		{
			name: "valid manifest with one package manager",
			setup: func(t *testing.T) string {
				tempDir := t.TempDir()
				content := `scoop:
  - name: "git"
  - name: "7zip"
    version: "1.0.0"`
				filePath := filepath.Join(tempDir, "manifest.yaml")
				require.NoError(t, os.WriteFile(filePath, []byte(content), 0644))
				return filePath
			},
			expected: &Manifest{
				Sections: []PackageManagerSection{
					{
						Name: "scoop",
						Packages: []Package{
							{Name: "git", Version: ""},
							{Name: "7zip", Version: "1.0.0"},
						},
					},
				},
			},
		},
		{
			name: "missing file",
			setup: func(t *testing.T) string {
				return "nonexistent.yaml"
			},
			expectError: "manifest file not found",
		},
		{
			name: "invalid yaml",
			setup: func(t *testing.T) string {
				tempDir := t.TempDir()
				content := `invalid: yaml: here`
				filePath := filepath.Join(tempDir, "invalid.yaml")
				require.NoError(t, os.WriteFile(filePath, []byte(content), 0644))
				return filePath
			},
			expectError: "failed to parse YAML manifest",
		},
		{
			name: "empty manifest",
			setup: func(t *testing.T) string {
				tempDir := t.TempDir()
				content := "# Empty manifest\n"
				filePath := filepath.Join(tempDir, "empty.yaml")
				require.NoError(t, os.WriteFile(filePath, []byte(content), 0644))
				return filePath
			},
			expectError: "manifest must contain at least one package manager section",
		},
		{
			name: "missing package name",
			setup: func(t *testing.T) string {
				tempDir := t.TempDir()
				content := `chocolatey:
  - version: "1.0.0"  # Missing name`
				filePath := filepath.Join(tempDir, "missing_name.yaml")
				require.NoError(t, os.WriteFile(filePath, []byte(content), 0644))
				return filePath
			},
			expectError: "package in section 'chocolatey' at index 0 is missing required 'name' field",
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			filePath := tc.setup(t)

			manifest, err := LoadManifest(filePath)

			if tc.expectError != "" {
				require.Error(t, err)
				assert.Contains(t, err.Error(), tc.expectError)
			} else {
				require.NoError(t, err)
				assert.Equal(t, tc.expected, manifest)
			}
		})
	}
}
