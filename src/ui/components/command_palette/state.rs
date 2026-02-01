//! Command palette state and logic

use std::sync::Arc;
use gpui::*;
use gpui::prelude::*;

use crate::app::state::AppState;
use super::types::*;
use super::fuzzy::{FuzzyMatch, fuzzy_search};

/// Maximum number of recent commands to track
const MAX_RECENT_COMMANDS: usize = 10;

/// Command palette state
pub struct CommandPalette {
    pub(crate) app_state: Arc<AppState>,
    pub(crate) query: String,
    pub(crate) selected_index: usize,
    pub(crate) focus_handle: FocusHandle,
    /// Recently used command IDs (most recent first)
    pub(crate) recent_commands: Vec<String>,
}

impl CommandPalette {
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            query: String::new(),
            selected_index: 0,
            focus_handle: cx.focus_handle(),
            recent_commands: Vec::new(),
        }
    }

    /// Get fuzzy-matched commands based on query
    pub(crate) fn fuzzy_matches(&self) -> Vec<FuzzyMatch<'_>> {
        fuzzy_search(&COMMANDS, &self.query, &self.recent_commands)
    }

    /// Get filtered commands based on query (legacy, returns just commands)
    pub(crate) fn filtered_commands(&self) -> Vec<&Command> {
        self.fuzzy_matches().into_iter().map(|m| m.command).collect()
    }

    /// Track a command as recently used
    pub(crate) fn track_recent_command(&mut self, command_id: &str) {
        // Remove if already present
        self.recent_commands.retain(|id| id != command_id);
        // Add to front
        self.recent_commands.insert(0, command_id.to_string());
        // Trim to max size
        self.recent_commands.truncate(MAX_RECENT_COMMANDS);
    }

    /// Handle key input
    pub(crate) fn handle_key_down(&mut self, event: &KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let filtered = self.filtered_commands();
        let count = filtered.len();

        match event.keystroke.key.as_str() {
            "escape" => {
                cx.emit(CommandPaletteEvent::Dismissed);
            }
            "enter" => {
                if let Some(cmd) = filtered.get(self.selected_index) {
                    cx.emit(CommandPaletteEvent::CommandSelected(cmd.id.to_string()));
                }
            }
            "up" => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                } else if count > 0 {
                    self.selected_index = count - 1;
                }
                cx.notify();
            }
            "down" => {
                if self.selected_index < count.saturating_sub(1) {
                    self.selected_index += 1;
                } else {
                    self.selected_index = 0;
                }
                cx.notify();
            }
            "backspace" => {
                self.query.pop();
                self.selected_index = 0;
                cx.notify();
            }
            _ => {}
        }
    }

    /// Handle text input
    pub(crate) fn handle_input(&mut self, text: &str, _window: &mut Window, cx: &mut Context<Self>) {
        self.query.push_str(text);
        self.selected_index = 0;
        cx.notify();
    }

    /// Select a command by index
    pub(crate) fn select_command(&mut self, index: usize, cx: &mut Context<Self>) {
        let matches = self.fuzzy_matches();
        if let Some(m) = matches.get(index) {
            let command_id = m.command.id.to_string();
            self.track_recent_command(&command_id);
            cx.emit(CommandPaletteEvent::CommandSelected(command_id));
        }
    }
}

impl EventEmitter<CommandPaletteEvent> for CommandPalette {}

impl Focusable for CommandPalette {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
