//! Avatar type definitions

use gpui::*;

/// Avatar size variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AvatarSize {
    /// Extra small (16px)
    XSmall,
    /// Small (24px)
    Small,
    /// Medium (32px) - default
    #[default]
    Medium,
    /// Large (48px)
    Large,
    /// Extra large (64px)
    XLarge,
}

impl AvatarSize {
    pub(crate) fn pixels(&self) -> f32 {
        match self {
            AvatarSize::XSmall => 16.0,
            AvatarSize::Small => 24.0,
            AvatarSize::Medium => 32.0,
            AvatarSize::Large => 48.0,
            AvatarSize::XLarge => 64.0,
        }
    }

    pub(crate) fn font_size(&self) -> f32 {
        match self {
            AvatarSize::XSmall => 8.0,
            AvatarSize::Small => 10.0,
            AvatarSize::Medium => 14.0,
            AvatarSize::Large => 20.0,
            AvatarSize::XLarge => 28.0,
        }
    }
}

/// Avatar shape
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AvatarShape {
    /// Circular
    #[default]
    Circle,
    /// Rounded square
    Rounded,
    /// Square
    Square,
}

/// Avatar type/role
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AvatarRole {
    /// User avatar
    #[default]
    User,
    /// Assistant (Claude)
    Assistant,
    /// System/Tool
    System,
    /// Team member
    Team,
    /// Bot/Automation
    Bot,
}

impl AvatarRole {
    pub(crate) fn default_icon(&self) -> &'static str {
        match self {
            AvatarRole::User => "👤",
            AvatarRole::Assistant => "🤖",
            AvatarRole::System => "⚙️",
            AvatarRole::Team => "👥",
            AvatarRole::Bot => "🔧",
        }
    }

    pub(crate) fn default_initials(&self) -> &'static str {
        match self {
            AvatarRole::User => "U",
            AvatarRole::Assistant => "C",
            AvatarRole::System => "S",
            AvatarRole::Team => "T",
            AvatarRole::Bot => "B",
        }
    }
}

/// Avatar display mode
#[derive(Debug, Clone)]
pub enum AvatarContent {
    /// Show initials
    Initials(String),
    /// Show emoji/icon
    Icon(String),
    /// Show image (URL or path)
    Image(String),
    /// Use default based on role
    Default,
}

impl Default for AvatarContent {
    fn default() -> Self {
        Self::Default
    }
}

/// Presence/status indicator
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum PresenceStatus {
    /// No status shown
    #[default]
    None,
    /// Online (green)
    Online,
    /// Away (yellow)
    Away,
    /// Busy (red)
    Busy,
    /// Offline (gray)
    Offline,
}
