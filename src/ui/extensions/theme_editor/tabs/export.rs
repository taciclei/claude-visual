//! Export tab rendering

use gpui::*;

use crate::ui::extensions::theme_editor::ThemeEditor;

impl ThemeEditor {
    /// Render export tab
    pub(crate) fn render_export_tab(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let json = self.export_to_json();

        div()
            .flex()
            .flex_col()
            .gap_4()
            // JSON export
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.text)
                            .child("Zed Theme Format (JSON)"),
                    )
                    .child(
                        div()
                            .p_3()
                            .rounded_md()
                            .bg(theme.colors.surface)
                            .border_1()
                            .border_color(theme.colors.border)
                            .max_h(px(400.0))
                            .id("scroll-export-json")
                            .overflow_y_scroll()
                            .child(
                                div()
                                    .text_xs()
                                    .font_family("JetBrains Mono")
                                    .text_color(theme.colors.text)
                                    .whitespace_nowrap()
                                    .child(json),
                            ),
                    ),
            )
            // Copy button
            .child(
                div()
                    .id("copy-json")
                    .px_4()
                    .py_2()
                    .rounded_md()
                    .bg(theme.colors.accent)
                    .text_sm()
                    .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                    .cursor_pointer()
                    .hover(|s| s.opacity(0.9))
                    .child("Copy to Clipboard"),
            )
    }
}
