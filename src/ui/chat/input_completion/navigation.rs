//! Navigation and interaction implementation

use super::types::*;
use super::manager::InputCompletionManager;

impl InputCompletionManager {
    /// Move selection up
    pub fn select_prev(&mut self) {
        if !self.items.is_empty() {
            self.selected_index = self.selected_index
                .checked_sub(1)
                .unwrap_or(self.items.len() - 1);
        }
    }

    /// Move selection down
    pub fn select_next(&mut self) {
        if !self.items.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.items.len();
        }
    }

    /// Accept current selection
    pub fn accept(&mut self) -> Option<CompletionResult> {
        let item = self.selected_item()?.clone();
        let trigger_pos = self.trigger_position?;

        let result = CompletionResult {
            insert_text: item.insert_text.clone(),
            replace_range: (trigger_pos, trigger_pos + self.query.len() + 1),
            kind: item.kind,
        };

        self.dismiss();
        Some(result)
    }

    /// Dismiss completion
    pub fn dismiss(&mut self) {
        self.state = CompletionState::Inactive;
        self.trigger = None;
        self.trigger_position = None;
        self.query.clear();
        self.items.clear();
        self.selected_index = 0;
    }

    /// Trigger completion manually
    pub fn trigger_manual(&mut self, text: &str, cursor_position: usize) {
        self.trigger = Some(CompletionTrigger::Manual);
        self.trigger_position = Some(cursor_position);
        self.query.clear();
        self.state = CompletionState::Loading;

        // Add all completions
        if self.config.show_commands {
            self.add_command_completions();
        }
        if self.config.show_files {
            self.add_file_completions();
        }

        self.state = if self.items.is_empty() {
            CompletionState::Inactive
        } else {
            CompletionState::Active
        };
    }
}
