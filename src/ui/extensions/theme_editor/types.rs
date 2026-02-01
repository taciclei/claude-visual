//! Types for theme editor

use crate::ui::extensions::theme_editor::ThemeEditor;
use gpui::*;

/// Color being edited
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditingColor {
    // Theme colors
    Background,
    Surface,
    SurfaceHover,
    Border,
    Text,
    TextMuted,
    Accent,
    AccentHover,
    Success,
    Warning,
    Error,
    Info,
    FocusRing,
    Selection,
    // Syntax colors
    Keyword,
    String,
    Number,
    Comment,
    Function,
    Variable,
    Constant,
    TypeName,
    Operator,
    Punctuation,
}

impl EditingColor {
    pub(crate) fn label(&self) -> &'static str {
        match self {
            Self::Background => "Background",
            Self::Surface => "Surface",
            Self::SurfaceHover => "Surface Hover",
            Self::Border => "Border",
            Self::Text => "Text",
            Self::TextMuted => "Text Muted",
            Self::Accent => "Accent",
            Self::AccentHover => "Accent Hover",
            Self::Success => "Success",
            Self::Warning => "Warning",
            Self::Error => "Error",
            Self::Info => "Info",
            Self::FocusRing => "Focus Ring",
            Self::Selection => "Selection",
            Self::Keyword => "Keyword",
            Self::String => "String",
            Self::Number => "Number",
            Self::Comment => "Comment",
            Self::Function => "Function",
            Self::Variable => "Variable",
            Self::Constant => "Constant",
            Self::TypeName => "Type Name",
            Self::Operator => "Operator",
            Self::Punctuation => "Punctuation",
        }
    }

    pub(crate) fn category(&self) -> &'static str {
        match self {
            Self::Background | Self::Surface | Self::SurfaceHover | Self::Border => "Background",
            Self::Text | Self::TextMuted => "Text",
            Self::Accent | Self::AccentHover => "Accent",
            Self::Success | Self::Warning | Self::Error | Self::Info => "Status",
            Self::FocusRing | Self::Selection => "Interaction",
            _ => "Syntax",
        }
    }

    pub(crate) fn all_theme_colors() -> &'static [EditingColor] {
        &[
            Self::Background,
            Self::Surface,
            Self::SurfaceHover,
            Self::Border,
            Self::Text,
            Self::TextMuted,
            Self::Accent,
            Self::AccentHover,
            Self::Success,
            Self::Warning,
            Self::Error,
            Self::Info,
            Self::FocusRing,
            Self::Selection,
        ]
    }

    pub(crate) fn all_syntax_colors() -> &'static [EditingColor] {
        &[
            Self::Keyword,
            Self::String,
            Self::Number,
            Self::Comment,
            Self::Function,
            Self::Variable,
            Self::Constant,
            Self::TypeName,
            Self::Operator,
            Self::Punctuation,
        ]
    }
}

/// Tab selection for theme editor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeEditorTab {
    Colors,
    Syntax,
    Preview,
    Export,
}

impl ThemeEditorTab {
    pub(crate) fn label(&self) -> &'static str {
        match self {
            Self::Colors => "Colors",
            Self::Syntax => "Syntax",
            Self::Preview => "Preview",
            Self::Export => "Export",
        }
    }

    pub(crate) fn all() -> &'static [ThemeEditorTab] {
        &[Self::Colors, Self::Syntax, Self::Preview, Self::Export]
    }
}

/// Events emitted by the theme editor
#[derive(Debug, Clone)]
pub enum ThemeEditorEvent {
    /// Theme was saved
    Saved(String),
    /// Editor was closed
    Closed,
    /// Theme was applied for preview
    PreviewApplied,
}

impl EventEmitter<ThemeEditorEvent> for ThemeEditor {}
