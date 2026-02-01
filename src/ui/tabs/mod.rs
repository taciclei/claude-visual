//! Tab bar for multi-conversation support
//!
//! Provides a tab bar UI for managing multiple conversation tabs,
//! with support for keyboard navigation and drag-to-reorder.

mod drag_support;
mod overflow_menu;
mod pin_management;
mod render;
mod tab_bar;
mod tabs_management;
mod types;

pub use tab_bar::*;
pub use types::*;
