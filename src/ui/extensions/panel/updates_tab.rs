use gpui::*;
use gpui::prelude::*;

use super::panel::ExtensionsPanel;

impl ExtensionsPanel {
    /// Render the updates tab
    pub(super) fn render_updates_tab(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let updates: Vec<_> = self.installed.iter().filter(|e| e.has_update).collect();

        div()
            .flex_1()
            .flex()
            .flex_col()
            .overflow_hidden()
            .when(updates.is_empty(), |d| {
                d.items_center()
                    .justify_center()
                    .gap_2()
                    .child(
                        div()
                            .text_2xl()
                            .child("All up to date!"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text_muted)
                            .child("No extension updates available"),
                    )
            })
            .when(!updates.is_empty(), |d| {
                d.p_4()
                    .gap_3()
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text_muted)
                            .child(format!("{} update(s) available", updates.len())),
                    )
                    .children(updates.iter().map(|ext| {
                        self.render_extension_item(ext, false, cx)
                    }))
            })
    }
}
