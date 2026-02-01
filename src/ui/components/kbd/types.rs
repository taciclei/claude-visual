//! Shared types for keyboard shortcut components

use gpui::*;

/// Keyboard key size
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum KbdSize {
    /// Small
    Small,
    /// Medium (default)
    #[default]
    Medium,
    /// Large
    Large,
}

impl KbdSize {
    pub(crate) fn padding(&self) -> (f32, f32) {
        match self {
            KbdSize::Small => (4.0, 2.0),
            KbdSize::Medium => (6.0, 3.0),
            KbdSize::Large => (8.0, 4.0),
        }
    }

    pub(crate) fn font_size(&self) -> f32 {
        match self {
            KbdSize::Small => 10.0,
            KbdSize::Medium => 12.0,
            KbdSize::Large => 14.0,
        }
    }

    pub(crate) fn min_width(&self) -> f32 {
        match self {
            KbdSize::Small => 18.0,
            KbdSize::Medium => 22.0,
            KbdSize::Large => 28.0,
        }
    }
}

/// Keyboard key style
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum KbdStyle {
    /// Default style with border
    #[default]
    Default,
    /// Flat style without shadow
    Flat,
    /// Outline only
    Outline,
    /// Minimal text only
    Minimal,
}

/// Platform-specific modifier symbols
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Platform {
    Mac,
    Windows,
    Linux,
}

impl Platform {
    pub(crate) fn cmd(&self) -> &str {
        match self {
            Platform::Mac => "⌘",
            Platform::Windows | Platform::Linux => "Ctrl",
        }
    }

    pub(crate) fn opt(&self) -> &str {
        match self {
            Platform::Mac => "⌥",
            Platform::Windows | Platform::Linux => "Alt",
        }
    }

    pub(crate) fn ctrl(&self) -> &str {
        match self {
            Platform::Mac => "⌃",
            Platform::Windows | Platform::Linux => "Ctrl",
        }
    }

    pub(crate) fn shift(&self) -> &str {
        match self {
            Platform::Mac => "⇧",
            Platform::Windows | Platform::Linux => "Shift",
        }
    }
}

impl Default for Platform {
    fn default() -> Self {
        // Default to Mac for now
        Platform::Mac
    }
}
