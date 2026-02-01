//! Default theme colors

/// Lighten a color
pub(crate) fn lighten(color: gpui::Hsla, amount: f32) -> gpui::Hsla {
    gpui::hsla(color.h, color.s, (color.l + amount).min(1.0), color.a)
}

// Default colors for dark theme
pub(crate) fn default_dark_bg() -> gpui::Hsla {
    gpui::hsla(220.0 / 360.0, 0.13, 0.09, 1.0)
}
pub(crate) fn default_dark_surface() -> gpui::Hsla {
    gpui::hsla(220.0 / 360.0, 0.13, 0.12, 1.0)
}
pub(crate) fn default_dark_surface_hover() -> gpui::Hsla {
    gpui::hsla(220.0 / 360.0, 0.13, 0.15, 1.0)
}
pub(crate) fn default_dark_border() -> gpui::Hsla {
    gpui::hsla(220.0 / 360.0, 0.13, 0.20, 1.0)
}
pub(crate) fn default_dark_text() -> gpui::Hsla {
    gpui::hsla(0.0, 0.0, 0.93, 1.0)
}
pub(crate) fn default_dark_text_muted() -> gpui::Hsla {
    gpui::hsla(0.0, 0.0, 0.60, 1.0)
}

// Default colors for light theme
pub(crate) fn default_light_bg() -> gpui::Hsla {
    gpui::hsla(0.0, 0.0, 0.98, 1.0)
}
pub(crate) fn default_light_surface() -> gpui::Hsla {
    gpui::hsla(0.0, 0.0, 1.0, 1.0)
}
pub(crate) fn default_light_surface_hover() -> gpui::Hsla {
    gpui::hsla(0.0, 0.0, 0.96, 1.0)
}
pub(crate) fn default_light_border() -> gpui::Hsla {
    gpui::hsla(0.0, 0.0, 0.88, 1.0)
}
pub(crate) fn default_light_text() -> gpui::Hsla {
    gpui::hsla(0.0, 0.0, 0.10, 1.0)
}
pub(crate) fn default_light_text_muted() -> gpui::Hsla {
    gpui::hsla(0.0, 0.0, 0.45, 1.0)
}

// Shared defaults
pub(crate) fn default_accent() -> gpui::Hsla {
    gpui::hsla(210.0 / 360.0, 0.80, 0.55, 1.0)
}
pub(crate) fn default_success() -> gpui::Hsla {
    gpui::hsla(142.0 / 360.0, 0.71, 0.45, 1.0)
}
pub(crate) fn default_warning() -> gpui::Hsla {
    gpui::hsla(38.0 / 360.0, 0.92, 0.50, 1.0)
}
pub(crate) fn default_error() -> gpui::Hsla {
    gpui::hsla(0.0, 0.84, 0.60, 1.0)
}
pub(crate) fn default_info() -> gpui::Hsla {
    gpui::hsla(199.0 / 360.0, 0.89, 0.48, 1.0)
}
