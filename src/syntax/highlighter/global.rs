//! Global syntax highlighter singleton

use std::sync::OnceLock;

use crate::app::theme::SyntaxColors;

use super::core::Highlighter;
use super::types::HighlightedSpan;

/// Global syntax highlighter instance
static HIGHLIGHTER: OnceLock<parking_lot::Mutex<Highlighter>> = OnceLock::new();

/// Get or create the global highlighter
pub struct SyntaxHighlighter;

impl SyntaxHighlighter {
    /// Highlight a line of code
    pub fn highlight(
        code: &str,
        language: Option<&str>,
        syntax_colors: &SyntaxColors,
    ) -> Vec<HighlightedSpan> {
        let highlighter = HIGHLIGHTER.get_or_init(|| parking_lot::Mutex::new(Highlighter::new()));
        highlighter
            .lock()
            .highlight_line(code, language, syntax_colors)
    }
}
