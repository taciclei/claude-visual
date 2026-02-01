//! Shared types for rating components

/// Rating size variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum RatingSize {
    /// Small (16px)
    Small,
    /// Medium (24px) - default
    #[default]
    Medium,
    /// Large (32px)
    Large,
}

impl RatingSize {
    pub(crate) fn icon_size(&self) -> f32 {
        match self {
            RatingSize::Small => 16.0,
            RatingSize::Medium => 24.0,
            RatingSize::Large => 32.0,
        }
    }
}

/// Events emitted by Rating
#[derive(Debug, Clone)]
pub enum RatingEvent {
    /// Rating changed
    Changed(f32),
    /// Hover over rating
    Hover(f32),
    /// Hover ended
    HoverEnd,
}
