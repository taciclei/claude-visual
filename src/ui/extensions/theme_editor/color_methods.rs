//! Color-related methods for ThemeEditor

use gpui::*;

use crate::ui::extensions::theme_editor::core::ThemeEditor;
use crate::ui::extensions::theme_editor::helpers::{hsla_to_hex, parse_hex_color};
use crate::ui::extensions::theme_editor::types::EditingColor;

impl ThemeEditor {
    /// Select a color for editing
    pub(crate) fn select_color(&mut self, color: EditingColor, cx: &mut Context<Self>) {
        self.selected_color = Some(color);
        self.color_input = self.get_color_hex(color);
        cx.notify();
    }

    /// Get the current value of a color as hex
    pub(crate) fn get_color_hex(&self, color: EditingColor) -> String {
        let hsla = self.get_color_value(color);
        hsla_to_hex(hsla)
    }

    /// Get the current value of a color
    pub(crate) fn get_color_value(&self, color: EditingColor) -> Hsla {
        match color {
            EditingColor::Background => self.editing_theme.colors.background,
            EditingColor::Surface => self.editing_theme.colors.surface,
            EditingColor::SurfaceHover => self.editing_theme.colors.surface_hover,
            EditingColor::Border => self.editing_theme.colors.border,
            EditingColor::Text => self.editing_theme.colors.text,
            EditingColor::TextMuted => self.editing_theme.colors.text_muted,
            EditingColor::Accent => self.editing_theme.colors.accent,
            EditingColor::AccentHover => self.editing_theme.colors.accent_hover,
            EditingColor::Success => self.editing_theme.colors.success,
            EditingColor::Warning => self.editing_theme.colors.warning,
            EditingColor::Error => self.editing_theme.colors.error,
            EditingColor::Info => self.editing_theme.colors.info,
            EditingColor::FocusRing => self.editing_theme.colors.focus_ring,
            EditingColor::Selection => self.editing_theme.colors.selection,
            EditingColor::Keyword => self.editing_theme.syntax.keyword,
            EditingColor::String => self.editing_theme.syntax.string,
            EditingColor::Number => self.editing_theme.syntax.number,
            EditingColor::Comment => self.editing_theme.syntax.comment,
            EditingColor::Function => self.editing_theme.syntax.function,
            EditingColor::Variable => self.editing_theme.syntax.variable,
            EditingColor::Constant => self.editing_theme.syntax.constant,
            EditingColor::TypeName => self.editing_theme.syntax.type_name,
            EditingColor::Operator => self.editing_theme.syntax.operator,
            EditingColor::Punctuation => self.editing_theme.syntax.punctuation,
        }
    }

    /// Set a color value
    pub(crate) fn set_color_value(
        &mut self,
        color: EditingColor,
        value: Hsla,
        cx: &mut Context<Self>,
    ) {
        match color {
            EditingColor::Background => self.editing_theme.colors.background = value,
            EditingColor::Surface => self.editing_theme.colors.surface = value,
            EditingColor::SurfaceHover => self.editing_theme.colors.surface_hover = value,
            EditingColor::Border => self.editing_theme.colors.border = value,
            EditingColor::Text => self.editing_theme.colors.text = value,
            EditingColor::TextMuted => self.editing_theme.colors.text_muted = value,
            EditingColor::Accent => self.editing_theme.colors.accent = value,
            EditingColor::AccentHover => self.editing_theme.colors.accent_hover = value,
            EditingColor::Success => self.editing_theme.colors.success = value,
            EditingColor::Warning => self.editing_theme.colors.warning = value,
            EditingColor::Error => self.editing_theme.colors.error = value,
            EditingColor::Info => self.editing_theme.colors.info = value,
            EditingColor::FocusRing => self.editing_theme.colors.focus_ring = value,
            EditingColor::Selection => self.editing_theme.colors.selection = value,
            EditingColor::Keyword => self.editing_theme.syntax.keyword = value,
            EditingColor::String => self.editing_theme.syntax.string = value,
            EditingColor::Number => self.editing_theme.syntax.number = value,
            EditingColor::Comment => self.editing_theme.syntax.comment = value,
            EditingColor::Function => self.editing_theme.syntax.function = value,
            EditingColor::Variable => self.editing_theme.syntax.variable = value,
            EditingColor::Constant => self.editing_theme.syntax.constant = value,
            EditingColor::TypeName => self.editing_theme.syntax.type_name = value,
            EditingColor::Operator => self.editing_theme.syntax.operator = value,
            EditingColor::Punctuation => self.editing_theme.syntax.punctuation = value,
        }
        self.has_changes = true;
        cx.notify();
    }

    /// Apply color from input
    pub(crate) fn apply_color_input(&mut self, cx: &mut Context<Self>) {
        if let Some(color) = self.selected_color {
            if let Some(hsla) = parse_hex_color(&self.color_input) {
                self.set_color_value(color, hsla, cx);
            }
        }
    }
}
