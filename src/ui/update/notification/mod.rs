//! Update Notification UI Component
//!
//! Displays a banner when a new version is available.

mod core;
mod types;

#[path = "render/mod.rs"]
mod render;

mod traits;

#[cfg(test)]
mod tests;

// Re-export public types
pub use core::UpdateNotification;
pub use types::UpdateNotificationEvent;
