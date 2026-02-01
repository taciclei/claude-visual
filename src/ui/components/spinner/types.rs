//! Shared types and enums for spinner components

/// Spinner size
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SpinnerSize {
    /// Extra small (12px)
    XSmall,
    /// Small (16px)
    Small,
    /// Medium (24px, default)
    #[default]
    Medium,
    /// Large (32px)
    Large,
    /// Extra large (48px)
    XLarge,
    /// Custom size
    Custom(f32),
}

impl SpinnerSize {
    pub(crate) fn size(&self) -> f32 {
        match self {
            SpinnerSize::XSmall => 12.0,
            SpinnerSize::Small => 16.0,
            SpinnerSize::Medium => 24.0,
            SpinnerSize::Large => 32.0,
            SpinnerSize::XLarge => 48.0,
            SpinnerSize::Custom(size) => *size,
        }
    }

    pub(crate) fn stroke_width(&self) -> f32 {
        match self {
            SpinnerSize::XSmall => 1.5,
            SpinnerSize::Small => 2.0,
            SpinnerSize::Medium => 2.5,
            SpinnerSize::Large => 3.0,
            SpinnerSize::XLarge => 4.0,
            SpinnerSize::Custom(size) => size / 8.0,
        }
    }
}

/// Spinner variant/style
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SpinnerVariant {
    /// Circular spinner (default)
    #[default]
    Circular,
    /// Dots pulsing
    Dots,
    /// Bar/line spinner
    Bars,
    /// Ring with gap
    Ring,
}
