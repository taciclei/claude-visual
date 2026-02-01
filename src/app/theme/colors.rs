//! Color utilities and contrast calculations

use gpui::Hsla;

/// Helper function to create Hsla color
pub(crate) fn hsla(h: f32, s: f32, l: f32, a: f32) -> Hsla {
    Hsla { h, s, l, a }
}

/// Calculate contrast ratio between two colors
/// Returns a value between 1 and 21 (WCAG formula)
pub(crate) fn contrast_ratio(foreground: Hsla, background: Hsla) -> f32 {
    let fg_lum = relative_luminance(foreground);
    let bg_lum = relative_luminance(background);

    let lighter = fg_lum.max(bg_lum);
    let darker = fg_lum.min(bg_lum);

    (lighter + 0.05) / (darker + 0.05)
}

/// Calculate relative luminance of a color (WCAG formula)
pub(crate) fn relative_luminance(color: Hsla) -> f32 {
    // Convert HSL to RGB first
    let c = (1.0 - (2.0 * color.l - 1.0).abs()) * color.s;
    let x = c * (1.0 - ((color.h * 6.0) % 2.0 - 1.0).abs());
    let m = color.l - c / 2.0;

    let (r, g, b) = match (color.h * 6.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let (r, g, b) = (r + m, g + m, b + m);

    // Apply sRGB linearization
    let linearize = |v: f32| {
        if v <= 0.03928 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };

    0.2126 * linearize(r) + 0.7152 * linearize(g) + 0.0722 * linearize(b)
}

/// Check if the contrast ratio meets WCAG AA (4.5:1 for normal text)
pub(crate) fn meets_wcag_aa(foreground: Hsla, background: Hsla) -> bool {
    contrast_ratio(foreground, background) >= 4.5
}

/// Check if the contrast ratio meets WCAG AAA (7:1 for normal text)
pub(crate) fn meets_wcag_aaa(foreground: Hsla, background: Hsla) -> bool {
    contrast_ratio(foreground, background) >= 7.0
}
