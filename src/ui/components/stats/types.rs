//! Shared types for stats components

/// Stat card size variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum StatSize {
    /// Small stat card
    Small,
    /// Medium (default)
    #[default]
    Medium,
    /// Large stat card
    Large,
}

impl StatSize {
    pub(crate) fn value_size(&self) -> f32 {
        match self {
            StatSize::Small => 20.0,
            StatSize::Medium => 28.0,
            StatSize::Large => 36.0,
        }
    }

    pub(crate) fn label_size(&self) -> f32 {
        match self {
            StatSize::Small => 11.0,
            StatSize::Medium => 12.0,
            StatSize::Large => 14.0,
        }
    }
}

/// Trend direction
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TrendDirection {
    /// No change
    #[default]
    Neutral,
    /// Increasing (positive)
    Up,
    /// Decreasing (negative)
    Down,
}

impl TrendDirection {
    pub(crate) fn icon(&self) -> &str {
        match self {
            TrendDirection::Neutral => "→",
            TrendDirection::Up => "↑",
            TrendDirection::Down => "↓",
        }
    }
}
