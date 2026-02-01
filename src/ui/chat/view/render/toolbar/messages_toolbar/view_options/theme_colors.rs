//! Theme colors extraction for move closures

use gpui::*;

/// Extracted theme colors for use in move closures
pub struct ThemeColors {
    pub accent: Hsla,
    pub text_muted: Hsla,
    pub surface_hover: Hsla,
    pub text: Hsla,
    pub success: Hsla,
}

impl ThemeColors {
    /// Extract theme colors from the theme for move closures
    pub fn from_theme(theme: &crate::app::theme::Theme) -> Self {
        Self {
            accent: theme.colors.accent,
            text_muted: theme.colors.text_muted,
            surface_hover: theme.colors.surface_hover,
            text: theme.colors.text,
            success: theme.colors.success,
        }
    }
}
