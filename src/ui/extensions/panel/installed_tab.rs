use gpui::prelude::*;
use gpui::*;

use super::panel::ExtensionsPanel;
use super::types::*;

impl ExtensionsPanel {
    /// Render the installed tab
    pub(super) fn render_installed_tab(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let filtered = self.filtered_extensions();
        let selected_id = self.selected.clone();

        div()
            .flex()
            .flex_row()
            .flex_1()
            .overflow_hidden()
            // Extensions list
            .child(
                div()
                    .w(px(300.0))
                    .h_full()
                    .border_r_1()
                    .border_color(theme.colors.border)
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    // Search
                    .child(
                        div()
                            .p_2()
                            .border_b_1()
                            .border_color(theme.colors.border)
                            .child(
                                div()
                                    .w_full()
                                    .px_3()
                                    .py_2()
                                    .rounded_md()
                                    .bg(theme.colors.surface)
                                    .text_sm()
                                    .text_color(theme.colors.text_muted)
                                    .child("Search extensions..."),
                            ),
                    )
                    // List
                    .child(
                        div()
                            .flex_1()
                            .id("scroll-extensions-list")
                            .overflow_y_scroll()
                            .p_2()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .when(filtered.is_empty(), |d| {
                                d.child(
                                    div()
                                        .py_8()
                                        .flex()
                                        .flex_col()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .text_2xl()
                                                .text_color(theme.colors.text_muted)
                                                .child("No extensions installed"),
                                        )
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(theme.colors.text_muted)
                                                .child("Browse the marketplace to find extensions"),
                                        ),
                                )
                            })
                            .children(filtered.into_iter().map(|ext| {
                                let is_selected = selected_id.as_deref() == Some(&ext.manifest.id);
                                self.render_extension_item(ext, is_selected, cx)
                            })),
                    ),
            )
            // Details panel
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .id("scroll-extension-details")
                    .overflow_y_scroll()
                    .when(self.selected.is_none(), |d| {
                        d.flex().items_center().justify_center().child(
                            div()
                                .text_sm()
                                .text_color(theme.colors.text_muted)
                                .child("Select an extension to view details"),
                        )
                    })
                    .when(self.selected.is_some(), |d| {
                        if let Some(ext) = self
                            .installed
                            .iter()
                            .find(|e| self.selected.as_ref() == Some(&e.manifest.id))
                        {
                            d.child(self.render_extension_details(ext, cx))
                        } else {
                            d
                        }
                    }),
            )
    }
}
