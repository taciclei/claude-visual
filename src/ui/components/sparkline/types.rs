//! Shared types for sparkline components

/// Sparkline variant
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SparklineVariant {
    #[default]
    Line,
    Area,
    Bar,
    Dots,
}

/// Sparkline size
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SparklineSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
}

impl SparklineSize {
    pub(crate) fn height(&self) -> f32 {
        match self {
            Self::Xs => 16.0,
            Self::Sm => 24.0,
            Self::Md => 32.0,
            Self::Lg => 48.0,
        }
    }

    pub(crate) fn width(&self) -> f32 {
        match self {
            Self::Xs => 48.0,
            Self::Sm => 64.0,
            Self::Md => 80.0,
            Self::Lg => 120.0,
        }
    }
}

/// Trend indicator - shows up/down/flat trend
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TrendDirection {
    Up,
    Down,
    #[default]
    Flat,
}

/// Trend indicator size
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TrendSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl TrendSize {
    pub(crate) fn font_size(&self) -> f32 {
        match self {
            Self::Sm => 12.0,
            Self::Md => 14.0,
            Self::Lg => 16.0,
        }
    }
}
