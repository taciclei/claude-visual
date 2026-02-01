//! Watch view events

use gpui::*;

use super::WatchView;

/// Events from watch view
#[derive(Debug, Clone)]
pub enum WatchViewEvent {
    /// Add a new watch expression
    Add(String),
    /// Remove a watch expression
    Remove(usize),
    /// Edit a watch expression
    Edit { id: usize, expression: String },
    /// Refresh all watches
    RefreshAll,
    /// Refresh a specific watch
    Refresh(usize),
    /// Expand a watch to show children
    Expand(usize),
    /// Collapse a watch
    Collapse(usize),
    /// Copy value to clipboard
    CopyValue(usize),
}

impl EventEmitter<WatchViewEvent> for WatchView {}
