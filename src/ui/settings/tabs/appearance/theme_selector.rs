use gpui::*;
use gpui::prelude::*;
use crate::ui::pct;
use crate::ui::settings::SettingsModal;

impl SettingsModal {
    /// Render a theme option button
    pub(crate) fn render_theme_option(
        &self,
        value: &'static str,
        label: &'static str,
        current: &str,
        theme: &crate::app::theme::Theme,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let is_selected = current == value;

        // Get preview colors for this theme
        let (bg, surface, text, accent) = self.get_theme_preview_colors(value);

        // Copy theme colors for move closure
        let accent_color = theme.colors.accent;
        let border_color = theme.colors.border;
        let text_muted_color = theme.colors.text_muted;
        let surface_color = theme.colors.surface;
        let text_color = theme.colors.text;

        // Extract listener before div chain
        let on_click = cx.listener(move |this, _, _window, cx| {
            this.pending.ui.theme = value.to_string();
            this.mark_changed(cx);
        });

        div()
            .id(SharedString::from(format!("theme-{}", value)))
            .w(px(140.0))
            .rounded_lg()
            .cursor_pointer()
            .border_2()
            .overflow_hidden()
            .when(is_selected, |d| {
                d.border_color(accent_color)
            })
            .when(!is_selected, |d| {
                d.border_color(border_color)
                    .hover(|s| s.border_color(text_muted_color))
            })
            // Color preview
            .child(
                div()
                    .h(px(60.0))
                    .bg(bg)
                    .p_2()
                    .flex()
                    .flex_col()
                    .justify_between()
                    // Mini UI preview
                    .child(
                        div()
                            .flex()
                            .gap_1()
                            // Sidebar preview
                            .child(
                                div()
                                    .w(px(24.0))
                                    .h_full()
                                    .rounded_sm()
                                    .bg(surface),
                            )
                            // Content preview
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .w_full()
                                            .h(px(4.0))
                                            .rounded_sm()
                                            .bg(text.opacity(0.3)),
                                    )
                                    .child(
                                        div()
                                            .w(pct(60.0))
                                            .h(px(4.0))
                                            .rounded_sm()
                                            .bg(accent),
                                    ),
                            ),
                    )
                    // Color swatches
                    .child(
                        div()
                            .flex()
                            .gap_1()
                            .child(
                                div()
                                    .size(px(10.0))
                                    .rounded_full()
                                    .bg(surface),
                            )
                            .child(
                                div()
                                    .size(px(10.0))
                                    .rounded_full()
                                    .bg(text),
                            )
                            .child(
                                div()
                                    .size(px(10.0))
                                    .rounded_full()
                                    .bg(accent),
                            ),
                    ),
            )
            // Label
            .child(
                div()
                    .px_3()
                    .py_2()
                    .bg(surface_color)
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(if is_selected {
                                FontWeight::SEMIBOLD
                            } else {
                                FontWeight::NORMAL
                            })
                            .text_color(text_color)
                            .child(label),
                    )
                    .when(is_selected, |d| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(accent_color)
                                .child("✓"),
                        )
                    }),
            )
            .on_click(on_click)
    }

    /// Get preview colors for a theme
    pub(crate) fn get_theme_preview_colors(&self, theme_name: &str) -> (Hsla, Hsla, Hsla, Hsla) {
        match theme_name {
            "dark" => (
                hsla(220.0 / 360.0, 0.13, 0.10, 1.0), // background
                hsla(220.0 / 360.0, 0.13, 0.15, 1.0), // surface
                hsla(0.0, 0.0, 0.93, 1.0),             // text
                hsla(210.0 / 360.0, 1.0, 0.56, 1.0),  // accent
            ),
            "light" => (
                hsla(0.0, 0.0, 0.98, 1.0),             // background
                hsla(0.0, 0.0, 1.0, 1.0),              // surface
                hsla(0.0, 0.0, 0.1, 1.0),              // text
                hsla(210.0 / 360.0, 1.0, 0.5, 1.0),   // accent
            ),
            "high-contrast-dark" => (
                hsla(0.0, 0.0, 0.0, 1.0),              // background
                hsla(0.0, 0.0, 0.08, 1.0),             // surface
                hsla(0.0, 0.0, 1.0, 1.0),              // text
                hsla(60.0 / 360.0, 1.0, 0.5, 1.0),    // accent (yellow)
            ),
            "high-contrast-light" => (
                hsla(0.0, 0.0, 1.0, 1.0),              // background
                hsla(0.0, 0.0, 0.95, 1.0),             // surface
                hsla(0.0, 0.0, 0.0, 1.0),              // text
                hsla(240.0 / 360.0, 1.0, 0.35, 1.0),  // accent (blue)
            ),
            _ => (
                hsla(220.0 / 360.0, 0.13, 0.10, 1.0),
                hsla(220.0 / 360.0, 0.13, 0.15, 1.0),
                hsla(0.0, 0.0, 0.93, 1.0),
                hsla(210.0 / 360.0, 1.0, 0.56, 1.0),
            ),
        }
    }

    /// Render an extension theme option button
    pub(crate) fn render_extension_theme_option(
        &self,
        name: &str,
        current: &str,
        theme: &crate::app::theme::Theme,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let is_selected = current == name;
        let name_owned = name.to_string();
        let name_clone = name.to_string();

        // Get metadata for tooltip
        let metadata = self.app_state.theme_loader.read().get_metadata(name).cloned();
        let author = metadata.as_ref().and_then(|m| m.author.clone());
        let ext_id = metadata.as_ref().and_then(|m| m.extension_id.clone());

        // Copy theme colors for move closure
        let accent_color = theme.colors.accent;
        let border_color = theme.colors.border;
        let surface_hover_color = theme.colors.surface_hover;
        let text_muted_color = theme.colors.text_muted;

        // Extract listener before div chain
        let on_click = cx.listener(move |this, _, _window, cx| {
            this.pending.ui.theme = name_clone.clone();
            this.mark_changed(cx);
        });

        div()
            .id(SharedString::from(format!("ext-theme-{}", name)))
            .flex()
            .flex_col()
            .px_4()
            .py_2()
            .rounded_md()
            .cursor_pointer()
            .border_2()
            .when(is_selected, |d| {
                d.border_color(accent_color)
                    .bg(accent_color.opacity(0.1))
            })
            .when(!is_selected, |d| {
                d.border_color(border_color)
                    .hover(|s| s.bg(surface_hover_color))
            })
            .child(
                div()
                    .text_sm()
                    .font_weight(if is_selected {
                        FontWeight::SEMIBOLD
                    } else {
                        FontWeight::NORMAL
                    })
                    .child(name_owned),
            )
            .when(author.is_some() || ext_id.is_some(), |d| {
                d.child(
                    div()
                        .text_xs()
                        .text_color(text_muted_color)
                        .child(format!(
                            "{}{}",
                            author.as_deref().unwrap_or(""),
                            ext_id.as_ref().map(|id| format!(" ({})", id)).unwrap_or_default()
                        )),
                )
            })
            .on_click(on_click)
    }
}
