//! Shared types for card components

/// Card style variant
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CardVariant {
    /// Default with subtle border
    #[default]
    Default,
    /// Elevated with shadow effect
    Elevated,
    /// Outlined with stronger border
    Outlined,
    /// Ghost (no background, just padding)
    Ghost,
    /// Interactive (hover effects)
    Interactive,
}

/// Card padding size
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CardPadding {
    /// No padding
    None,
    /// Small (8px)
    Small,
    /// Medium (16px) - default
    #[default]
    Medium,
    /// Large (24px)
    Large,
}

impl CardPadding {
    pub(crate) fn pixels(&self) -> f32 {
        match self {
            CardPadding::None => 0.0,
            CardPadding::Small => 8.0,
            CardPadding::Medium => 16.0,
            CardPadding::Large => 24.0,
        }
    }
}

/// Events emitted by Card
#[derive(Debug, Clone)]
pub enum CardEvent {
    /// Card was clicked (for interactive cards)
    Clicked,
}
