use thiserror::Error;

#[derive(Error, Debug)]
pub enum YukiError {
    #[error("Package manager '{0}' not found")]
    PackageManagerNotFound(String),
    
    #[error("Package '{package}' failed to install via {pm}: {reason}")]
    InstallFailed {
        package: String,
        pm: String,
        reason: String,
    },
    
    #[error("Circuit breaker triggered for {0} after {1} consecutive failures")]
    CircuitBreakerTriggered(String, usize),
    
    #[error("Manifest validation failed: {0}")]
    ManifestValidation(String),
    
    #[error("Operation requires elevated permissions")]
    ElevationRequired,
    
    #[error("Dry run - no changes made")]
    DryRun,
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),
}