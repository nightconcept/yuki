pub mod scoop;
pub mod chocolatey;
pub mod winget;

pub use scoop::ScoopManager;
pub use chocolatey::ChocolateyManager;
pub use winget::WingetManager;