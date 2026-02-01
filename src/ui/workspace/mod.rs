//! Main workspace layout module
//!
//! This module contains the workspace implementation split into logical components:
//! - `types`: Common types like SidebarTab
//! - `core`: Core Workspace struct and main methods
//! - `actions`: Action handler implementations
//! - `modals`: Modal and overlay management
//! - `diff`: Diff preview functionality
//! - `messaging`: Claude messaging and streaming
//! - `split`: Split view functionality
//! - `render`: Render implementation (modular)

mod types;
mod core;
mod actions;
mod modals;
mod diff;
mod messaging;
mod split;
mod render;

// Re-export the main types
pub use types::SidebarTab;
pub use core::Workspace;
