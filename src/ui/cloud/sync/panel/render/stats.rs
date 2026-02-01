//! Stats rendering for sync status panel

use gpui::prelude::*;
use gpui::*;

use super::super::super::utils;
use super::super::SyncStatusPanel;

impl SyncStatusPanel {
    /// Render stats
    pub(crate) fn render_stats(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let last_sync_text = utils::format_last_sync(self.last_sync, true);
        let text_muted = theme.colors.text_muted;
        let text_color = theme.colors.text;
        let accent_color = theme.colors.accent;

        div()
            .flex()
            .flex_row()
            .gap_4()
            .px_3()
            .py_2()
            // Last sync
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_0p5()
                    .child(div().text_xs().text_color(text_muted).child("Last sync"))
                    .child(div().text_sm().text_color(text_color).child(last_sync_text)),
            )
            // Pending
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_0p5()
                    .child(div().text_xs().text_color(text_muted).child("Pending"))
                    .child(
                        div()
                            .text_sm()
                            .text_color(if self.pending_count > 0 {
                                accent_color
                            } else {
                                text_color
                            })
                            .child(format!("{}", self.pending_count)),
                    ),
            )
            // Uploaded/Downloaded
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_0p5()
                    .child(div().text_xs().text_color(text_muted).child("Up/Down"))
                    .child(
                        div()
                            .text_sm()
                            .text_color(text_color)
                            .child(format!("{}/{}", self.upload_count, self.download_count)),
                    ),
            )
    }
}
