//! Extensions UI module
//!
//! Provides UI components for browsing, installing, and managing extensions.

mod panel;
mod theme_editor;

pub use panel::ExtensionsPanel;
pub use theme_editor::{ThemeEditor, ThemeEditorEvent};
