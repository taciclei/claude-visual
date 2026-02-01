//! Input and event handlers

use super::core::TerminalView;
use super::types::*;
use gpui::*;

impl TerminalView {
    /// Send input to terminal
    pub fn send_input(&mut self, input: &str, cx: &mut Context<Self>) {
        if !self.is_running {
            return;
        }

        // Save to history if it's a command (ends with newline)
        if input.ends_with('\n') {
            let cmd = input.trim_end().to_string();
            if !cmd.is_empty() {
                self.command_history.push(cmd.clone());
                self.history_index = None;
                cx.emit(TerminalViewEvent::CommandExecuted(cmd));
            }
        }

        // Send to PTY
        if let Some(_pty) = &mut self.pty {
            // TODO: Would need to make pty accessible for async writes
        }
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        match &key.keystroke.key {
            key if key == "enter" => {
                let input = std::mem::take(&mut self.input_buffer);
                self.send_input(&format!("{}\n", input), cx);
            }
            key if key == "backspace" => {
                self.input_buffer.pop();
                cx.notify();
            }
            key if key == "up" => {
                // History navigation
                if !self.command_history.is_empty() {
                    let new_index = match self.history_index {
                        None => self.command_history.len() - 1,
                        Some(0) => 0,
                        Some(i) => i - 1,
                    };
                    self.history_index = Some(new_index);
                    self.input_buffer = self.command_history[new_index].clone();
                    cx.notify();
                }
            }
            key if key == "down" => {
                if let Some(index) = self.history_index {
                    if index + 1 < self.command_history.len() {
                        self.history_index = Some(index + 1);
                        self.input_buffer = self.command_history[index + 1].clone();
                    } else {
                        self.history_index = None;
                        self.input_buffer.clear();
                    }
                    cx.notify();
                }
            }
            key if key == "c" && key.contains("ctrl") => {
                // Ctrl+C - interrupt
                self.send_input("\x03", cx);
            }
            key if key == "d" && key.contains("ctrl") => {
                // Ctrl+D - EOF
                self.send_input("\x04", cx);
            }
            key if key == "l" && key.contains("ctrl") => {
                // Ctrl+L - clear
                self.lines.clear();
                self.current_line = TerminalLine { spans: Vec::new() };
                cx.notify();
            }
            _ => {
                // Regular character input
                if key.keystroke.key.len() == 1 {
                    self.input_buffer.push_str(&key.keystroke.key);
                    cx.notify();
                }
            }
        }
    }
}
