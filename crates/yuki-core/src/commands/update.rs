use anyhow::Result;
use tracing::{info, warn};
use yuki_pm::PackageManager;
use yuki_pm::managers::{ChocolateyManager, ScoopManager, WingetManager};

pub async fn run(all: bool, pm_filter: Option<String>, dry_run: bool) -> Result<()> {
    if !all && pm_filter.is_none() {
        anyhow::bail!("Please specify --all or --pm <package-manager>");
    }
    
    if dry_run {
        info!("Running in DRY-RUN mode - no changes will be made");
    }
    
    let mut total_success = 0;
    let mut total_failed = 0;
    
    if pm_filter.as_deref() == Some("scoop") || (all && pm_filter.is_none()) {
        let scoop = ScoopManager::new();
        if scoop.is_available().await {
            info!("Updating Scoop packages...");
            match scoop.update_all(dry_run).await {
                Ok(results) => {
                    for result in results {
                        if result.success {
                            total_success += 1;
                        } else {
                            total_failed += 1;
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to update Scoop packages: {}", e);
                    total_failed += 1;
                }
            }
        } else {
            info!("Scoop is not available");
        }
    }
    
    if pm_filter.as_deref() == Some("chocolatey") || (all && pm_filter.is_none()) {
        let choco = ChocolateyManager::new();
        if choco.is_available().await {
            if choco.requires_elevation() {
                warn!("Chocolatey may require administrator privileges");
            }
            info!("Updating Chocolatey packages...");
            match choco.update_all(dry_run).await {
                Ok(results) => {
                    for result in results {
                        if result.success {
                            total_success += 1;
                        } else {
                            total_failed += 1;
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to update Chocolatey packages: {}", e);
                    total_failed += 1;
                }
            }
        } else {
            info!("Chocolatey is not available");
        }
    }
    
    if pm_filter.as_deref() == Some("winget") || (all && pm_filter.is_none()) {
        let winget = WingetManager::new();
        if winget.is_available().await {
            info!("Updating Winget packages...");
            match winget.update_all(dry_run).await {
                Ok(results) => {
                    for result in results {
                        if result.success {
                            total_success += 1;
                        } else {
                            total_failed += 1;
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to update Winget packages: {}", e);
                    total_failed += 1;
                }
            }
        } else {
            info!("Winget is not available");
        }
    }
    
    println!("\n╔══════════════════════════════════════╗");
    println!("║        UPDATE SUMMARY                ║");
    println!("╚══════════════════════════════════════╝");
    
    if dry_run {
        println!("  Mode: DRY RUN (no changes made)");
    }
    
    println!("  ✓ Succeeded: {}", total_success);
    if total_failed > 0 {
        println!("  ✗ Failed: {}", total_failed);
    }
    
    if total_failed > 0 {
        anyhow::bail!("{} update operations failed", total_failed);
    }
    
    Ok(())
}