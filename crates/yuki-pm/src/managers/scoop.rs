use async_trait::async_trait;
use anyhow::{Context, Result};
use std::time::Instant;
use tokio::process::Command;
use tracing::{debug, info};
use crate::traits::{PackageManager, PackageInfo, InstallOptions, OperationResult};

pub struct ScoopManager;

impl ScoopManager {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PackageManager for ScoopManager {
    fn pm_type(&self) -> String {
        "scoop".to_string()
    }
    
    async fn is_available(&self) -> bool {
        // On Windows, try cmd.exe first as it's more reliable
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "scoop", "--version"])
                .output()
                .await
                .map(|output| output.status.success())
                .unwrap_or(false)
        } else {
            Command::new("scoop")
                .arg("--version")
                .output()
                .await
                .map(|output| output.status.success())
                .unwrap_or(false)
        }
    }
    
    async fn install(&self, package: &str, options: InstallOptions) -> Result<()> {
        if options.dry_run {
            info!("[DRY-RUN] Would install {} via scoop", package);
            if let Some(bucket) = &options.bucket {
                info!("  From bucket: {}", bucket);
            }
            if let Some(version) = &options.version {
                info!("  Version: {}", version);
            }
            return Ok(());
        }
        
        if self.is_installed(package).await? {
            info!("Package {} is already installed", package);
            return Ok(());
        }
        
        let mut cmd = if cfg!(target_os = "windows") {
            let mut c = Command::new("cmd");
            c.args(&["/C", "scoop", "install"]);
            c
        } else {
            let mut c = Command::new("scoop");
            c.arg("install");
            c
        };
        
        if let Some(bucket) = &options.bucket {
            cmd.arg(format!("{}/{}", bucket, package));
        } else {
            cmd.arg(package);
        }
        
        if options.force {
            cmd.arg("--skip-checksum");
        }
        
        for arg in &options.args {
            cmd.arg(arg);
        }
        
        debug!("Executing: {:?}", cmd);
        
        let output = cmd
            .output()
            .await
            .context("Failed to execute scoop install")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Scoop install failed: {}", stderr);
        }
        
        Ok(())
    }
    
    async fn uninstall(&self, package: &str, dry_run: bool) -> Result<()> {
        if dry_run {
            info!("[DRY-RUN] Would uninstall {} via scoop", package);
            return Ok(());
        }
        
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "scoop", "uninstall", package])
                .output()
                .await?
        } else {
            Command::new("scoop")
                .arg("uninstall")
                .arg(package)
                .output()
                .await?
        };
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Scoop uninstall failed: {}", stderr);
        }
        
        Ok(())
    }
    
    async fn update(&self, package: &str, dry_run: bool) -> Result<()> {
        if dry_run {
            info!("[DRY-RUN] Would update {} via scoop", package);
            return Ok(());
        }
        
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "scoop", "update", package])
                .output()
                .await?
        } else {
            Command::new("scoop")
                .arg("update")
                .arg(package)
                .output()
                .await?
        };
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Scoop update failed: {}", stderr);
        }
        
        Ok(())
    }
    
    async fn update_all(&self, dry_run: bool) -> Result<Vec<OperationResult>> {
        if dry_run {
            info!("[DRY-RUN] Would update all scoop packages");
            return Ok(vec![]);
        }
        
        info!("Updating scoop buckets...");
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "scoop", "update"])
                .output()
                .await?
        } else {
            Command::new("scoop")
                .arg("update")
                .output()
                .await?
        };
        
        info!("Updating all scoop packages...");
        let start = Instant::now();
        
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "scoop", "update", "*"])
                .output()
                .await?
        } else {
            Command::new("scoop")
                .arg("update")
                .arg("*")
                .output()
                .await?
        };
        
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
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "scoop", "list"])
                .output()
                .await?
        } else {
            Command::new("scoop")
                .arg("list")
                .output()
                .await?
        };
        
        if !output.status.success() {
            return Ok(vec![]);
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut packages = Vec::new();
        let mut skip_header = true;
        
        for line in stdout.lines() {
            // Skip header lines (containing "Name" or dashes)
            if skip_header {
                if line.contains("----") || line.contains("Name") || line.contains("Version") {
                    continue;
                }
                skip_header = false;
            }
            
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0].to_string();
                let version = parts[1].to_string();
                
                // Skip if this looks like a header line or summary
                if name.contains("Name") || name.contains("----") || version.contains("Version") 
                   || name == "Installed" || version.contains("apps:") {
                    continue;
                }
                
                // The third field is usually the bucket, not a date
                let source = parts.get(2).and_then(|s| {
                    // If it looks like a date/time, skip it
                    if s.contains(':') || s.chars().all(|c| c.is_numeric() || c == '-' || c == ':') {
                        None
                    } else {
                        Some(s.to_string())
                    }
                });
                
                packages.push(PackageInfo {
                    name,
                    version,
                    pm_type: self.pm_type(),
                    source,
                    installed_date: None, // Don't store date for now
                });
            }
        }
        
        Ok(packages)
    }
    
    async fn is_installed(&self, package: &str) -> Result<bool> {
        let packages = self.list_installed().await?;
        Ok(packages.iter().any(|p| p.name == package))
    }
    
    async fn prune(&self, keep_packages: &[String], dry_run: bool) -> Result<()> {
        let installed = self.list_installed().await?;
        let to_remove: Vec<_> = installed
            .into_iter()
            .filter(|p| !keep_packages.contains(&p.name))
            .collect();
        
        if to_remove.is_empty() {
            info!("No packages to prune");
            return Ok(());
        }
        
        info!("Packages to prune: {}", to_remove.len());
        
        for pkg in to_remove {
            info!("  - {} ({})", pkg.name, pkg.version);
            if !dry_run {
                self.uninstall(&pkg.name, false).await?;
            }
        }
        
        Ok(())
    }
    
    fn supports_prune(&self) -> bool {
        true
    }
    
    fn requires_elevation(&self) -> bool {
        false
    }
}