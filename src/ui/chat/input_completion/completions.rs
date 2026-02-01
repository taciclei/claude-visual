//! Completion generation implementation

use super::manager::InputCompletionManager;
use super::types::*;
use super::utils::fuzzy_match;

impl InputCompletionManager {
    /// Handle text input
    pub fn handle_input(&mut self, text: &str, cursor_position: usize) -> bool {
        if !self.config.enabled {
            return false;
        }

        // Check for trigger character at cursor
        if cursor_position > 0 && cursor_position <= text.len() {
            let before_cursor = &text[..cursor_position];

            // Look for trigger
            for (i, ch) in before_cursor.char_indices().rev() {
                if let Some(trigger) = CompletionTrigger::from_char(ch) {
                    // Check if this is a valid trigger position
                    if i == 0 || before_cursor[..i].ends_with(char::is_whitespace) {
                        self.trigger = Some(trigger);
                        self.trigger_position = Some(i);
                        self.query = before_cursor[i + ch.len_utf8()..].to_string();

                        if self.query.len() >= self.config.min_chars {
                            self.update_completions();
                            return true;
                        }
                    }
                    break;
                }

                // Stop looking after whitespace
                if ch.is_whitespace() {
                    break;
                }
            }
        }

        // No trigger found
        if self.state != CompletionState::Inactive {
            self.dismiss();
        }
        false
    }

    /// Update completions based on current trigger and query
    pub(crate) fn update_completions(&mut self) {
        self.items.clear();
        self.selected_index = 0;

        match self.trigger {
            Some(CompletionTrigger::Command) => {
                if self.config.show_commands {
                    self.add_command_completions();
                }
            }
            Some(CompletionTrigger::Mention) => {
                self.add_mention_completions();
            }
            Some(CompletionTrigger::FilePath) => {
                if self.config.show_files {
                    self.add_file_completions();
                }
            }
            _ => {}
        }

        // Sort by relevance
        let query = self.query.clone();
        self.items.sort_by(|a, b| {
            let score_a = a.match_score(&query).unwrap_or(i32::MIN);
            let score_b = b.match_score(&query).unwrap_or(i32::MIN);
            score_b
                .cmp(&score_a)
                .then_with(|| a.kind.sort_priority().cmp(&b.kind.sort_priority()))
        });

        // Limit items
        self.items.truncate(self.config.max_items);

        self.state = if self.items.is_empty() {
            CompletionState::Inactive
        } else {
            CompletionState::Active
        };
    }

    /// Add command completions
    pub(crate) fn add_command_completions(&mut self) {
        for (name, description) in &self.commands {
            if name.starts_with(&self.query) || fuzzy_match(name, &self.query).is_some() {
                self.items
                    .push(ChatCompletionItem::command(name, description));
            }
        }
    }

    /// Add mention completions
    pub(crate) fn add_mention_completions(&mut self) {
        // File mentions
        if self.config.show_files {
            for path in &self.recent_files {
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();

                if name.starts_with(&self.query) || fuzzy_match(&name, &self.query).is_some() {
                    self.items.push(ChatCompletionItem::file(path.clone()));
                }
            }
        }

        // Built-in mentions
        let mention_types = [
            ("file", "Reference a file"),
            ("snippet", "Reference a code snippet"),
            ("url", "Reference a URL"),
            ("symbol", "Reference a symbol"),
        ];

        for (type_name, desc) in mention_types {
            if type_name.starts_with(&self.query) {
                self.items
                    .push(ChatCompletionItem::mention(type_name, "", Some(desc)));
            }
        }
    }

    /// Add file path completions
    pub(crate) fn add_file_completions(&mut self) {
        // Add recent files that match
        for path in &self.recent_files {
            let display = path.display().to_string();
            if display.contains(&self.query) {
                self.items.push(ChatCompletionItem::file(path.clone()));
            }
        }
    }
}
