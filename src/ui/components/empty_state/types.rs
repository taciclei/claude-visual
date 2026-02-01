//! Types for empty state components

use gpui::*;

/// Empty state size variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum EmptyStateSize {
    /// Small (icon 32px)
    Small,
    /// Medium (icon 48px) - default
    #[default]
    Medium,
    /// Large (icon 64px)
    Large,
}

impl EmptyStateSize {
    pub(crate) fn icon_size(&self) -> f32 {
        match self {
            EmptyStateSize::Small => 32.0,
            EmptyStateSize::Medium => 48.0,
            EmptyStateSize::Large => 64.0,
        }
    }

    pub(crate) fn title_size(&self) -> Pixels {
        match self {
            EmptyStateSize::Small => px(14.0),
            EmptyStateSize::Medium => px(16.0),
            EmptyStateSize::Large => px(20.0),
        }
    }
}

/// Events emitted by EmptyState
#[derive(Debug, Clone)]
pub enum EmptyStateEvent {
    /// Primary action clicked
    PrimaryAction,
    /// Secondary action clicked
    SecondaryAction,
}
