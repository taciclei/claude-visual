//! Badge type definitions and shared enums

/// Badge style variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum BadgeVariant {
    /// Default subtle badge
    #[default]
    Default,
    /// Primary/accent color
    Primary,
    /// Success (green)
    Success,
    /// Warning (yellow/orange)
    Warning,
    /// Error/destructive (red)
    Error,
    /// Outline style
    Outline,
}

/// Badge size variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum BadgeSize {
    /// Extra small (for dots)
    XSmall,
    /// Small
    #[default]
    Small,
    /// Medium
    Medium,
    /// Large
    Large,
}

impl BadgeSize {
    pub(crate) fn height(&self) -> f32 {
        match self {
            BadgeSize::XSmall => 8.0,
            BadgeSize::Small => 18.0,
            BadgeSize::Medium => 22.0,
            BadgeSize::Large => 26.0,
        }
    }

    pub(crate) fn font_size(&self) -> f32 {
        match self {
            BadgeSize::XSmall => 0.0, // No text for dot
            BadgeSize::Small => 10.0,
            BadgeSize::Medium => 12.0,
            BadgeSize::Large => 14.0,
        }
    }

    pub(crate) fn padding(&self) -> f32 {
        match self {
            BadgeSize::XSmall => 0.0,
            BadgeSize::Small => 6.0,
            BadgeSize::Medium => 8.0,
            BadgeSize::Large => 10.0,
        }
    }
}

/// Badge position for wrapper component
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum BadgePosition {
    #[default]
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}
