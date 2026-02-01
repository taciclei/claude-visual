//! Scrolling functionality

use super::core::TerminalView;

impl TerminalView {
    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        if self.lines.len() > self.size.1 as usize {
            self.scroll_offset = self.lines.len() - self.size.1 as usize;
        }
    }

    /// Scroll up
    pub fn scroll_up(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
        self.auto_scroll = false;
    }

    /// Scroll down
    pub fn scroll_down(&mut self, lines: usize) {
        let max_offset = self.lines.len().saturating_sub(self.size.1 as usize);
        self.scroll_offset = (self.scroll_offset + lines).min(max_offset);

        // Re-enable auto-scroll if at bottom
        if self.scroll_offset == max_offset {
            self.auto_scroll = true;
        }
    }
}
