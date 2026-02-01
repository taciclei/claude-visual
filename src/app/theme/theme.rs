//! Main theme implementation

use super::types::{AccessibilitySettings, SyntaxColors, ThemeColors, ThemeVariant};
use serde::{Deserialize, Serialize};

/// Application theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Theme variant
    pub variant: ThemeVariant,
    /// Is this a dark theme
    pub is_dark: bool,
    /// Theme colors
    pub colors: ThemeColors,
    /// Syntax highlighting colors
    pub syntax: SyntaxColors,
    /// Accessibility settings
    #[serde(default)]
    pub accessibility: AccessibilitySettings,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// Create theme from variant
    pub fn from_variant(variant: ThemeVariant) -> Self {
        match variant {
            ThemeVariant::Dark => Self::dark(),
            ThemeVariant::Light => Self::light(),
            ThemeVariant::HighContrastDark => Self::high_contrast_dark(),
            ThemeVariant::HighContrastLight => Self::high_contrast_light(),
        }
    }

    /// Get the mode name (for settings comparison)
    pub fn mode_name(&self) -> &str {
        match self.variant {
            ThemeVariant::Dark => "dark",
            ThemeVariant::Light => "light",
            ThemeVariant::HighContrastDark => "high-contrast-dark",
            ThemeVariant::HighContrastLight => "high-contrast-light",
        }
    }

    /// Toggle between dark and light mode (preserves high contrast setting)
    pub fn toggle_mode(&mut self) {
        let new_variant = match self.variant {
            ThemeVariant::Dark => ThemeVariant::Light,
            ThemeVariant::Light => ThemeVariant::Dark,
            ThemeVariant::HighContrastDark => ThemeVariant::HighContrastLight,
            ThemeVariant::HighContrastLight => ThemeVariant::HighContrastDark,
        };
        self.set_variant(new_variant);
    }

    /// Toggle high contrast mode (preserves dark/light setting)
    pub fn toggle_high_contrast(&mut self) {
        let new_variant = match self.variant {
            ThemeVariant::Dark => ThemeVariant::HighContrastDark,
            ThemeVariant::Light => ThemeVariant::HighContrastLight,
            ThemeVariant::HighContrastDark => ThemeVariant::Dark,
            ThemeVariant::HighContrastLight => ThemeVariant::Light,
        };
        self.set_variant(new_variant);
    }

    /// Set theme variant
    pub fn set_variant(&mut self, variant: ThemeVariant) {
        let new_theme = Self::from_variant(variant);
        self.name = new_theme.name;
        self.variant = new_theme.variant;
        self.is_dark = new_theme.is_dark;
        self.colors = new_theme.colors;
        self.syntax = new_theme.syntax;
        // Preserve user's accessibility settings unless switching to high contrast
        if variant.is_high_contrast() {
            self.accessibility = new_theme.accessibility;
        }
    }

    /// Switch to dark mode
    pub fn set_dark_mode(&mut self) {
        if self.variant.is_high_contrast() {
            self.set_variant(ThemeVariant::HighContrastDark);
        } else {
            self.set_variant(ThemeVariant::Dark);
        }
    }

    /// Switch to light mode
    pub fn set_light_mode(&mut self) {
        if self.variant.is_high_contrast() {
            self.set_variant(ThemeVariant::HighContrastLight);
        } else {
            self.set_variant(ThemeVariant::Light);
        }
    }

    /// Check if high contrast mode is enabled
    pub fn is_high_contrast(&self) -> bool {
        self.variant.is_high_contrast()
    }

    /// Set reduced motion preference
    pub fn set_reduce_motion(&mut self, reduce: bool) {
        self.accessibility.reduce_motion = reduce;
    }

    /// Check if motion should be reduced
    pub fn should_reduce_motion(&self) -> bool {
        self.accessibility.reduce_motion
    }
}
