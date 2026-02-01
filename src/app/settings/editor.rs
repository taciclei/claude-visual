//! Editor settings

use serde::{Deserialize, Serialize};

/// Editor-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    /// Font family for code
    pub code_font_family: String,
    /// Font size for code
    pub code_font_size: f32,
    /// Enable vim keybindings
    pub vim_mode: bool,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            code_font_family: "JetBrains Mono".to_string(),
            code_font_size: 14.0,
            vim_mode: false,
        }
    }
}

impl EditorSettings {
    // Font size constants
    const MIN_FONT_SIZE: f32 = 8.0;
    const MAX_FONT_SIZE: f32 = 32.0;
    const FONT_SIZE_STEP: f32 = 1.0;

    /// Increase code font size
    pub fn increase_code_font_size(&mut self) {
        self.code_font_size = (self.code_font_size + Self::FONT_SIZE_STEP).min(Self::MAX_FONT_SIZE);
    }

    /// Decrease code font size
    pub fn decrease_code_font_size(&mut self) {
        self.code_font_size = (self.code_font_size - Self::FONT_SIZE_STEP).max(Self::MIN_FONT_SIZE);
    }

    /// Reset code font size to default
    pub fn reset_code_font_size(&mut self) {
        self.code_font_size = 14.0;
    }
}
