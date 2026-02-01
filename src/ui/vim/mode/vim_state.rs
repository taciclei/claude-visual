//! Core VimState struct and constructor

use gpui::*;
use std::sync::Arc;

use crate::app::state::AppState;
use super::super::VimKeyHandler;
use super::VimMode;

/// Vim state manager
pub struct VimState {
    pub(super) app_state: Arc<AppState>,
    /// Current mode
    pub(super) mode: VimMode,
    /// Key handler for processing keystrokes
    pub(super) key_handler: VimKeyHandler,
    /// Command buffer for command mode
    pub(super) command_buffer: String,
    /// Search pattern for search mode
    pub(super) search_pattern: String,
    /// Last search pattern (for n/N navigation)
    pub(super) last_search: String,
    /// Cursor position (line, column)
    pub(super) cursor: (usize, usize),
    /// Visual selection start (if in visual mode)
    pub(super) visual_start: Option<(usize, usize)>,
    /// Focus handle
    pub(super) focus_handle: FocusHandle,
    /// Whether vim mode is enabled
    pub(super) enabled: bool,
    /// Count prefix (e.g., 5j moves down 5 lines)
    pub(super) count: Option<usize>,
    /// Pending operator (d, y, c, etc.)
    pub(super) pending_operator: Option<char>,
}

impl VimState {
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            mode: VimMode::Normal,
            key_handler: VimKeyHandler::new(),
            command_buffer: String::new(),
            search_pattern: String::new(),
            last_search: String::new(),
            cursor: (0, 0),
            visual_start: None,
            focus_handle: cx.focus_handle(),
            enabled: false,
            count: None,
            pending_operator: None,
        }
    }

    /// Get current mode
    pub fn mode(&self) -> VimMode {
        self.mode
    }

    /// Check if vim mode is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
