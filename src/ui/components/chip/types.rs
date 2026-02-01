//! Shared types for chip components

use gpui::*;

/// Chip size variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ChipSize {
    /// Small chip
    Small,
    /// Medium (default)
    #[default]
    Medium,
    /// Large chip
    Large,
}

impl ChipSize {
    pub(crate) fn height(&self) -> f32 {
        match self {
            ChipSize::Small => 24.0,
            ChipSize::Medium => 32.0,
            ChipSize::Large => 40.0,
        }
    }

    pub(crate) fn font_size(&self) -> f32 {
        match self {
            ChipSize::Small => 11.0,
            ChipSize::Medium => 13.0,
            ChipSize::Large => 14.0,
        }
    }

    pub(crate) fn padding(&self) -> f32 {
        match self {
            ChipSize::Small => 8.0,
            ChipSize::Medium => 12.0,
            ChipSize::Large => 16.0,
        }
    }
}

/// Chip variant/style
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ChipVariant {
    /// Filled background (default)
    #[default]
    Filled,
    /// Outlined border only
    Outlined,
    /// Soft/subtle background
    Soft,
}

/// Events emitted by Chip
#[derive(Debug, Clone)]
pub enum ChipEvent {
    /// Chip clicked
    Clicked,
    /// Chip deleted/removed
    Deleted,
}
