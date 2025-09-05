use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{debug, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(default)]
    pub packages: PackagesConfig,
    
    #[serde(default)]
    pub configs: Vec<ConfigFile>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackagesConfig {
    #[serde(default)]
    pub chocolatey: PackageManagerConfig,
    
    #[serde(default)]
    pub scoop: PackageManagerConfig,
    
    #[serde(default)]
    pub winget: PackageManagerConfig,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackageManagerConfig {
    #[serde(default)]
    pub prune: bool,
    
    #[serde(default)]
    pub packages: Vec<Package>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    
    #[serde(default)]
    pub args: Vec<String>,
    
    #[serde(default)]
    pub pre_install: Vec<String>,
    
    #[serde(default)]
    pub post_install: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFile {
    pub source: String,
    pub target: String,
    
    #[serde(default)]
    pub backup: bool,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<bool>,
}

impl Manifest {
    pub async fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        info!("Loading manifest from: {}", path.display());
        
        let contents = tokio::fs::read_to_string(path)
            .await
            .with_context(|| format!("Failed to read manifest file: {}", path.display()))?;
        
        let manifest: Self = serde_yaml::from_str(&contents)
            .with_context(|| format!("Failed to parse YAML manifest: {}", path.display()))?;
        
        manifest.validate()?;
        
        debug!("Successfully loaded manifest with {} total packages", 
            manifest.total_packages());
        
        Ok(manifest)
    }
    
    pub fn validate(&self) -> Result<()> {
        if self.packages.winget.prune {
            tracing::warn!("Winget does not support pruning - this setting will be ignored");
        }
        
        for pkg in &self.packages.chocolatey.packages {
            self.validate_package(pkg, "chocolatey")?;
        }
        
        for pkg in &self.packages.scoop.packages {
            self.validate_package(pkg, "scoop")?;
        }
        
        for pkg in &self.packages.winget.packages {
            self.validate_package(pkg, "winget")?;
        }
        
        Ok(())
    }
    
    fn validate_package(&self, pkg: &Package, pm: &str) -> Result<()> {
        if pkg.name.is_empty() {
            anyhow::bail!("Package name cannot be empty in {} section", pm);
        }
        
        if let Some(version) = &pkg.version {
            if version.is_empty() {
                anyhow::bail!("Package version cannot be empty for {} in {}", pkg.name, pm);
            }
        }
        
        if pm != "scoop" && pkg.bucket.is_some() {
            tracing::warn!("Bucket specification for {} is only valid for Scoop, ignoring", pkg.name);
        }
        
        Ok(())
    }
    
    pub fn total_packages(&self) -> usize {
        self.packages.chocolatey.packages.len() +
        self.packages.scoop.packages.len() +
        self.packages.winget.packages.len()
    }
    
    pub fn has_configs(&self) -> bool {
        !self.configs.is_empty()
    }
}

impl PackageManagerConfig {
    pub fn is_empty(&self) -> bool {
        self.packages.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_parse_minimal_manifest() {
        let yaml = r#"
packages:
  scoop:
    packages:
      - name: git
"#;
        let manifest: Manifest = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(manifest.packages.scoop.packages.len(), 1);
        assert_eq!(manifest.packages.scoop.packages[0].name, "git");
    }
    
    #[tokio::test]
    async fn test_parse_full_manifest() {
        let yaml = r#"
packages:
  chocolatey:
    prune: true
    packages:
      - name: git
        version: "2.45.1"
  scoop:
    prune: true
    packages:
      - name: 7zip
        bucket: main
  winget:
    packages:
      - name: Microsoft.PowerToys
configs:
  - source: ./configs/gitconfig
    target: ~/.gitconfig
    backup: true
"#;
        let manifest: Manifest = serde_yaml::from_str(yaml).unwrap();
        assert!(manifest.packages.chocolatey.prune);
        assert_eq!(manifest.packages.chocolatey.packages[0].version, Some("2.45.1".to_string()));
        assert_eq!(manifest.configs.len(), 1);
    }
    
    #[test]
    fn test_validate_empty_package_name() {
        let yaml = r#"
packages:
  scoop:
    packages:
      - name: ""
"#;
        let manifest: Manifest = serde_yaml::from_str(yaml).unwrap();
        assert!(manifest.validate().is_err());
    }
}