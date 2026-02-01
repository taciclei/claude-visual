use gpui::prelude::*;
use gpui::*;

use super::panel::ExtensionsPanel;
use super::types::*;

impl ExtensionsPanel {
    /// Render an extension item
    pub(super) fn render_extension_item(
        &self,
        ext: &ExtensionItem,
        is_selected: bool,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let id = ext.manifest.id.clone();
        let id_for_click = id.clone();

        div()
            .id(SharedString::from(format!("ext-{}", id)))
            .w_full()
            .p_3()
            .rounded_lg()
            .cursor_pointer()
            .when(is_selected, |d| {
                d.bg(theme.colors.accent.opacity(0.1))
                    .border_1()
                    .border_color(theme.colors.accent)
            })
            .when(!is_selected, |d| {
                d.bg(theme.colors.surface)
                    .hover(|s| s.bg(theme.colors.surface_hover))
            })
            .on_click(cx.listener(move |this, _, _window, cx| {
                this.select_extension(&id_for_click, cx);
            }))
            .child(
                div()
                    .flex()
                    .items_start()
                    .justify_between()
                    .gap_3()
                    // Extension info
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap_1()
                            // Name and version
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.colors.text)
                                            .child(ext.manifest.name.clone()),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child(format!("v{}", ext.manifest.version)),
                                    )
                                    .when(ext.has_update, |d| {
                                        d.child(
                                            div()
                                                .px_1()
                                                .py_px()
                                                .rounded_sm()
                                                .bg(theme.colors.warning.opacity(0.2))
                                                .text_xs()
                                                .text_color(theme.colors.warning)
                                                .child("Update"),
                                        )
                                    }),
                            )
                            // Description
                            .when(ext.manifest.description.is_some(), |d| {
                                d.child(
                                    div().text_xs().text_color(theme.colors.text_muted).child(
                                        ext.manifest.description.clone().unwrap_or_default(),
                                    ),
                                )
                            })
                            // Authors
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child(format!("by {}", ext.manifest.authors.join(", "))),
                            ),
                    )
                    // Status indicator
                    .child(
                        div()
                            .size(px(10.0))
                            .rounded_full()
                            .when(ext.enabled, |d| d.bg(theme.colors.success))
                            .when(!ext.enabled, |d| d.bg(theme.colors.text_muted)),
                    ),
            )
    }

    /// Render extension details
    pub(super) fn render_extension_details(
        &self,
        ext: &ExtensionItem,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let id = ext.manifest.id.clone();
        let id_for_toggle = id.clone();
        let id_for_uninstall = id.clone();

        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            // Header
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.colors.text)
                            .child(ext.manifest.name.clone()),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.colors.text_muted)
                                    .child(format!("Version {}", ext.manifest.version)),
                            )
                            .when(ext.manifest.repository.is_some(), |d| {
                                d.child(
                                    div()
                                        .text_sm()
                                        .text_color(theme.colors.accent)
                                        .cursor_pointer()
                                        .child("Repository"),
                                )
                            }),
                    ),
            )
            // Description
            .when(ext.manifest.description.is_some(), |d| {
                d.child(
                    div()
                        .p_3()
                        .rounded_lg()
                        .bg(theme.colors.surface)
                        .text_sm()
                        .text_color(theme.colors.text)
                        .child(ext.manifest.description.clone().unwrap_or_default()),
                )
            })
            // Authors
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.text_muted)
                            .child("AUTHORS"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text)
                            .child(ext.manifest.authors.join(", ")),
                    ),
            )
            // Features
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.text_muted)
                            .child("PROVIDES"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_1()
                            .when(ext.manifest.themes.is_some(), |d| {
                                d.child(self.render_feature_badge("Themes", cx))
                            })
                            .when(ext.manifest.languages.is_some(), |d| {
                                d.child(self.render_feature_badge("Languages", cx))
                            })
                            .when(ext.manifest.lib.is_some(), |d| {
                                d.child(self.render_feature_badge("Commands", cx))
                            })
                            .when(ext.manifest.grammars.is_some(), |d| {
                                d.child(self.render_feature_badge("Grammars", cx))
                            }),
                    ),
            )
            // Actions
            .child(
                div()
                    .flex()
                    .gap_2()
                    .mt_4()
                    // Enable/Disable button
                    .child(
                        div()
                            .id(SharedString::from(format!("toggle-{}", id)))
                            .flex_1()
                            .px_4()
                            .py_2()
                            .rounded_md()
                            .cursor_pointer()
                            .text_sm()
                            .text_center()
                            .when(ext.enabled, |d| {
                                d.bg(theme.colors.surface_hover)
                                    .text_color(theme.colors.text)
                                    .hover(|s| s.bg(theme.colors.border))
                            })
                            .when(!ext.enabled, |d| {
                                d.bg(theme.colors.accent)
                                    .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                                    .hover(|s| s.opacity(0.9))
                            })
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                this.toggle_extension(&id_for_toggle, cx);
                            }))
                            .child(if ext.enabled { "Disable" } else { "Enable" }),
                    )
                    // Uninstall button
                    .child(
                        div()
                            .id(SharedString::from(format!(
                                "uninstall-{}",
                                id_for_uninstall.clone()
                            )))
                            .px_4()
                            .py_2()
                            .rounded_md()
                            .cursor_pointer()
                            .text_sm()
                            .text_color(theme.colors.error)
                            .hover(|s| s.bg(theme.colors.error.opacity(0.1)))
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                this.uninstall_extension(&id_for_uninstall, cx);
                            }))
                            .child("Uninstall"),
                    ),
            )
    }

    /// Render a feature badge
    pub(super) fn render_feature_badge(&self, label: &str, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let label = label.to_string();

        div()
            .px_2()
            .py_1()
            .rounded_md()
            .bg(theme.colors.accent.opacity(0.1))
            .text_xs()
            .text_color(theme.colors.accent)
            .child(label)
    }
}
