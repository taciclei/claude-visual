//! Parser state management

use super::types::TextStyle;

/// ANSI escape code parser
pub struct AnsiParser {
    /// Current text style
    pub(crate) current_style: TextStyle,
    /// Buffer for incomplete sequences
    pub(crate) buffer: String,
    /// In escape sequence
    pub(crate) in_escape: bool,
    /// In OSC sequence
    pub(crate) in_osc: bool,
    /// OSC buffer
    pub(crate) osc_buffer: String,
}

impl AnsiParser {
    /// Create a new parser
    pub fn new() -> Self {
        Self {
            current_style: TextStyle::default(),
            buffer: String::new(),
            in_escape: false,
            in_osc: false,
            osc_buffer: String::new(),
        }
    }

    /// Get current style
    pub fn current_style(&self) -> &TextStyle {
        &self.current_style
    }

    /// Reset parser state
    pub fn reset(&mut self) {
        self.current_style = TextStyle::default();
        self.buffer.clear();
        self.in_escape = false;
        self.in_osc = false;
        self.osc_buffer.clear();
    }
}

impl Default for AnsiParser {
    fn default() -> Self {
        Self::new()
    }
}
