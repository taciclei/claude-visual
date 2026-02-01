//! Default theme implementations

use super::colors::hsla;
use super::types::{AccessibilitySettings, SyntaxColors, ThemeColors, ThemeVariant};
use super::Theme;

impl Theme {
    /// Create dark theme (default)
    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),
            variant: ThemeVariant::Dark,
            is_dark: true,
            colors: ThemeColors {
                background: hsla(220.0 / 360.0, 0.13, 0.09, 1.0),
                surface: hsla(220.0 / 360.0, 0.13, 0.12, 1.0),
                surface_hover: hsla(220.0 / 360.0, 0.13, 0.15, 1.0),
                border: hsla(220.0 / 360.0, 0.13, 0.20, 1.0),
                text: hsla(0.0, 0.0, 0.93, 1.0),
                text_muted: hsla(0.0, 0.0, 0.60, 1.0),
                accent: hsla(210.0 / 360.0, 0.80, 0.55, 1.0),
                accent_hover: hsla(210.0 / 360.0, 0.80, 0.60, 1.0),
                success: hsla(142.0 / 360.0, 0.71, 0.45, 1.0),
                warning: hsla(38.0 / 360.0, 0.92, 0.50, 1.0),
                error: hsla(0.0, 0.84, 0.60, 1.0),
                info: hsla(199.0 / 360.0, 0.89, 0.48, 1.0),
                focus_ring: hsla(210.0 / 360.0, 1.0, 0.60, 1.0),
                selection: hsla(210.0 / 360.0, 0.80, 0.35, 0.5),
            },
            syntax: SyntaxColors {
                keyword: hsla(280.0 / 360.0, 0.65, 0.70, 1.0),
                string: hsla(95.0 / 360.0, 0.60, 0.60, 1.0),
                number: hsla(30.0 / 360.0, 0.90, 0.65, 1.0),
                comment: hsla(0.0, 0.0, 0.45, 1.0),
                function: hsla(200.0 / 360.0, 0.75, 0.65, 1.0),
                variable: hsla(0.0, 0.0, 0.90, 1.0),
                constant: hsla(30.0 / 360.0, 0.90, 0.65, 1.0),
                type_name: hsla(180.0 / 360.0, 0.60, 0.60, 1.0),
                operator: hsla(0.0, 0.0, 0.80, 1.0),
                punctuation: hsla(0.0, 0.0, 0.70, 1.0),
            },
            accessibility: AccessibilitySettings::default(),
        }
    }

    /// Create light theme
    pub fn light() -> Self {
        Self {
            name: "Light".to_string(),
            variant: ThemeVariant::Light,
            is_dark: false,
            colors: ThemeColors {
                background: hsla(0.0, 0.0, 0.98, 1.0),
                surface: hsla(0.0, 0.0, 1.0, 1.0),
                surface_hover: hsla(0.0, 0.0, 0.96, 1.0),
                border: hsla(0.0, 0.0, 0.88, 1.0),
                text: hsla(0.0, 0.0, 0.10, 1.0),
                text_muted: hsla(0.0, 0.0, 0.45, 1.0),
                accent: hsla(210.0 / 360.0, 0.80, 0.45, 1.0),
                accent_hover: hsla(210.0 / 360.0, 0.80, 0.40, 1.0),
                success: hsla(142.0 / 360.0, 0.71, 0.35, 1.0),
                warning: hsla(38.0 / 360.0, 0.92, 0.45, 1.0),
                error: hsla(0.0, 0.84, 0.50, 1.0),
                info: hsla(199.0 / 360.0, 0.89, 0.40, 1.0),
                focus_ring: hsla(210.0 / 360.0, 1.0, 0.50, 1.0),
                selection: hsla(210.0 / 360.0, 0.80, 0.55, 0.3),
            },
            syntax: SyntaxColors {
                keyword: hsla(280.0 / 360.0, 0.65, 0.45, 1.0),
                string: hsla(95.0 / 360.0, 0.60, 0.35, 1.0),
                number: hsla(30.0 / 360.0, 0.90, 0.40, 1.0),
                comment: hsla(0.0, 0.0, 0.50, 1.0),
                function: hsla(200.0 / 360.0, 0.75, 0.40, 1.0),
                variable: hsla(0.0, 0.0, 0.15, 1.0),
                constant: hsla(30.0 / 360.0, 0.90, 0.40, 1.0),
                type_name: hsla(180.0 / 360.0, 0.60, 0.35, 1.0),
                operator: hsla(0.0, 0.0, 0.30, 1.0),
                punctuation: hsla(0.0, 0.0, 0.40, 1.0),
            },
            accessibility: AccessibilitySettings::default(),
        }
    }

    /// Create high contrast dark theme (WCAG AAA compliant)
    /// Contrast ratio >= 7:1 for text, >= 4.5:1 for large text
    pub fn high_contrast_dark() -> Self {
        Self {
            name: "High Contrast Dark".to_string(),
            variant: ThemeVariant::HighContrastDark,
            is_dark: true,
            colors: ThemeColors {
                // Pure black background for maximum contrast
                background: hsla(0.0, 0.0, 0.0, 1.0),
                surface: hsla(0.0, 0.0, 0.05, 1.0),
                surface_hover: hsla(0.0, 0.0, 0.10, 1.0),
                // Bright white borders for visibility
                border: hsla(0.0, 0.0, 0.80, 1.0),
                // Pure white text
                text: hsla(0.0, 0.0, 1.0, 1.0),
                text_muted: hsla(0.0, 0.0, 0.75, 1.0),
                // High saturation accent colors
                accent: hsla(200.0 / 360.0, 1.0, 0.70, 1.0),
                accent_hover: hsla(200.0 / 360.0, 1.0, 0.80, 1.0),
                // High contrast status colors
                success: hsla(120.0 / 360.0, 1.0, 0.50, 1.0),
                warning: hsla(50.0 / 360.0, 1.0, 0.55, 1.0),
                error: hsla(0.0, 1.0, 0.65, 1.0),
                info: hsla(190.0 / 360.0, 1.0, 0.60, 1.0),
                // Very visible focus ring (yellow for high visibility)
                focus_ring: hsla(60.0 / 360.0, 1.0, 0.50, 1.0),
                selection: hsla(200.0 / 360.0, 1.0, 0.40, 0.6),
            },
            syntax: SyntaxColors {
                // High contrast syntax colors
                keyword: hsla(300.0 / 360.0, 1.0, 0.80, 1.0),      // Bright magenta
                string: hsla(100.0 / 360.0, 1.0, 0.65, 1.0),       // Bright green
                number: hsla(30.0 / 360.0, 1.0, 0.70, 1.0),        // Bright orange
                comment: hsla(0.0, 0.0, 0.60, 1.0),                 // Light gray
                function: hsla(190.0 / 360.0, 1.0, 0.70, 1.0),     // Bright cyan
                variable: hsla(0.0, 0.0, 1.0, 1.0),                 // White
                constant: hsla(40.0 / 360.0, 1.0, 0.70, 1.0),      // Bright yellow-orange
                type_name: hsla(170.0 / 360.0, 1.0, 0.65, 1.0),    // Bright teal
                operator: hsla(0.0, 0.0, 1.0, 1.0),                 // White
                punctuation: hsla(0.0, 0.0, 0.85, 1.0),             // Light gray
            },
            accessibility: AccessibilitySettings::high_accessibility(),
        }
    }

    /// Create high contrast light theme (WCAG AAA compliant)
    pub fn high_contrast_light() -> Self {
        Self {
            name: "High Contrast Light".to_string(),
            variant: ThemeVariant::HighContrastLight,
            is_dark: false,
            colors: ThemeColors {
                // Pure white background
                background: hsla(0.0, 0.0, 1.0, 1.0),
                surface: hsla(0.0, 0.0, 0.97, 1.0),
                surface_hover: hsla(0.0, 0.0, 0.93, 1.0),
                // Very dark borders
                border: hsla(0.0, 0.0, 0.20, 1.0),
                // Pure black text
                text: hsla(0.0, 0.0, 0.0, 1.0),
                text_muted: hsla(0.0, 0.0, 0.25, 1.0),
                // Dark accent colors
                accent: hsla(210.0 / 360.0, 1.0, 0.30, 1.0),
                accent_hover: hsla(210.0 / 360.0, 1.0, 0.20, 1.0),
                // Dark status colors
                success: hsla(140.0 / 360.0, 1.0, 0.25, 1.0),
                warning: hsla(30.0 / 360.0, 1.0, 0.35, 1.0),
                error: hsla(0.0, 1.0, 0.35, 1.0),
                info: hsla(200.0 / 360.0, 1.0, 0.30, 1.0),
                // Dark focus ring
                focus_ring: hsla(0.0, 0.0, 0.0, 1.0),
                selection: hsla(210.0 / 360.0, 1.0, 0.70, 0.5),
            },
            syntax: SyntaxColors {
                // Dark, high contrast syntax colors
                keyword: hsla(280.0 / 360.0, 1.0, 0.35, 1.0),      // Dark purple
                string: hsla(120.0 / 360.0, 1.0, 0.25, 1.0),       // Dark green
                number: hsla(20.0 / 360.0, 1.0, 0.35, 1.0),        // Dark orange
                comment: hsla(0.0, 0.0, 0.40, 1.0),                 // Dark gray
                function: hsla(200.0 / 360.0, 1.0, 0.30, 1.0),     // Dark blue
                variable: hsla(0.0, 0.0, 0.0, 1.0),                 // Black
                constant: hsla(30.0 / 360.0, 1.0, 0.30, 1.0),      // Dark orange
                type_name: hsla(180.0 / 360.0, 1.0, 0.25, 1.0),    // Dark teal
                operator: hsla(0.0, 0.0, 0.0, 1.0),                 // Black
                punctuation: hsla(0.0, 0.0, 0.20, 1.0),             // Dark gray
            },
            accessibility: AccessibilitySettings::high_accessibility(),
        }
    }
}
