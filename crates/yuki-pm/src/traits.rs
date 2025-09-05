use async_trait::async_trait;
use anyhow::Result;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct PackageManagerType(String);

impl PackageManagerType {
    pub const SCOOP: &'static str = "scoop";
    pub const CHOCOLATEY: &'static str = "chocolatey";
    pub const WINGET: &'static str = "winget";
}

impl fmt::Display for PackageManagerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for PackageManagerType {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct InstallOptions {
    pub version: Option<String>,
    pub bucket: Option<String>,
    pub source: Option<String>,
    pub args: Vec<String>,
    pub dry_run: bool,
    pub force: bool,
}

impl Default for InstallOptions {
    fn default() -> Self {
        Self {
            version: None,
            bucket: None,
            source: None,
            args: Vec::new(),
            dry_run: false,
            force: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OperationResult {
    pub package_name: String,
    pub pm_type: String,
    pub operation: String,
    pub success: bool,
    pub skipped: bool,
    pub message: Option<String>,
    pub duration: std::time::Duration,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub pm_type: String,
    pub source: Option<String>,
    pub installed_date: Option<String>,
}

#[async_trait]
pub trait PackageManager: Send + Sync {
    fn pm_type(&self) -> String;
    
    async fn is_available(&self) -> bool;
    
    async fn install(&self, package: &str, options: InstallOptions) -> Result<()>;
    
    async fn uninstall(&self, package: &str, dry_run: bool) -> Result<()>;
    
    async fn update(&self, package: &str, dry_run: bool) -> Result<()>;
    
    async fn update_all(&self, dry_run: bool) -> Result<Vec<OperationResult>>;
    
    async fn list_installed(&self) -> Result<Vec<PackageInfo>>;
    
    async fn is_installed(&self, package: &str) -> Result<bool>;
    
    async fn prune(&self, keep_packages: &[String], dry_run: bool) -> Result<()>;
    
    fn supports_prune(&self) -> bool;
    
    fn requires_elevation(&self) -> bool;
}