//! ANSI escape sequence parsing

use super::state::AnsiParser;
use super::types::AnsiEvent;

impl AnsiParser {
    /// Try to parse escape sequence
    pub(crate) fn try_parse_escape(&mut self) -> Option<AnsiEvent> {
        if self.buffer.is_empty() {
            return None;
        }

        let first = self.buffer.chars().next()?;

        match first {
            '[' => self.try_parse_csi(),
            ']' => {
                // Start of OSC sequence
                self.in_osc = true;
                self.osc_buffer = self.buffer[1..].to_string();
                self.buffer.clear();
                self.in_escape = false;
                None
            }
            '(' | ')' => {
                // Character set selection - ignore
                if self.buffer.len() >= 2 {
                    Some(AnsiEvent::Text(String::new()))
                } else {
                    None
                }
            }
            'c' => {
                // Reset
                self.current_style.reset();
                Some(AnsiEvent::ClearScreen)
            }
            _ => {
                // Unknown sequence
                Some(AnsiEvent::Text(String::new()))
            }
        }
    }

    /// Try to parse CSI sequence (ESC [)
    pub(crate) fn try_parse_csi(&mut self) -> Option<AnsiEvent> {
        let seq = &self.buffer[1..]; // Skip '['

        if seq.is_empty() {
            return None;
        }

        let last = seq.chars().last()?;

        // Check if sequence is complete
        if !last.is_ascii_alphabetic() && last != '@' && last != '`' {
            return None;
        }

        // Parse parameters
        let params_str = &seq[..seq.len() - 1];
        let params: Vec<u32> = if params_str.is_empty() {
            vec![]
        } else {
            params_str
                .split(';')
                .filter_map(|s| s.parse().ok())
                .collect()
        };

        match last {
            'm' => {
                // SGR (Select Graphic Rendition)
                self.apply_sgr_params(&params);
                Some(AnsiEvent::Style(self.current_style.clone()))
            }
            'H' | 'f' => {
                // Cursor position
                let row = params.first().copied().unwrap_or(1);
                let col = params.get(1).copied().unwrap_or(1);
                Some(AnsiEvent::CursorPosition { row, col })
            }
            'A' => {
                // Cursor up
                let n = params.first().copied().unwrap_or(1) as i32;
                Some(AnsiEvent::CursorMove { row: -n, col: 0 })
            }
            'B' => {
                // Cursor down
                let n = params.first().copied().unwrap_or(1) as i32;
                Some(AnsiEvent::CursorMove { row: n, col: 0 })
            }
            'C' => {
                // Cursor forward
                let n = params.first().copied().unwrap_or(1) as i32;
                Some(AnsiEvent::CursorMove { row: 0, col: n })
            }
            'D' => {
                // Cursor back
                let n = params.first().copied().unwrap_or(1) as i32;
                Some(AnsiEvent::CursorMove { row: 0, col: -n })
            }
            'J' => {
                // Erase in display
                Some(AnsiEvent::ClearScreen)
            }
            'K' => {
                // Erase in line
                Some(AnsiEvent::ClearLine)
            }
            _ => {
                // Unknown CSI sequence
                Some(AnsiEvent::Text(String::new()))
            }
        }
    }
}
