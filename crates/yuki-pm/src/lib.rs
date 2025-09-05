pub mod traits;
pub mod managers;
pub mod utils;

pub use traits::{PackageManager, PackageManagerType, InstallOptions, OperationResult};
pub use managers::{ScoopManager, ChocolateyManager, WingetManager};