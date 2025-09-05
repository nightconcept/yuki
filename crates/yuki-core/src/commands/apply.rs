use anyhow::Result;
use std::path::PathBuf;
use std::time::Instant;
use tracing::{debug, error, info, warn};
use yuki_pm::{PackageManager, OperationResult, InstallOptions};
use yuki_pm::managers::{ChocolateyManager, ScoopManager, WingetManager};
use crate::manifest::{Manifest, PackageManagerConfig};

pub struct ApplyResult {
    pub total_packages: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub skipped: usize,
    pub duration: std::time::Duration,
    pub dry_run: bool,
    pub details: Vec<OperationResult>,
}

pub async fn run(manifest_path: PathBuf, dry_run: bool, force: bool) -> Result<()> {
    let start = Instant::now();
    
    if dry_run {
        info!("Running in DRY-RUN mode - no changes will be made");
    }
    
    let manifest = Manifest::load(&manifest_path).await?;
    info!("Loaded manifest with {} packages", manifest.total_packages());
    
    if manifest.total_packages() == 0 {
        warn!("No packages defined in manifest");
        return Ok(());
    }
    
    let mut results = Vec::new();
    let mut circuit_breakers = CircuitBreakers::new();
    
    if !manifest.packages.scoop.is_empty() {
        let scoop_results = apply_package_manager(
            Box::new(ScoopManager::new()),
            &manifest.packages.scoop,
            dry_run,
            force,
            &mut circuit_breakers,
        ).await?;
        results.extend(scoop_results);
    }
    
    if !manifest.packages.chocolatey.is_empty() {
        let choco_results = apply_package_manager(
            Box::new(ChocolateyManager::new()),
            &manifest.packages.chocolatey,
            dry_run,
            force,
            &mut circuit_breakers,
        ).await?;
        results.extend(choco_results);
    }
    
    if !manifest.packages.winget.is_empty() {
        let winget_results = apply_package_manager(
            Box::new(WingetManager::new()),
            &manifest.packages.winget,
            dry_run,
            force,
            &mut circuit_breakers,
        ).await?;
        results.extend(winget_results);
    }
    
    let apply_result = ApplyResult {
        total_packages: manifest.total_packages(),
        succeeded: results.iter().filter(|r| r.success).count(),
        failed: results.iter().filter(|r| !r.success && !r.skipped).count(),
        skipped: results.iter().filter(|r| r.skipped).count(),
        duration: start.elapsed(),
        dry_run,
        details: results,
    };
    
    print_summary(&apply_result);
    
    if apply_result.failed > 0 && !force {
        anyhow::bail!("{} packages failed to install", apply_result.failed);
    }
    
    Ok(())
}

async fn apply_package_manager(
    pm: Box<dyn PackageManager>,
    config: &PackageManagerConfig,
    dry_run: bool,
    force: bool,
    circuit_breakers: &mut CircuitBreakers,
) -> Result<Vec<OperationResult>> {
    let pm_type = pm.pm_type();
    info!("Processing {} packages", pm_type);
    
    if circuit_breakers.is_triggered(&pm_type) {
        warn!("Circuit breaker triggered for {} - skipping", pm_type);
        return Ok(config.packages.iter().map(|pkg| OperationResult {
            package_name: pkg.name.clone(),
            pm_type: pm_type.clone(),
            operation: "install".to_string(),
            success: false,
            skipped: true,
            message: Some("Circuit breaker triggered".to_string()),
            duration: std::time::Duration::ZERO,
        }).collect());
    }
    
    if !pm.is_available().await {
        error!("{} is not installed or not available", pm_type);
        if !force {
            anyhow::bail!("{} not available", pm_type);
        }
        return Ok(config.packages.iter().map(|pkg| OperationResult {
            package_name: pkg.name.clone(),
            pm_type: pm_type.clone(),
            operation: "install".to_string(),
            success: false,
            skipped: true,
            message: Some(format!("{} not available", pm_type)),
            duration: std::time::Duration::ZERO,
        }).collect());
    }
    
    let mut results = Vec::new();
    
    for package in &config.packages {
        if circuit_breakers.is_triggered(&pm_type) {
            warn!("Circuit breaker triggered for {} - skipping remaining packages", pm_type);
            results.push(OperationResult {
                package_name: package.name.clone(),
                pm_type: pm_type.clone(),
                operation: "install".to_string(),
                success: false,
                skipped: true,
                message: Some("Circuit breaker triggered".to_string()),
                duration: std::time::Duration::ZERO,
            });
            continue;
        }
        
        let options = InstallOptions {
            version: package.version.clone(),
            bucket: package.bucket.clone(),
            source: package.source.clone(),
            args: package.args.clone(),
            dry_run,
            force,
        };
        
        info!("Installing {} via {}", package.name, pm_type);
        let start = Instant::now();
        
        match pm.install(&package.name, options).await {
            Ok(()) => {
                let duration = start.elapsed();
                info!("✓ Successfully installed {} in {:?}", package.name, duration);
                circuit_breakers.reset(&pm_type);
                results.push(OperationResult {
                    package_name: package.name.clone(),
                    pm_type: pm_type.clone(),
                    operation: "install".to_string(),
                    success: true,
                    skipped: false,
                    message: None,
                    duration,
                });
            }
            Err(e) => {
                let duration = start.elapsed();
                error!("✗ Failed to install {}: {}", package.name, e);
                circuit_breakers.record_failure(&pm_type);
                results.push(OperationResult {
                    package_name: package.name.clone(),
                    pm_type: pm_type.clone(),
                    operation: "install".to_string(),
                    success: false,
                    skipped: false,
                    message: Some(e.to_string()),
                    duration,
                });
                
                if !force {
                    break;
                }
            }
        }
    }
    
    if config.prune && pm.supports_prune() {
        info!("Pruning unmanaged packages for {}", pm_type);
        if let Err(e) = pm.prune(&config.packages.iter().map(|p| p.name.clone()).collect::<Vec<_>>(), dry_run).await {
            error!("Failed to prune {}: {}", pm_type, e);
        }
    }
    
    Ok(results)
}

struct CircuitBreakers {
    failures: std::collections::HashMap<String, usize>,
    threshold: usize,
}

impl CircuitBreakers {
    fn new() -> Self {
        Self {
            failures: std::collections::HashMap::new(),
            threshold: 3,
        }
    }
    
    fn record_failure(&mut self, pm_type: &str) {
        *self.failures.entry(pm_type.to_string()).or_insert(0) += 1;
        debug!("Failure count for {}: {}", pm_type, self.failures[pm_type]);
    }
    
    fn reset(&mut self, pm_type: &str) {
        self.failures.remove(pm_type);
        debug!("Reset failure count for {}", pm_type);
    }
    
    fn is_triggered(&self, pm_type: &str) -> bool {
        self.failures.get(pm_type).copied().unwrap_or(0) >= self.threshold
    }
}

fn print_summary(result: &ApplyResult) {
    println!("\n╔══════════════════════════════════════╗");
    println!("║          OPERATION SUMMARY           ║");
    println!("╚══════════════════════════════════════╝");
    
    if result.dry_run {
        println!("  Mode: DRY RUN (no changes made)");
    }
    
    println!("  Total packages: {}", result.total_packages);
    println!("  ✓ Succeeded: {}", result.succeeded);
    
    if result.failed > 0 {
        println!("  ✗ Failed: {}", result.failed);
    }
    
    if result.skipped > 0 {
        println!("  ⊘ Skipped: {}", result.skipped);
    }
    
    println!("  Duration: {:.2}s", result.duration.as_secs_f64());
    
    if result.failed > 0 {
        println!("\nFailed packages:");
        for detail in &result.details {
            if !detail.success && !detail.skipped {
                println!("  - {} ({}): {}", 
                    detail.package_name, 
                    detail.pm_type,
                    detail.message.as_ref().unwrap_or(&"Unknown error".to_string())
                );
            }
        }
    }
}