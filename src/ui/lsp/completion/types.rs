//! Completion types and color scheme

use gpui::*;
use crate::lsp::protocol::CompletionItem;

pub(super) struct SimpleColors {
    pub surface: Hsla,
    pub border: Hsla,
    pub hover: Hsla,
    pub selection: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub syntax: SyntaxColors,
}

pub(super) struct SyntaxColors {
    pub function: Hsla,
    pub variable: Hsla,
    pub type_name: Hsla,
    pub keyword: Hsla,
    pub constant: Hsla,
}

pub(super) fn default_colors() -> SimpleColors {
    SimpleColors {
        surface: hsla(220.0 / 360.0, 0.13, 0.12, 1.0),
        border: hsla(220.0 / 360.0, 0.13, 0.20, 1.0),
        hover: hsla(220.0 / 360.0, 0.13, 0.15, 1.0),
        selection: hsla(210.0 / 360.0, 0.50, 0.30, 1.0),
        text: hsla(0.0, 0.0, 0.93, 1.0),
        text_muted: hsla(0.0, 0.0, 0.60, 1.0),
        syntax: SyntaxColors {
            function: hsla(207.0 / 360.0, 0.82, 0.66, 1.0),
            variable: hsla(180.0 / 360.0, 0.40, 0.70, 1.0),
            type_name: hsla(29.0 / 360.0, 0.54, 0.61, 1.0),
            keyword: hsla(286.0 / 360.0, 0.60, 0.67, 1.0),
            constant: hsla(355.0 / 360.0, 0.65, 0.65, 1.0),
        },
    }
}

/// Events emitted by the completion dropdown
#[derive(Debug, Clone)]
pub enum CompletionDropdownEvent {
    /// A completion item was selected
    Selected(CompletionItem),
    /// Dropdown was closed
    Closed,
}
