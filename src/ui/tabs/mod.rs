//! Tab bar for multi-conversation support
//!
//! Provides a tab bar UI for managing multiple conversation tabs,
//! with support for keyboard navigation and drag-to-reorder.

mod types;
mod tab_bar;
mod tabs_management;
mod pin_management;
mod drag_support;
mod overflow_menu;
mod render;

pub use types::*;
pub use tab_bar::*;
