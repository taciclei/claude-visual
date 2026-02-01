//! Dropdown state and struct definition

use std::sync::Arc;
use gpui::*;

use crate::app::state::AppState;
use super::types::*;

/// Dropdown/Select component
pub struct Dropdown {
    pub(super) app_state: Arc<AppState>,
    /// Available options
    pub(super) options: Vec<DropdownOption>,
    /// Currently selected option ID
    pub(super) selected: Option<String>,
    /// Whether dropdown is open
    pub(super) is_open: bool,
    /// Size variant
    pub(super) size: DropdownSize,
    /// Placeholder text when nothing selected
    pub(super) placeholder: String,
    /// Whether dropdown is disabled
    pub(super) disabled: bool,
    /// Whether dropdown is searchable
    pub(super) searchable: bool,
    /// Search query
    pub(super) search_query: String,
    /// Label/title above dropdown
    pub(super) label: Option<String>,
    /// Error message
    pub(super) error: Option<String>,
}

impl EventEmitter<DropdownEvent> for Dropdown {}
