//! Textarea state and constructors

use crate::app::state::AppState;
use crate::ui::components::textarea::types::*;
use gpui::*;
use std::sync::Arc;

/// Textarea component for multiline text input
pub struct Textarea {
    pub(crate) app_state: Arc<AppState>,
    /// Current text content
    pub(crate) text: String,
    /// Placeholder text
    pub(crate) placeholder: String,
    /// Number of visible rows
    pub(crate) rows: usize,
    /// Minimum rows (for auto-resize)
    pub(crate) min_rows: usize,
    /// Maximum rows (for auto-resize)
    pub(crate) max_rows: usize,
    /// Resize mode
    pub(crate) resize: TextareaResize,
    /// Whether textarea is disabled
    pub(crate) disabled: bool,
    /// Whether textarea is readonly
    pub(crate) readonly: bool,
    /// Label text
    pub(crate) label: Option<String>,
    /// Helper/hint text
    pub(crate) helper: Option<String>,
    /// Error message
    pub(crate) error: Option<String>,
    /// Character limit
    pub(crate) max_length: Option<usize>,
    /// Show character count
    pub(crate) show_count: bool,
    /// Whether textarea is focused
    pub(crate) focused: bool,
    /// Focus handle
    pub(crate) focus_handle: FocusHandle,
}

impl Textarea {
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            text: String::new(),
            placeholder: String::new(),
            rows: 4,
            min_rows: 2,
            max_rows: 10,
            resize: TextareaResize::default(),
            disabled: false,
            readonly: false,
            label: None,
            helper: None,
            error: None,
            max_length: None,
            show_count: false,
            focused: false,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Create with placeholder
    pub fn with_placeholder(
        app_state: Arc<AppState>,
        placeholder: impl Into<String>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut textarea = Self::new(app_state, cx);
        textarea.placeholder = placeholder.into();
        textarea
    }
}
