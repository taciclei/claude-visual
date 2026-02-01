//! UI settings

use serde::{Deserialize, Serialize};

/// UI-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UISettings {
    /// Theme name (dark, light, or custom)
    pub theme: String,
    /// Font family for UI
    pub ui_font_family: String,
    /// Font size for UI
    pub ui_font_size: f32,
    /// Show sidebar by default
    pub show_sidebar: bool,
    /// Sidebar width in pixels
    pub sidebar_width: f32,
}

impl Default for UISettings {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            ui_font_family: "Inter".to_string(),
            ui_font_size: 14.0,
            show_sidebar: true,
            sidebar_width: 280.0,
        }
    }
}

impl UISettings {
    // Font size constants
    const MIN_FONT_SIZE: f32 = 8.0;
    const MAX_FONT_SIZE: f32 = 32.0;
    const FONT_SIZE_STEP: f32 = 1.0;

    /// Increase UI font size
    pub fn increase_ui_font_size(&mut self) {
        self.ui_font_size = (self.ui_font_size + Self::FONT_SIZE_STEP).min(Self::MAX_FONT_SIZE);
    }

    /// Decrease UI font size
    pub fn decrease_ui_font_size(&mut self) {
        self.ui_font_size = (self.ui_font_size - Self::FONT_SIZE_STEP).max(Self::MIN_FONT_SIZE);
    }

    /// Reset UI font size to default
    pub fn reset_ui_font_size(&mut self) {
        self.ui_font_size = 14.0;
    }
}
