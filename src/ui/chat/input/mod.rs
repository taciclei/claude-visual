//! Chat input component

use std::sync::Arc;

use gpui::*;

use crate::ai::mention::{Mention, PartialMention};
use crate::app::state::AppState;
use crate::ui::vim::VimState;

// Module declarations
mod autocomplete;
mod file_autocomplete;
mod file_drop;
mod history;
mod key_handling;
mod mentions;
mod render;
mod text_ops;
mod types;
mod vim;

// Rendering modules
mod command_dropdown;
mod context_chips;
mod drag_overlay;
mod file_dropdown;
mod footer;
mod input_area;
mod send_button;
mod templates_dropdown;
mod toolbar;
mod utils;
mod vim_indicator;

// Re-exports
pub use types::ChatInputEvent;

/// Multi-line chat input with send button
pub struct ChatInput {
    pub(crate) app_state: Arc<AppState>,
    pub(crate) text: String,
    pub(crate) is_focused: bool,
    pub(crate) focus_handle: FocusHandle,
    /// Whether input is disabled (during streaming)
    pub(crate) is_disabled: bool,
    /// Parsed mentions in the current text
    pub(crate) mentions: Vec<Mention>,
    /// Partial mention being typed (for autocomplete)
    pub(crate) partial_mention: Option<PartialMention>,
    /// Cursor position in text
    pub(crate) cursor_position: usize,
    /// Vim state for vim mode integration
    pub(crate) vim_state: Option<Entity<VimState>>,
    /// Whether a file is being dragged over
    pub(crate) is_drag_over: bool,
    /// Available slash commands (from Claude CLI)
    pub(crate) available_commands: Vec<String>,
    /// Show slash command autocomplete
    pub(crate) show_command_autocomplete: bool,
    /// Selected command index in autocomplete
    pub(crate) selected_command_index: usize,
    /// Filtered commands matching current input
    pub(crate) filtered_commands: Vec<String>,
    /// Fuzzy match data for filtered commands (parallel to filtered_commands)
    pub(crate) command_matches: Vec<utils::CommandMatch>,
    /// Show file mention autocomplete
    pub(crate) show_file_autocomplete: bool,
    /// Selected file index in autocomplete
    pub(crate) selected_file_index: usize,
    /// Filtered files matching current mention prefix
    pub(crate) filtered_files: Vec<utils::FileMatch>,
    /// Number of files being dragged (for overlay)
    pub(crate) drag_file_count: usize,
    /// Input history (previous prompts)
    pub(crate) input_history: Vec<String>,
    /// Current position in input history (-1 = current input, 0 = most recent, etc.)
    pub(crate) history_index: Option<usize>,
    /// Saved current input when navigating history
    pub(crate) saved_current_input: Option<String>,
    /// Whether think mode is enabled (synced from ChatView)
    pub(crate) think_mode_enabled: bool,
    /// Show prompt templates dropdown
    pub(crate) show_templates: bool,
}

impl ChatInput {
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        // Create vim state if vim mode is enabled in settings
        let vim_enabled = app_state.settings.read(cx).editor.vim_mode;
        let vim_state = if vim_enabled {
            let state = app_state.clone();
            Some(cx.new(|cx| {
                let mut vim = VimState::new(state, cx);
                vim.enable(cx);
                vim
            }))
        } else {
            None
        };

        Self {
            app_state,
            text: String::new(),
            is_focused: false,
            focus_handle: cx.focus_handle(),
            is_disabled: false,
            mentions: Vec::new(),
            partial_mention: None,
            cursor_position: 0,
            vim_state,
            is_drag_over: false,
            available_commands: Vec::new(),
            show_command_autocomplete: false,
            selected_command_index: 0,
            filtered_commands: Vec::new(),
            command_matches: Vec::new(),
            show_file_autocomplete: false,
            selected_file_index: 0,
            filtered_files: Vec::new(),
            drag_file_count: 0,
            input_history: Vec::new(),
            history_index: None,
            saved_current_input: None,
            think_mode_enabled: false,
            show_templates: false,
        }
    }

    /// Set think mode state (called by ChatView to sync state)
    pub fn set_think_mode(&mut self, enabled: bool, cx: &mut Context<Self>) {
        self.think_mode_enabled = enabled;
        cx.notify();
    }

    /// Toggle templates dropdown
    pub fn toggle_templates(&mut self, cx: &mut Context<Self>) {
        self.show_templates = !self.show_templates;
        cx.notify();
    }

    /// Insert a template into the input
    pub fn insert_template(&mut self, template: &str, cx: &mut Context<Self>) {
        self.text = template.to_string();
        self.cursor_position = self.text.len();
        self.show_templates = false;
        cx.notify();
    }

    /// Set the disabled state
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.is_disabled = disabled;
        cx.notify();
    }

    /// Check if input is disabled
    pub fn is_disabled(&self) -> bool {
        self.is_disabled
    }

    /// Get the current input text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Set the input text
    pub fn set_text(&mut self, text: String, cx: &mut Context<Self>) {
        self.text = text;
        self.cursor_position = self.text.len();
        self.update_mentions();
        cx.notify();
    }

    /// Clear the input
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.text.clear();
        self.mentions.clear();
        self.partial_mention = None;
        self.cursor_position = 0;
        self.show_command_autocomplete = false;
        self.filtered_commands.clear();
        self.show_file_autocomplete = false;
        self.filtered_files.clear();
        cx.notify();
    }

    /// Insert text at cursor position
    pub fn insert_text(&mut self, text: &str, cx: &mut Context<Self>) {
        // Add a space before if there's text and cursor is not at start
        if self.cursor_position > 0 && !self.text.is_empty() {
            let char_before = self
                .text
                .chars()
                .nth(self.cursor_position.saturating_sub(1));
            if char_before != Some(' ') {
                self.text.insert(self.cursor_position, ' ');
                self.cursor_position += 1;
            }
        }
        // Insert the text
        self.text.insert_str(self.cursor_position, text);
        self.cursor_position += text.len();
        // Add a space after
        self.text.insert(self.cursor_position, ' ');
        self.cursor_position += 1;
        self.update_mentions();
        cx.notify();
    }
}

impl EventEmitter<ChatInputEvent> for ChatInput {}

impl Focusable for ChatInput {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
