//! Shared types for text highlighting

use gpui::*;

/// Highlight color/style
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum HighlightColor {
    /// Yellow (default)
    #[default]
    Yellow,
    /// Green
    Green,
    /// Blue
    Blue,
    /// Pink
    Pink,
    /// Orange
    Orange,
    /// Purple
    Purple,
    /// Gray
    Gray,
}

impl HighlightColor {
    pub(crate) fn background(&self) -> Hsla {
        match self {
            HighlightColor::Yellow => hsla(0.14, 0.9, 0.5, 0.3),
            HighlightColor::Green => hsla(0.38, 0.7, 0.45, 0.3),
            HighlightColor::Blue => hsla(0.6, 0.8, 0.5, 0.3),
            HighlightColor::Pink => hsla(0.92, 0.7, 0.6, 0.3),
            HighlightColor::Orange => hsla(0.08, 0.9, 0.55, 0.3),
            HighlightColor::Purple => hsla(0.75, 0.6, 0.55, 0.3),
            HighlightColor::Gray => hsla(0.0, 0.0, 0.5, 0.3),
        }
    }

    pub(crate) fn border(&self) -> Hsla {
        match self {
            HighlightColor::Yellow => hsla(0.14, 0.9, 0.5, 0.5),
            HighlightColor::Green => hsla(0.38, 0.7, 0.45, 0.5),
            HighlightColor::Blue => hsla(0.6, 0.8, 0.5, 0.5),
            HighlightColor::Pink => hsla(0.92, 0.7, 0.6, 0.5),
            HighlightColor::Orange => hsla(0.08, 0.9, 0.55, 0.5),
            HighlightColor::Purple => hsla(0.75, 0.6, 0.55, 0.5),
            HighlightColor::Gray => hsla(0.0, 0.0, 0.5, 0.5),
        }
    }
}

/// Highlight style (renamed to avoid conflict with gpui::HighlightStyle)
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TextHighlightStyle {
    /// Background highlight (default)
    #[default]
    Background,
    /// Underline
    Underline,
    /// Border box
    Border,
    /// Glow effect
    Glow,
}
