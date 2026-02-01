//! UI module
//!
//! Contains all user interface components built with GPUI.

use gpui::{relative, div, Div, DefiniteLength};

/// Helper function to create a percentage-based length.
/// Takes a percentage (0-100) and converts to a relative fraction.
pub fn pct(percentage: f32) -> DefiniteLength {
    relative(percentage / 100.0)
}

/// Helper function to create an inline span element.
/// This is a wrapper around div() for inline text styling.
pub fn span() -> Div {
    div()
}

pub mod accessibility;
pub mod agent;
pub mod ai;
pub mod blocks;
pub mod chat;
pub mod cloud;
pub mod components;
pub mod debug;
pub mod diff;
pub mod explorer;
pub mod extensions;
pub mod lsp;
pub mod mcp;
pub mod settings;
pub mod sidebar;
pub mod split;
pub mod tabs;
pub mod terminal;
pub mod update;
pub mod vim;
pub mod workspace;
