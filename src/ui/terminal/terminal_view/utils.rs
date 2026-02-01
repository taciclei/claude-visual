//! Utility methods

use super::core::TerminalView;
use super::types::TerminalLine;
use gpui::*;

impl TerminalView {
    /// Capture recent output for AI context
    pub fn capture_output(&self, lines: usize) -> String {
        let start = self.lines.len().saturating_sub(lines);
        self.lines[start..]
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|s| s.text.as_str())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Clear terminal
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.lines.clear();
        self.current_line = TerminalLine { spans: Vec::new() };
        self.parser.reset();
        self.scroll_offset = 0;
        cx.notify();
    }

    /// Resize terminal
    pub fn resize(&mut self, cols: u16, rows: u16, _cx: &mut Context<Self>) {
        self.size = (cols, rows);
        if let Some(pty) = &mut self.pty {
            pty.resize(cols, rows);
        }
    }
}
