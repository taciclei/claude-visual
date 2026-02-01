//! Auto-Update Module
//!
//! Handles checking for updates and downloading new versions.

mod checker;
mod installer;

pub use checker::{UpdateChecker, UpdateInfo, UpdateStatus};
pub use installer::{InstallProgress, UpdateInstaller};
