//! Color utilities for image input component

use gpui::*;

pub struct SimpleColors {
    pub surface: Hsla,
    pub border: Hsla,
    pub text_muted: Hsla,
    pub accent: Hsla,
    pub error: Hsla,
    pub editor_background: Hsla,
}

pub fn default_colors() -> SimpleColors {
    SimpleColors {
        surface: hsla(220.0 / 360.0, 0.13, 0.12, 1.0),
        border: hsla(220.0 / 360.0, 0.13, 0.20, 1.0),
        text_muted: hsla(0.0, 0.0, 0.60, 1.0),
        accent: hsla(210.0 / 360.0, 0.80, 0.55, 1.0),
        error: hsla(0.0, 0.84, 0.60, 1.0),
        editor_background: hsla(220.0 / 360.0, 0.13, 0.09, 1.0),
    }
}
