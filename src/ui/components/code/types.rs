//! Shared types for code display components

use gpui::*;

/// Code size
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CodeSize {
    /// Small
    Small,
    /// Medium (default)
    #[default]
    Medium,
    /// Large
    Large,
}

impl CodeSize {
    pub(crate) fn font_size(&self) -> f32 {
        match self {
            CodeSize::Small => 11.0,
            CodeSize::Medium => 13.0,
            CodeSize::Large => 15.0,
        }
    }

    pub(crate) fn padding(&self) -> (f32, f32) {
        match self {
            CodeSize::Small => (3.0, 1.0),
            CodeSize::Medium => (4.0, 2.0),
            CodeSize::Large => (6.0, 3.0),
        }
    }
}

/// JSON type variants
#[derive(Clone)]
pub enum JsonType {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Array(usize),  // Just count for display
    Object(usize), // Just count for display
}
