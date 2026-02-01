//! Link type definitions

use gpui::prelude::*;
use gpui::*;

/// Link style variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum LinkVariant {
    #[default]
    Default,
    Subtle,
    Underline,
    Bold,
    Button,
}

/// Link size options
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum LinkSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl LinkSize {
    pub(crate) fn font_size(&self) -> f32 {
        match self {
            LinkSize::Small => 12.0,
            LinkSize::Medium => 14.0,
            LinkSize::Large => 16.0,
        }
    }
}

/// Link list direction
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum LinkListDirection {
    #[default]
    Vertical,
    Horizontal,
}
