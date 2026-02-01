//! Theme type definitions

use gpui::Hsla;
use serde::{Deserialize, Serialize};

/// Theme variant for accessibility options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThemeVariant {
    #[default]
    Dark,
    Light,
    HighContrastDark,
    HighContrastLight,
}

impl ThemeVariant {
    /// Get all available variants
    pub fn all() -> &'static [ThemeVariant] {
        &[
            ThemeVariant::Dark,
            ThemeVariant::Light,
            ThemeVariant::HighContrastDark,
            ThemeVariant::HighContrastLight,
        ]
    }

    /// Get display name for the variant
    pub fn display_name(&self) -> &'static str {
        match self {
            ThemeVariant::Dark => "Dark",
            ThemeVariant::Light => "Light",
            ThemeVariant::HighContrastDark => "High Contrast Dark",
            ThemeVariant::HighContrastLight => "High Contrast Light",
        }
    }

    /// Check if this is a dark variant
    pub fn is_dark(&self) -> bool {
        matches!(self, ThemeVariant::Dark | ThemeVariant::HighContrastDark)
    }

    /// Check if this is a high contrast variant
    pub fn is_high_contrast(&self) -> bool {
        matches!(
            self,
            ThemeVariant::HighContrastDark | ThemeVariant::HighContrastLight
        )
    }
}

/// Accessibility settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilitySettings {
    /// Reduce motion for animations
    pub reduce_motion: bool,
    /// Increase focus indicator visibility
    pub enhanced_focus: bool,
    /// Minimum contrast ratio (WCAG AA = 4.5, AAA = 7.0)
    pub min_contrast_ratio: f32,
    /// Use underlines for links in addition to color
    pub underline_links: bool,
    /// Show icons alongside color-coded status
    pub show_status_icons: bool,
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self {
            reduce_motion: false,
            enhanced_focus: false,
            min_contrast_ratio: 4.5, // WCAG AA
            underline_links: false,
            show_status_icons: true,
        }
    }
}

impl AccessibilitySettings {
    /// Settings optimized for maximum accessibility
    pub fn high_accessibility() -> Self {
        Self {
            reduce_motion: true,
            enhanced_focus: true,
            min_contrast_ratio: 7.0, // WCAG AAA
            underline_links: true,
            show_status_icons: true,
        }
    }
}

/// Color palette for a theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    /// Background color
    pub background: Hsla,
    /// Surface color (panels, cards)
    pub surface: Hsla,
    /// Surface hover color
    pub surface_hover: Hsla,
    /// Border color
    pub border: Hsla,
    /// Primary text color
    pub text: Hsla,
    /// Secondary text color
    pub text_muted: Hsla,
    /// Accent color
    pub accent: Hsla,
    /// Accent hover color
    pub accent_hover: Hsla,
    /// Success color
    pub success: Hsla,
    /// Warning color
    pub warning: Hsla,
    /// Error color
    pub error: Hsla,
    /// Info color
    pub info: Hsla,
    /// Focus ring color (for accessibility)
    pub focus_ring: Hsla,
    /// Selection background
    pub selection: Hsla,
}

/// Syntax highlighting colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxColors {
    pub keyword: Hsla,
    pub string: Hsla,
    pub number: Hsla,
    pub comment: Hsla,
    pub function: Hsla,
    pub variable: Hsla,
    pub constant: Hsla,
    pub type_name: Hsla,
    pub operator: Hsla,
    pub punctuation: Hsla,
}
