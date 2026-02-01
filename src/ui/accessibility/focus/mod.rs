//! Focus management for keyboard navigation
//!
//! Provides a focus ring system for accessible keyboard navigation.

mod manager;
mod types;

#[cfg(test)]
mod tests;

// Re-export public API
pub use manager::FocusManager;
pub use types::{FocusRingStyle, FocusZone, FocusTrap, FocusableElement};
