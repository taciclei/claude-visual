//! Event generation from ANSI input

use super::state::AnsiParser;
use super::types::AnsiEvent;

impl AnsiParser {
    /// Parse input and return events
    pub fn parse(&mut self, input: &str) -> Vec<AnsiEvent> {
        let mut events = Vec::new();
        let mut text_buffer = String::new();

        for ch in input.chars() {
            if self.in_osc {
                if ch == '\x07' || ch == '\x1b' {
                    // End of OSC sequence
                    self.in_osc = false;
                    if self.osc_buffer.starts_with("0;") || self.osc_buffer.starts_with("2;") {
                        let title = self.osc_buffer[2..].to_string();
                        events.push(AnsiEvent::SetTitle(title));
                    }
                    self.osc_buffer.clear();
                } else {
                    self.osc_buffer.push(ch);
                }
                continue;
            }

            if self.in_escape {
                self.buffer.push(ch);

                // Check for complete sequence
                if let Some(event) = self.try_parse_escape() {
                    // Flush text buffer first
                    if !text_buffer.is_empty() {
                        events.push(AnsiEvent::Text(std::mem::take(&mut text_buffer)));
                    }
                    events.push(event);
                    self.buffer.clear();
                    self.in_escape = false;
                } else if self.buffer.len() > 20 {
                    // Sequence too long, abort
                    self.buffer.clear();
                    self.in_escape = false;
                }
                continue;
            }

            match ch {
                '\x1b' => {
                    self.in_escape = true;
                    self.buffer.clear();
                }
                '\x07' => {
                    if !text_buffer.is_empty() {
                        events.push(AnsiEvent::Text(std::mem::take(&mut text_buffer)));
                    }
                    events.push(AnsiEvent::Bell);
                }
                '\n' => {
                    if !text_buffer.is_empty() {
                        events.push(AnsiEvent::Text(std::mem::take(&mut text_buffer)));
                    }
                    events.push(AnsiEvent::Newline);
                }
                '\r' => {
                    if !text_buffer.is_empty() {
                        events.push(AnsiEvent::Text(std::mem::take(&mut text_buffer)));
                    }
                    events.push(AnsiEvent::CarriageReturn);
                }
                '\t' => {
                    if !text_buffer.is_empty() {
                        events.push(AnsiEvent::Text(std::mem::take(&mut text_buffer)));
                    }
                    events.push(AnsiEvent::Tab);
                }
                '\x08' => {
                    if !text_buffer.is_empty() {
                        events.push(AnsiEvent::Text(std::mem::take(&mut text_buffer)));
                    }
                    events.push(AnsiEvent::Backspace);
                }
                _ if ch.is_control() => {
                    // Skip other control characters
                }
                _ => {
                    text_buffer.push(ch);
                }
            }
        }

        // Flush remaining text
        if !text_buffer.is_empty() {
            events.push(AnsiEvent::Text(text_buffer));
        }

        events
    }
}
