//! Meter type definitions

/// Meter size variants
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MeterSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Meter variant styles
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MeterVariant {
    #[default]
    Default,
    Success,
    Warning,
    Danger,
    Info,
    Gradient,
}

/// Meter orientation
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MeterOrientation {
    #[default]
    Horizontal,
    Vertical,
}
