use anyhow::Result;
use tracing::info;
use yuki_pm::PackageManager;
use yuki_pm::traits::PackageInfo;
use yuki_pm::managers::{ChocolateyManager, ScoopManager, WingetManager};

pub async fn run(pm_filter: Option<String>, format: String) -> Result<()> {
    info!("Listing installed packages...");
    
    let mut all_packages = Vec::new();
    
    if pm_filter.as_deref() == Some("scoop") || pm_filter.is_none() {
        let scoop = ScoopManager::new();
        if scoop.is_available().await {
            match scoop.list_installed().await {
                Ok(packages) => {
                    info!("Found {} Scoop packages", packages.len());
                    all_packages.extend(packages);
                }
                Err(e) => {
                    tracing::warn!("Failed to list Scoop packages: {}", e);
                }
            }
        }
    }
    
    if pm_filter.as_deref() == Some("chocolatey") || pm_filter.is_none() {
        let choco = ChocolateyManager::new();
        if choco.is_available().await {
            match choco.list_installed().await {
                Ok(packages) => {
                    info!("Found {} Chocolatey packages", packages.len());
                    all_packages.extend(packages);
                }
                Err(e) => {
                    tracing::warn!("Failed to list Chocolatey packages: {}", e);
                }
            }
        }
    }
    
    if pm_filter.as_deref() == Some("winget") || pm_filter.is_none() {
        let winget = WingetManager::new();
        if winget.is_available().await {
            match winget.list_installed().await {
                Ok(packages) => {
                    info!("Found {} Winget packages", packages.len());
                    all_packages.extend(packages);
                }
                Err(e) => {
                    tracing::warn!("Failed to list Winget packages: {}", e);
                }
            }
        }
    }
    
    // Sort all packages alphabetically by name (case-insensitive)
    all_packages.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    
    // Remove duplicates - keep the first occurrence of each package name + version combo
    let mut seen = std::collections::HashSet::new();
    let deduplicated: Vec<PackageInfo> = all_packages
        .into_iter()
        .filter(|pkg| {
            // Create a key from name + version for deduplication
            let key = format!("{}:{}", pkg.name.to_lowercase(), pkg.version.to_lowercase());
            seen.insert(key)
        })
        .collect();
    
    match format.as_str() {
        "json" => print_json(&deduplicated),
        "yaml" => print_yaml(&deduplicated),
        _ => print_table(&deduplicated),
    }
    
    Ok(())
}

fn print_table(packages: &[PackageInfo]) {
    if packages.is_empty() {
        println!("No packages installed");
        return;
    }
    
    println!("\n{:<50} {:<20} {:<10} {:<15}", "Package", "Version", "Manager", "Source");
    println!("{}", "-".repeat(95));
    
    for pkg in packages {
        // Truncate long package names
        let name = if pkg.name.len() > 49 {
            format!("{}…", &pkg.name[..48])
        } else {
            pkg.name.clone()
        };
        
        // Truncate long versions  
        let version = if pkg.version.len() > 19 {
            format!("{}…", &pkg.version[..18])
        } else {
            pkg.version.clone()
        };
        
        println!(
            "{:<50} {:<20} {:<10} {:<15}",
            name,
            version,
            pkg.pm_type,
            pkg.source.as_deref().unwrap_or("-")
        );
    }
    
    println!("\nTotal: {} packages", packages.len());
}

fn print_json(packages: &[PackageInfo]) {
    match serde_json::to_string_pretty(packages) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Failed to serialize to JSON: {}", e),
    }
}

fn print_yaml(packages: &[PackageInfo]) {
    match serde_yaml::to_string(packages) {
        Ok(yaml) => println!("{}", yaml),
        Err(e) => eprintln!("Failed to serialize to YAML: {}", e),
    }
}