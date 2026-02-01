//! Conflicts rendering for sync status panel

use gpui::*;
use gpui::prelude::*;

use super::super::SyncStatusPanel;
use super::super::super::types::SyncStatusPanelEvent;

impl SyncStatusPanel {
    /// Render conflict warning
    pub(crate) fn render_conflicts(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let warning_color = theme.colors.warning;
        let text_muted = theme.colors.text_muted;
        let warning_opacity_1 = theme.colors.warning.opacity(0.1);
        let warning_opacity_2 = theme.colors.warning.opacity(0.2);
        let warning_opacity_3 = theme.colors.warning.opacity(0.3);

        let on_resolve = cx.listener(|_this, _event, _window, cx| {
            cx.emit(SyncStatusPanelEvent::OpenConflicts);
        });

        div().when(self.conflict_count > 0, |this| {
                this.child(
                div()
                    .mx_3()
                    .my_2()
                    .p_3()
                    .rounded_md()
                    .bg(warning_opacity_1)
                    .border_1()
                    .border_color(warning_opacity_3)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(warning_color)
                                            .child(format!(
                                                "{} conflict{}",
                                                self.conflict_count,
                                                if self.conflict_count == 1 { "" } else { "s" }
                                            )),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(text_muted)
                                            .child("Manual resolution required"),
                                    ),
                            )
                            .child(
                                div()
                                    .id("resolve-conflicts")
                                    .px_3()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(warning_opacity_2)
                                    .text_sm()
                                    .text_color(warning_color)
                                    .hover(|this| this.bg(warning_opacity_3))
                                    .on_click(on_resolve)
                                    .child("Resolve"),
                            ),
                    ),
            )
        })
    }
}
