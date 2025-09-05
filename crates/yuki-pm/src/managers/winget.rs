use async_trait::async_trait;
use anyhow::{Context, Result};
use std::time::Instant;
use tokio::process::Command;
use tracing::{debug, info, warn};
use crate::traits::{PackageManager, PackageInfo, InstallOptions, OperationResult};

pub struct WingetManager;

impl WingetManager {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PackageManager for WingetManager {
    fn pm_type(&self) -> String {
        "winget".to_string()
    }
    
    async fn is_available(&self) -> bool {
        Command::new("winget")
            .arg("--version")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
    
    async fn install(&self, package: &str, options: InstallOptions) -> Result<()> {
        if options.dry_run {
            info!("[DRY-RUN] Would install {} via winget", package);
            if let Some(version) = &options.version {
                info!("  Version: {}", version);
            }
            if let Some(source) = &options.source {
                info!("  Source: {}", source);
            }
            return Ok(());
        }
        
        if self.is_installed(package).await? {
            info!("Package {} is already installed", package);
            return Ok(());
        }
        
        let mut cmd = Command::new("winget");
        cmd.arg("install")
            .arg(package)
            .arg("--accept-source-agreements")
            .arg("--accept-package-agreements");
        
        if let Some(version) = &options.version {
            cmd.arg("--version").arg(version);
        }
        
        if let Some(source) = &options.source {
            cmd.arg("--source").arg(source);
        }
        
        if options.force {
            cmd.arg("--force");
        }
        
        for arg in &options.args {
            cmd.arg(arg);
        }
        
        debug!("Executing: {:?}", cmd);
        
        let output = cmd
            .output()
            .await
            .context("Failed to execute winget install")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            if stdout.contains("No package found") {
                anyhow::bail!("Package '{}' not found in winget", package);
            }
            
            anyhow::bail!("Winget install failed: {}", stderr);
        }
        
        Ok(())
    }
    
    async fn uninstall(&self, package: &str, dry_run: bool) -> Result<()> {
        if dry_run {
            info!("[DRY-RUN] Would uninstall {} via winget", package);
            return Ok(());
        }
        
        let output = Command::new("winget")
            .arg("uninstall")
            .arg(package)
            .arg("--silent")
            .output()
            .await?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Winget uninstall failed: {}", stderr);
        }
        
        Ok(())
    }
    
    async fn update(&self, package: &str, dry_run: bool) -> Result<()> {
        if dry_run {
            info!("[DRY-RUN] Would update {} via winget", package);
            return Ok(());
        }
        
        let output = Command::new("winget")
            .arg("upgrade")
            .arg(package)
            .arg("--accept-source-agreements")
            .arg("--accept-package-agreements")
            .output()
            .await?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Winget update failed: {}", stderr);
        }
        
        Ok(())
    }
    
    async fn update_all(&self, dry_run: bool) -> Result<Vec<OperationResult>> {
        if dry_run {
            info!("[DRY-RUN] Would update all winget packages");
            return Ok(vec![]);
        }
        
        info!("Updating all winget packages...");
        let start = Instant::now();
        
        let output = Command::new("winget")
            .arg("upgrade")
            .arg("--all")
            .arg("--accept-source-agreements")
            .arg("--accept-package-agreements")
            .output()
            .await?;
        
        let duration = start.elapsed();
        
        if output.status.success() {
            Ok(vec![OperationResult {
                package_name: "*".to_string(),
                pm_type: self.pm_type(),
                operation: "update-all".to_string(),
                success: true,
                skipped: false,
                message: None,
                duration,
            }])
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Ok(vec![OperationResult {
                package_name: "*".to_string(),
                pm_type: self.pm_type(),
                operation: "update-all".to_string(),
                success: false,
                skipped: false,
                message: Some(stderr.to_string()),
                duration,
            }])
        }
    }
    
    async fn list_installed(&self) -> Result<Vec<PackageInfo>> {
        let output = Command::new("winget")
            .arg("list")
            .arg("--accept-source-agreements")
            .arg("--disable-interactivity")
            .output()
            .await?;
        
        if !output.status.success() {
            return Ok(vec![]);
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut packages = Vec::new();
        let mut in_package_list = false;
        
        for line in stdout.lines() {
            // Skip control characters at start
            let clean_line = line.trim_start_matches(|c: char| c.is_control() || c == '\r');
            
            // Look for the separator line that comes after the header
            if clean_line.contains("------") && clean_line.len() > 50 {
                in_package_list = true;
                continue;
            }
            
            if !in_package_list {
                continue;
            }
            
            // Skip empty lines
            if clean_line.trim().is_empty() {
                continue;
            }
            
            // Parse as a package line - split by multiple spaces
            let parts: Vec<&str> = clean_line.split("  ").filter(|s| !s.trim().is_empty()).collect();
            
            if parts.len() >= 3 {
                let display_name = parts[0].trim();
                let id = parts[1].trim();
                let version = parts[2].trim();
                
                // Skip if this looks like metadata
                if id == "Id" || version == "Version" {
                    continue;
                }
                
                // Skip version-only entries (no proper ID, just a version number)
                if id.is_empty() || id == "winget" || (display_name.chars().all(|c| c.is_numeric() || c == '.') && id == "winget") {
                    continue;
                }
                
                // Clean up the package name
                let clean_name = if id.starts_with("ARP\\") {
                    // For ARP entries, use the display name but clean it up
                    let mut cleaned = display_name
                        .replace("ARP\\Machine\\X64\\", "")
                        .replace("ARP\\Machine\\X86\\", "")
                        .replace("ARP\\User\\X64\\", "")
                        .replace("ARP\\User\\X86\\", "");
                    
                    // Remove version number if it appears at the end of the name
                    // Check if the last part after space matches the version
                    if let Some(last_space_pos) = cleaned.rfind(' ') {
                        let potential_version = &cleaned[last_space_pos + 1..];
                        if potential_version == version || potential_version.chars().all(|c| c.is_numeric() || c == '.' || c == '-' || c == '_') {
                            cleaned.truncate(last_space_pos);
                        }
                    }
                    
                    cleaned.split('_').next().unwrap_or(&cleaned).to_string()
                } else if id.starts_with("MSIX\\") {
                    // For MSIX, extract a cleaner name
                    id.replace("MSIX\\", "")
                        .split('_').next().unwrap_or(id)
                        .split('.').last().unwrap_or(id)
                        .to_string()
                } else {
                    // For regular winget packages, remove company prefix
                    let parts: Vec<&str> = id.split('.').collect();
                    if parts.len() > 1 {
                        // Remove company prefix (e.g., Microsoft.Teams -> Teams)
                        parts[1..].join(".")
                    } else {
                        id.to_string()
                    }
                };
                
                // Skip if name is empty or just numbers
                if clean_name.is_empty() || clean_name.chars().all(|c| c.is_numeric() || c == '.') {
                    continue;
                }
                
                // Get additional fields if present
                let available = parts.get(3).map(|s| s.trim().to_string());
                let source = parts.get(4).map(|s| s.trim().to_string()).unwrap_or_else(|| {
                    // Guess source from ID
                    if id.starts_with("MSIX\\") {
                        "msstore".to_string()
                    } else if id.starts_with("ARP\\") {
                        "registry".to_string()  
                    } else {
                        "winget".to_string()
                    }
                });
                
                packages.push(PackageInfo {
                    name: clean_name,
                    version: version.to_string(),
                    pm_type: self.pm_type(),
                    source: Some(source),
                    installed_date: available,
                });
            }
        }
        
        debug!("Found {} winget packages", packages.len());
        Ok(packages)
    }
    
    async fn is_installed(&self, package: &str) -> Result<bool> {
        let packages = self.list_installed().await?;
        Ok(packages.iter().any(|p| p.name.contains(package) || package.contains(&p.name)))
    }
    
    async fn prune(&self, _keep_packages: &[String], _dry_run: bool) -> Result<()> {
        warn!("Winget does not support pruning - skipping");
        Ok(())
    }
    
    fn supports_prune(&self) -> bool {
        false
    }
    
    fn requires_elevation(&self) -> bool {
        false
    }
}