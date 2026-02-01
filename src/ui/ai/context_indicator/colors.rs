//! Color definitions for context indicator

use gpui::*;

pub struct SimpleColors {
    pub surface: Hsla,
    pub border: Hsla,
    pub hover: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub accent: Hsla,
    pub error: Hsla,
    pub success: Hsla,
    pub warning: Hsla,
    pub selection: Hsla,
}

pub fn default_colors() -> SimpleColors {
    SimpleColors {
        surface: hsla(220.0 / 360.0, 0.13, 0.12, 1.0),
        border: hsla(220.0 / 360.0, 0.13, 0.20, 1.0),
        hover: hsla(220.0 / 360.0, 0.13, 0.15, 1.0),
        text: hsla(0.0, 0.0, 0.93, 1.0),
        text_muted: hsla(0.0, 0.0, 0.60, 1.0),
        accent: hsla(210.0 / 360.0, 0.80, 0.55, 1.0),
        error: hsla(0.0, 0.84, 0.60, 1.0),
        success: hsla(142.0 / 360.0, 0.71, 0.45, 1.0),
        warning: hsla(38.0 / 360.0, 0.92, 0.50, 1.0),
        selection: hsla(210.0 / 360.0, 0.50, 0.30, 1.0),
    }
}
