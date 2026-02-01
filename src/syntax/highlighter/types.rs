//! Type definitions for syntax highlighting

use gpui::Hsla;

/// A highlighted span of text
#[derive(Debug, Clone)]
pub struct HighlightedSpan {
    pub text: String,
    pub color: Option<Hsla>,
}
