//! Header rendering for sync status panel

use gpui::prelude::*;
use gpui::*;

use super::super::super::utils;
use super::super::SyncStatusPanel;

impl SyncStatusPanel {
    /// Render header
    pub(crate) fn render_header(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let status_color = utils::status_color(self.status, &self.app_state, cx);
        let status_text = utils::status_text(self.status, true);
        let border_color = theme.colors.border;
        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;

        let on_toggle = cx.listener(|this, _event, _window, cx| {
            this.toggle_expanded(cx);
        });

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .px_3()
            .py_2()
            .border_b_1()
            .border_color(border_color)
            .child(
                div()
                    .id("sync-panel-header-toggle")
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .cursor_pointer()
                    .on_click(on_toggle)
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(text_color)
                            .child("Cloud Sync"),
                    )
                    .child(div().size_2().rounded_full().bg(status_color)),
            )
            .child(div().text_xs().text_color(text_muted).child(status_text))
    }
}
