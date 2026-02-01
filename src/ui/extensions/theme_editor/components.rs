//! Reusable rendering components

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::{Theme, ThemeVariant};
use crate::ui::extensions::theme_editor::ThemeEditor;
use crate::ui::extensions::theme_editor::types::EditingColor;

impl ThemeEditor {
    /// Render a variant button
    pub(crate) fn render_variant_button(
        &self,
        variant: ThemeVariant,
        theme: &Theme,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let is_selected = self.editing_theme.variant == variant;
        let label = variant.display_name();

        div()
            .id(SharedString::from(format!("variant-{}", label)))
            .px_4()
            .py_2()
            .rounded_md()
            .cursor_pointer()
            .border_2()
            .when(is_selected, |d| {
                d.border_color(theme.colors.accent)
                    .bg(theme.colors.accent.opacity(0.1))
            })
            .when(!is_selected, |d| {
                d.border_color(theme.colors.border)
                    .hover(|s| s.bg(theme.colors.surface_hover))
            })
            .text_sm()
            .font_weight(if is_selected { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
            .child(label)
            .on_click(cx.listener(move |this, _, _window, cx| {
                this.set_variant(variant, cx);
            }))
    }

    /// Render a color section
    pub(crate) fn render_color_section(
        &self,
        title: &str,
        colors: &[EditingColor],
        theme: &Theme,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let title = title.to_string();
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.colors.text)
                    .child(title),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_2()
                    .children(colors.iter().map(|&color| {
                        self.render_color_swatch(color, theme, cx)
                    })),
            )
    }

    /// Render a color swatch
    pub(crate) fn render_color_swatch(
        &self,
        color: EditingColor,
        theme: &Theme,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let value = self.get_color_value(color);
        let is_selected = self.selected_color == Some(color);
        let label = color.label();

        div()
            .id(SharedString::from(format!("color-{}", label)))
            .flex()
            .flex_col()
            .items_center()
            .gap_1()
            .cursor_pointer()
            .p_2()
            .rounded_md()
            .when(is_selected, |d| {
                d.bg(theme.colors.accent.opacity(0.1))
                    .border_2()
                    .border_color(theme.colors.accent)
            })
            .hover(|s| s.bg(theme.colors.surface_hover))
            .on_click(cx.listener(move |this, _, _window, cx| {
                this.select_color(color, cx);
            }))
            .child(
                div()
                    .size(px(32.0))
                    .rounded_md()
                    .border_1()
                    .border_color(theme.colors.border)
                    .bg(value),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child(label),
            )
    }

    /// Render color picker (shown when a color is selected)
    pub(crate) fn render_color_picker(&self, color: EditingColor, theme: &Theme, _cx: &Context<Self>) -> impl IntoElement {
        let value = self.get_color_value(color);
        let hex = self.get_color_hex(color);

        div()
            .flex()
            .flex_col()
            .gap_3()
            .p_3()
            .rounded_lg()
            .bg(theme.colors.surface)
            .border_1()
            .border_color(theme.colors.border)
            // Header
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.text)
                            .child(color.label()),
                    )
                    .child(
                        div()
                            .size(px(24.0))
                            .rounded_md()
                            .border_1()
                            .border_color(theme.colors.border)
                            .bg(value),
                    ),
            )
            // Hex input
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text_muted)
                            .child("Hex:"),
                    )
                    .child(
                        div()
                            .flex_1()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(theme.colors.background)
                            .border_1()
                            .border_color(theme.colors.border)
                            .text_sm()
                            .font_family("JetBrains Mono")
                            .text_color(theme.colors.text)
                            .child(hex),
                    ),
            )
            // HSL values display
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child(format!("H: {:.0}°", value.h * 360.0))
                    .child(format!("S: {:.0}%", value.s * 100.0))
                    .child(format!("L: {:.0}%", value.l * 100.0))
                    .child(format!("A: {:.0}%", value.a * 100.0)),
            )
    }
}
