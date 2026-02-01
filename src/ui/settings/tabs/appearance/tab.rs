use gpui::*;
use gpui::prelude::*;
use crate::ui::settings::SettingsModal;

impl SettingsModal {
    /// Render the appearance tab
    pub(crate) fn render_appearance_tab(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let extension_themes: Vec<String> = self.app_state.theme_loader.read().list()
            .iter()
            .map(|s| s.to_string())
            .collect();

        div()
            .flex()
            .flex_col()
            .gap_4()
            // Theme selection
            .child(self.render_section(
                "Theme",
                "Choose the color theme for the application",
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    // Built-in themes
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child("Built-in Themes"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap_2()
                                    .child(self.render_theme_option("dark", "Dark", &self.pending.ui.theme, theme, cx))
                                    .child(self.render_theme_option("light", "Light", &self.pending.ui.theme, theme, cx))
                                    .child(self.render_theme_option("high-contrast-dark", "High Contrast Dark", &self.pending.ui.theme, theme, cx))
                                    .child(self.render_theme_option("high-contrast-light", "High Contrast Light", &self.pending.ui.theme, theme, cx)),
                            ),
                    )
                    // Extension themes (if any)
                    .when(!extension_themes.is_empty(), |d| {
                        d.child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .mt_2()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .child("Extension Themes"),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .gap_2()
                                        .children(extension_themes.iter().map(|name| {
                                            self.render_extension_theme_option(name, &self.pending.ui.theme, theme, cx)
                                        })),
                                ),
                        )
                    }),
                cx,
            ))
            // Language selection
            .child(self.render_section(
                "Language",
                "Choose the display language for the application",
                self.render_language_selector(cx),
                cx,
            ))
            // Sidebar settings
            .child(self.render_section(
                "Sidebar",
                "Configure sidebar behavior",
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(self.render_toggle(
                        "Show sidebar by default",
                        self.pending.ui.show_sidebar,
                        |this, cx| {
                            this.pending.ui.show_sidebar = !this.pending.ui.show_sidebar;
                            this.mark_changed(cx);
                        },
                        cx,
                    ))
                    .child(self.render_slider(
                        "Sidebar width",
                        self.pending.ui.sidebar_width,
                        200.0,
                        400.0,
                        |this, value, cx| {
                            this.pending.ui.sidebar_width = value;
                            this.mark_changed(cx);
                        },
                        cx,
                    )),
                cx,
            ))
    }
}
