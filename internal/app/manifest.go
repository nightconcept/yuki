package app

import (
	"errors"
	"fmt"
	"io"
	"log"
	"os"

	"gopkg.in/yaml.v3"
)

// Package represents a package entry in the manifest
type Package struct {
	Name    string `yaml:"name"`
	Version string `yaml:"version,omitempty"`
}

// PackageManagerSection holds packages for a specific package manager, preserving order.
type PackageManagerSection struct {
	Name     string    `yaml:"-"` // Name is derived from the key, not from YAML content itself
	Packages []Package `yaml:",inline"`
}

// Manifest represents the top-level structure of the YAML manifest file
// It will be populated to respect the order of package manager sections.
type Manifest struct {
	Sections []PackageManagerSection
}

// UnmarshalYAML implements custom unmarshaling for the Manifest struct
// to preserve the order of package manager sections.
func (m *Manifest) UnmarshalYAML(node *yaml.Node) error {
	if node.Kind != yaml.MappingNode {
		return errors.New("manifest must be a map")
	}

	m.Sections = make([]PackageManagerSection, 0, len(node.Content)/2)

	for i := 0; i < len(node.Content); i += 2 {
		keyNode := node.Content[i]
		valueNode := node.Content[i+1]

		pmName := keyNode.Value
		var packages []Package

		// Unmarshal the packages for this section
		if err := valueNode.Decode(&packages); err != nil {
			return fmt.Errorf("failed to decode packages for %s: %w", pmName, err)
		}

		m.Sections = append(m.Sections, PackageManagerSection{
			Name:     pmName,
			Packages: packages,
		})
	}
	return nil
}

// LoadManifest reads and parses a YAML manifest file into a Manifest struct
func LoadManifest(filePath string) (*Manifest, error) {
	// Open the manifest file
	file, err := os.Open(filePath)
	if err != nil {
		if os.IsNotExist(err) {
			return nil, fmt.Errorf("manifest file not found: %s", filePath)
		}
		return nil, fmt.Errorf("failed to open manifest file: %w", err)
	}
	defer func() {
		if closeErr := file.Close(); closeErr != nil {
			// Log the error but don't fail the operation for a close error
			// since we've already read the content we need
			log.Printf("warning: failed to close manifest file: %v", closeErr)
		}
	}()

	// Read the file content
	content, err := io.ReadAll(file)
	if err != nil {
		return nil, fmt.Errorf("failed to read manifest file: %w", err)
	}

	// Parse the YAML content using the custom unmarshaler
	var manifest Manifest
	if err := yaml.Unmarshal(content, &manifest); err != nil {
		return nil, fmt.Errorf("failed to parse YAML manifest: %w", err)
	}

	// Validate the manifest
	if err := validateManifest(&manifest); err != nil {
		return nil, fmt.Errorf("invalid manifest: %w", err)
	}

	return &manifest, nil
}

// validateManifest performs basic validation on the manifest
func validateManifest(manifest *Manifest) error {
	if len(manifest.Sections) == 0 {
		return errors.New("manifest must contain at least one package manager section")
	}

	for _, section := range manifest.Sections {
		// Basic check: ensure the section has a name (derived during unmarshal)
		// and is one of the known package managers for more specific validation if needed in future.
		// For now, just check if there are packages.
		if section.Name == "" { // Should not happen with current UnmarshalYAML logic
			return errors.New("package manager section found with no name")
		}
		// if len(section.Packages) == 0 {
		// This check might be too strict if an empty section is allowed
		// return fmt.Errorf("package manager section '%s' contains no packages", section.Name)
		// }
		for i, pkg := range section.Packages {
			if pkg.Name == "" {
				return fmt.Errorf("package in section '%s' at index %d is missing required 'name' field", section.Name, i)
			}
		}
	}

	return nil
}
