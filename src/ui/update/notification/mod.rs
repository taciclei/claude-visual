//! Update Notification UI Component
//!
//! Displays a banner when a new version is available.

mod types;
mod core;

#[path = "render/mod.rs"]
mod render;

mod traits;

#[cfg(test)]
mod tests;

// Re-export public types
pub use types::UpdateNotificationEvent;
pub use core::UpdateNotification;
