//! Text selection functionality

use super::core::TerminalView;

impl TerminalView {
    /// Get selected text
    pub fn selected_text(&self) -> Option<String> {
        let (start, end) = match (self.selection_start, self.selection_end) {
            (Some(s), Some(e)) => {
                if s.0 < e.0 || (s.0 == e.0 && s.1 < e.1) {
                    (s, e)
                } else {
                    (e, s)
                }
            }
            _ => return None,
        };

        let mut text = String::new();
        for row in start.0..=end.0 {
            if let Some(line) = self.lines.get(row) {
                let line_text: String = line.spans.iter().map(|s| s.text.as_str()).collect();
                let start_col = if row == start.0 { start.1 } else { 0 };
                let end_col = if row == end.0 {
                    end.1
                } else {
                    line_text.len()
                };

                if start_col < line_text.len() {
                    let actual_end = end_col.min(line_text.len());
                    text.push_str(&line_text[start_col..actual_end]);
                }

                if row < end.0 {
                    text.push('\n');
                }
            }
        }

        Some(text)
    }
}
