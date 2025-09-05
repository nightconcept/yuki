use async_trait::async_trait;
use anyhow::{Context, Result};
use std::time::Instant;
use tokio::process::Command;
use tracing::{debug, info};
use crate::traits::{PackageManager, PackageInfo, InstallOptions, OperationResult};

pub struct ChocolateyManager;

impl ChocolateyManager {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PackageManager for ChocolateyManager {
    fn pm_type(&self) -> String {
        "chocolatey".to_string()
    }
    
    async fn is_available(&self) -> bool {
        Command::new("choco")
            .arg("--version")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
    
    async fn install(&self, package: &str, options: InstallOptions) -> Result<()> {
        if options.dry_run {
            info!("[DRY-RUN] Would install {} via chocolatey", package);
            if let Some(version) = &options.version {
                info!("  Version: {}", version);
            }
            return Ok(());
        }
        
        if self.is_installed(package).await? {
            info!("Package {} is already installed", package);
            return Ok(());
        }
        
        let mut cmd = Command::new("choco");
        cmd.arg("install")
            .arg(package)
            .arg("-y");
        
        if let Some(version) = &options.version {
            cmd.arg("--version").arg(version);
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
            .context("Failed to execute choco install")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            if stdout.contains("may require administrative rights") || 
               stderr.contains("may require administrative rights") {
                anyhow::bail!("Chocolatey requires administrator privileges");
            }
            
            anyhow::bail!("Chocolatey install failed: {}", stderr);
        }
        
        Ok(())
    }
    
    async fn uninstall(&self, package: &str, dry_run: bool) -> Result<()> {
        if dry_run {
            info!("[DRY-RUN] Would uninstall {} via chocolatey", package);
            return Ok(());
        }
        
        let output = Command::new("choco")
            .arg("uninstall")
            .arg(package)
            .arg("-y")
            .output()
            .await?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Chocolatey uninstall failed: {}", stderr);
        }
        
        Ok(())
    }
    
    async fn update(&self, package: &str, dry_run: bool) -> Result<()> {
        if dry_run {
            info!("[DRY-RUN] Would update {} via chocolatey", package);
            return Ok(());
        }
        
        let output = Command::new("choco")
            .arg("upgrade")
            .arg(package)
            .arg("-y")
            .output()
            .await?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Chocolatey update failed: {}", stderr);
        }
        
        Ok(())
    }
    
    async fn update_all(&self, dry_run: bool) -> Result<Vec<OperationResult>> {
        if dry_run {
            info!("[DRY-RUN] Would update all chocolatey packages");
            return Ok(vec![]);
        }
        
        info!("Updating all chocolatey packages...");
        let start = Instant::now();
        
        let output = Command::new("choco")
            .arg("upgrade")
            .arg("all")
            .arg("-y")
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
        let output = Command::new("choco")
            .arg("list")
            .arg("--local-only")
            .output()
            .await?;
        
        if !output.status.success() {
            return Ok(vec![]);
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut packages = Vec::new();
        
        for line in stdout.lines() {
            if line.contains(" packages installed") {
                break;
            }
            
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && !line.starts_with("Chocolatey") {
                packages.push(PackageInfo {
                    name: parts[0].to_string(),
                    version: parts[1].to_string(),
                    pm_type: self.pm_type(),
                    source: None,
                    installed_date: None,
                });
            }
        }
        
        Ok(packages)
    }
    
    async fn is_installed(&self, package: &str) -> Result<bool> {
        let packages = self.list_installed().await?;
        Ok(packages.iter().any(|p| p.name.eq_ignore_ascii_case(package)))
    }
    
    async fn prune(&self, keep_packages: &[String], dry_run: bool) -> Result<()> {
        let installed = self.list_installed().await?;
        let to_remove: Vec<_> = installed
            .into_iter()
            .filter(|p| !keep_packages.iter().any(|k| k.eq_ignore_ascii_case(&p.name)))
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
        true
    }
}