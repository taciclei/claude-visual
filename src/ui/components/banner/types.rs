//! Banner types and enums

/// Banner position
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum BannerPosition {
    /// Top of viewport (default)
    #[default]
    Top,
    /// Bottom of viewport
    Bottom,
}

/// Banner type/severity
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum BannerType {
    /// Informational (default)
    #[default]
    Info,
    /// Success message
    Success,
    /// Warning message
    Warning,
    /// Error/critical message
    Error,
    /// Announcement/marketing
    Announcement,
}

impl BannerType {
    pub(crate) fn icon(&self) -> &str {
        match self {
            BannerType::Info => "ℹ️",
            BannerType::Success => "✓",
            BannerType::Warning => "⚠️",
            BannerType::Error => "✕",
            BannerType::Announcement => "📢",
        }
    }
}

/// Events emitted by Banner
#[derive(Debug, Clone)]
pub enum BannerEvent {
    /// Banner dismissed
    Dismissed,
    /// Action clicked
    ActionClicked(String),
    /// Link clicked
    LinkClicked(String),
}
