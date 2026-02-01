//! Audio player rendering

mod minimal;
mod full;
mod compact;

pub(super) use minimal::render_minimal;
pub(super) use full::render_full;
pub(super) use compact::render_compact;

use gpui::*;

/// Theme colors extracted for use in closures
#[derive(Clone, Copy)]
pub(super) struct AudioColors {
    pub primary_bg: Hsla,
    pub primary_fg: Hsla,
    pub track_bg: Hsla,
    pub track_fg: Hsla,
    pub text_primary: Hsla,
    pub text_secondary: Hsla,
    pub border: Hsla,
    pub container_bg: Hsla,
}

impl Default for AudioColors {
    fn default() -> Self {
        Self {
            primary_bg: hsla(0.6, 0.7, 0.5, 1.0),
            primary_fg: hsla(0.0, 0.0, 1.0, 1.0),
            track_bg: hsla(0.0, 0.0, 0.2, 1.0),
            track_fg: hsla(0.6, 0.7, 0.5, 1.0),
            text_primary: hsla(0.0, 0.0, 0.95, 1.0),
            text_secondary: hsla(0.0, 0.0, 0.6, 1.0),
            border: hsla(0.0, 0.0, 0.2, 1.0),
            container_bg: hsla(0.0, 0.0, 0.1, 1.0),
        }
    }
}
