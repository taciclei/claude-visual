//! Command palette and keyboard event handlers

use gpui::*;
use crate::ui::chat::view::core::ChatView;
use crate::ui::chat::view::types::PaletteCommand;

impl ChatView {
    /// Handle key events for command palette
    pub fn handle_palette_key(&mut self, key: &str, cx: &mut Context<Self>) -> bool {
        if !self.panels.command_palette {
            return false;
        }

        let commands = Self::get_palette_commands();
        let query = self.palette.query.to_lowercase();

        let filtered_count = if query.is_empty() {
            commands.len()
        } else {
            commands.iter().filter(|cmd| {
                cmd.label.to_lowercase().contains(&query) ||
                cmd.description.to_lowercase().contains(&query) ||
                cmd.category.to_lowercase().contains(&query)
            }).count()
        };

        match key {
            "escape" => {
                self.panels.command_palette = false;
                self.palette.query.clear();
                cx.notify();
                true
            }
            "up" => {
                if self.palette.selected_index > 0 {
                    self.palette.selected_index -= 1;
                } else if filtered_count > 0 {
                    self.palette.selected_index = filtered_count - 1;
                }
                cx.notify();
                true
            }
            "down" => {
                if self.palette.selected_index + 1 < filtered_count {
                    self.palette.selected_index += 1;
                } else {
                    self.palette.selected_index = 0;
                }
                cx.notify();
                true
            }
            "enter" => {
                let filtered: Vec<&PaletteCommand> = if query.is_empty() {
                    commands.iter().collect()
                } else {
                    commands.iter().filter(|cmd| {
                        cmd.label.to_lowercase().contains(&query) ||
                        cmd.description.to_lowercase().contains(&query) ||
                        cmd.category.to_lowercase().contains(&query)
                    }).collect()
                };

                if let Some(cmd) = filtered.get(self.palette.selected_index) {
                    let cmd_id = cmd.id;
                    self.execute_palette_command(cmd_id, cx);
                }
                true
            }
            "backspace" => {
                self.palette.query.pop();
                self.palette.selected_index = 0;
                cx.notify();
                true
            }
            _ if key.len() == 1 => {
                self.palette.query.push_str(key);
                self.palette.selected_index = 0;
                cx.notify();
                true
            }
            _ => false
        }
    }

    /// Type a character into the palette search
    pub fn palette_type_char(&mut self, ch: char, cx: &mut Context<Self>) {
        if self.panels.command_palette {
            self.palette.query.push(ch);
            self.palette.selected_index = 0;
            cx.notify();
        }
    }

    /// Delete a character from the palette search
    pub fn palette_backspace(&mut self, cx: &mut Context<Self>) {
        if self.panels.command_palette {
            self.palette.query.pop();
            self.palette.selected_index = 0;
            cx.notify();
        }
    }

    /// Navigate up in the palette
    pub fn palette_select_prev(&mut self, cx: &mut Context<Self>) {
        if !self.panels.command_palette {
            return;
        }
        let commands = Self::get_palette_commands();
        let query = self.palette.query.to_lowercase();
        let filtered_count = if query.is_empty() {
            commands.len()
        } else {
            commands.iter().filter(|cmd| {
                cmd.label.to_lowercase().contains(&query) ||
                cmd.description.to_lowercase().contains(&query)
            }).count()
        };

        if self.palette.selected_index > 0 {
            self.palette.selected_index -= 1;
        } else if filtered_count > 0 {
            self.palette.selected_index = filtered_count - 1;
        }
        cx.notify();
    }

    /// Navigate down in the palette
    pub fn palette_select_next(&mut self, cx: &mut Context<Self>) {
        if !self.panels.command_palette {
            return;
        }
        let commands = Self::get_palette_commands();
        let query = self.palette.query.to_lowercase();
        let filtered_count = if query.is_empty() {
            commands.len()
        } else {
            commands.iter().filter(|cmd| {
                cmd.label.to_lowercase().contains(&query) ||
                cmd.description.to_lowercase().contains(&query)
            }).count()
        };

        if self.palette.selected_index + 1 < filtered_count {
            self.palette.selected_index += 1;
        } else {
            self.palette.selected_index = 0;
        }
        cx.notify();
    }

    /// Execute selected palette command
    pub fn palette_execute_selected(&mut self, cx: &mut Context<Self>) {
        if !self.panels.command_palette {
            return;
        }
        let commands = Self::get_palette_commands();
        let query = self.palette.query.to_lowercase();
        let filtered: Vec<&PaletteCommand> = if query.is_empty() {
            commands.iter().collect()
        } else {
            commands.iter().filter(|cmd| {
                cmd.label.to_lowercase().contains(&query) ||
                cmd.description.to_lowercase().contains(&query)
            }).collect()
        };

        if let Some(cmd) = filtered.get(self.palette.selected_index) {
            let cmd_id = cmd.id;
            self.execute_palette_command(cmd_id, cx);
        }
    }

    /// Handle key events during title edit
    pub(crate) fn handle_title_key_down(&mut self, event: &KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        if !self.editing_title {
            return;
        }

        match &event.keystroke.key {
            key if key == "enter" => {
                self.save_edited_title(cx);
            }
            key if key == "escape" => {
                self.cancel_editing_title(cx);
            }
            key if key == "backspace" => {
                self.title_edit_buffer.pop();
                cx.notify();
            }
            _ => {}
        }
    }

    /// Handle text input during title edit
    pub(crate) fn handle_title_input(&mut self, text: &str, _window: &mut Window, cx: &mut Context<Self>) {
        if !self.editing_title {
            return;
        }

        // Add the input text to the buffer
        self.title_edit_buffer.push_str(text);
        cx.notify();
    }

    /// Toggle keyboard shortcuts help panel
    pub fn toggle_shortcuts_help(&mut self, cx: &mut Context<Self>) {
        self.panels.shortcuts_help = !self.panels.shortcuts_help;
        cx.notify();
    }

    /// Check if shortcuts help is visible
    pub fn is_shortcuts_help_visible(&self) -> bool {
        self.panels.shortcuts_help
    }
}
