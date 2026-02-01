//! Theme conversion logic from Zed format to internal format

use anyhow::Result;
use std::collections::HashMap;

use super::defaults::*;
use super::parser::parse_color;
use super::types::{SyntaxStyle, ZedThemeVariant};
use crate::app::theme::{AccessibilitySettings, SyntaxColors, Theme, ThemeColors, ThemeVariant};

/// Convert a Zed theme variant to our internal Theme
pub(crate) fn convert_variant(family_name: &str, variant: &ZedThemeVariant) -> Result<Theme> {
    let style = &variant.style;
    let is_dark = variant.appearance.to_lowercase() == "dark";

    // Determine theme variant
    let theme_variant = if is_dark {
        ThemeVariant::Dark
    } else {
        ThemeVariant::Light
    };

    // Parse colors with fallbacks
    let background = parse_color(style.background.as_deref())
        .or_else(|| parse_color(style.editor_background.as_deref()))
        .unwrap_or_else(|| {
            if is_dark {
                default_dark_bg()
            } else {
                default_light_bg()
            }
        });

    let surface = parse_color(style.element_background.as_deref()).unwrap_or_else(|| {
        if is_dark {
            default_dark_surface()
        } else {
            default_light_surface()
        }
    });

    let surface_hover = parse_color(style.element_hover.as_deref()).unwrap_or_else(|| {
        if is_dark {
            default_dark_surface_hover()
        } else {
            default_light_surface_hover()
        }
    });

    let border = parse_color(style.border.as_deref()).unwrap_or_else(|| {
        if is_dark {
            default_dark_border()
        } else {
            default_light_border()
        }
    });

    let text = parse_color(style.text.as_deref()).unwrap_or_else(|| {
        if is_dark {
            default_dark_text()
        } else {
            default_light_text()
        }
    });

    let text_muted = parse_color(style.text_muted.as_deref())
        .or_else(|| parse_color(style.text_placeholder.as_deref()))
        .unwrap_or_else(|| {
            if is_dark {
                default_dark_text_muted()
            } else {
                default_light_text_muted()
            }
        });

    // Get accent color from players or text.accent
    let accent = parse_color(style.text_accent.as_deref())
        .or_else(|| {
            style
                .players
                .as_ref()
                .and_then(|p| p.first())
                .and_then(|p| parse_color(p.cursor.as_deref()))
        })
        .unwrap_or_else(default_accent);

    // Get focus ring from border.focused or derive from accent
    let focus_ring = parse_color(style.border_focused.as_deref()).unwrap_or_else(|| {
        if is_dark {
            gpui::hsla(210.0 / 360.0, 1.0, 0.60, 1.0)
        } else {
            gpui::hsla(210.0 / 360.0, 1.0, 0.50, 1.0)
        }
    });

    // Get selection from players or derive from accent
    let selection = style
        .players
        .as_ref()
        .and_then(|p| p.first())
        .and_then(|p| parse_color(p.selection.as_deref()))
        .or_else(|| parse_color(style.element_selected.as_deref()))
        .unwrap_or_else(|| {
            if is_dark {
                gpui::hsla(210.0 / 360.0, 0.80, 0.35, 0.5)
            } else {
                gpui::hsla(210.0 / 360.0, 0.80, 0.55, 0.3)
            }
        });

    let colors = ThemeColors {
        background,
        surface,
        surface_hover,
        border,
        text,
        text_muted,
        accent,
        accent_hover: lighten(accent, 0.1),
        success: parse_color(style.status_success.as_deref()).unwrap_or_else(default_success),
        warning: parse_color(style.status_warning.as_deref()).unwrap_or_else(default_warning),
        error: parse_color(style.status_error.as_deref()).unwrap_or_else(default_error),
        info: parse_color(style.status_info.as_deref()).unwrap_or_else(default_info),
        focus_ring,
        selection,
    };

    // Parse syntax colors
    let syntax = convert_syntax(style.syntax.as_ref(), is_dark);

    Ok(Theme {
        name: format!("{} - {}", family_name, variant.name),
        variant: theme_variant,
        is_dark,
        colors,
        syntax,
        accessibility: AccessibilitySettings::default(),
    })
}

/// Convert Zed syntax styles to our SyntaxColors
pub(crate) fn convert_syntax(
    syntax: Option<&HashMap<String, SyntaxStyle>>,
    is_dark: bool,
) -> SyntaxColors {
    let get_color = |keys: &[&str]| -> gpui::Hsla {
        if let Some(syntax) = syntax {
            for key in keys {
                if let Some(style) = syntax.get(*key) {
                    if let Some(color) = parse_color(style.color.as_deref()) {
                        return color;
                    }
                }
            }
        }
        // Return default
        if is_dark {
            gpui::hsla(0.0, 0.0, 0.9, 1.0)
        } else {
            gpui::hsla(0.0, 0.0, 0.1, 1.0)
        }
    };

    SyntaxColors {
        keyword: get_color(&["keyword", "keyword.control", "keyword.function"]),
        string: get_color(&["string", "string.quoted", "string.template"]),
        number: get_color(&["number", "constant.numeric"]),
        comment: get_color(&["comment", "comment.line", "comment.block"]),
        function: get_color(&["function", "entity.name.function", "support.function"]),
        variable: get_color(&["variable", "variable.other", "identifier"]),
        constant: get_color(&["constant", "constant.language", "constant.other"]),
        type_name: get_color(&["type", "entity.name.type", "support.type"]),
        operator: get_color(&["operator", "keyword.operator"]),
        punctuation: get_color(&[
            "punctuation",
            "punctuation.delimiter",
            "punctuation.bracket",
        ]),
    }
}
