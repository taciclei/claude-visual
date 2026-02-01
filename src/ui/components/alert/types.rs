//! Alert types and enums

/// Alert severity/type
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AlertType {
    /// Informational (blue)
    #[default]
    Info,
    /// Success (green)
    Success,
    /// Warning (yellow/orange)
    Warning,
    /// Error/danger (red)
    Error,
}

impl AlertType {
    pub(crate) fn icon(&self) -> &'static str {
        match self {
            AlertType::Info => "ℹ️",
            AlertType::Success => "✓",
            AlertType::Warning => "⚠️",
            AlertType::Error => "✕",
        }
    }
}

/// Alert style variant
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AlertStyle {
    /// Filled background
    #[default]
    Filled,
    /// Outline/bordered
    Outline,
    /// Subtle (light background)
    Subtle,
    /// Left border accent
    LeftAccent,
}

/// Events emitted by Alert
#[derive(Debug, Clone)]
pub enum AlertEvent {
    /// Alert was dismissed
    Dismissed,
    /// Action button was clicked
    ActionClicked,
}
