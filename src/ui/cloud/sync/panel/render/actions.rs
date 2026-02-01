//! Actions rendering for sync status panel

use gpui::*;
use gpui::prelude::*;

use super::super::SyncStatusPanel;
use super::super::super::types::SyncStatusPanelEvent;
use crate::cloud::SyncStatus;

impl SyncStatusPanel {
    /// Render actions
    pub(crate) fn render_actions(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let is_syncing = matches!(self.status, SyncStatus::Syncing);
        let border_color = theme.colors.border;
        let text_muted = theme.colors.text_muted;
        let accent_color = theme.colors.accent;
        let surface_hover = theme.colors.surface_hover;
        let background_color = theme.colors.background;
        let surface_color = theme.colors.surface;

        let on_toggle_auto_sync = cx.listener(|this, _event, _window, cx| {
            this.toggle_auto_sync(cx);
        });

        let on_sync_now = cx.listener(|_this, _event, _window, cx| {
            cx.emit(SyncStatusPanelEvent::TriggerSync);
        });

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .px_3()
            .py_2()
            .border_t_1()
            .border_color(border_color)
            // Auto-sync toggle
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_xs()
                            .text_color(text_muted)
                            .child("Auto-sync"),
                    )
                    .child(
                        div()
                            .id("toggle-auto-sync")
                            .w(px(36.0))
                            .h(px(20.0))
                            .rounded_full()
                            .cursor_pointer()
                            .bg(if self.auto_sync {
                                accent_color
                            } else {
                                surface_hover
                            })
                            .on_click(on_toggle_auto_sync)
                            .child(
                                div()
                                    .size(px(16.0))
                                    .rounded_full()
                                    .bg(background_color)
                                    .mt(px(2.0))
                                    .ml(if self.auto_sync { px(18.0) } else { px(2.0) }),
                            ),
                    ),
            )
            // Sync now button
            .child(
                div()
                    .id("sync-now")
                    .px_3()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .bg(if is_syncing {
                        surface_color
                    } else {
                        accent_color
                    })
                    .text_sm()
                    .text_color(if is_syncing {
                        text_muted
                    } else {
                        background_color
                    })
                    .when(!is_syncing, |this| {
                this.hover(|this| this.opacity(0.9))
                    })
                    .when(is_syncing, |this| {
                this.cursor_not_allowed()
                    })
                    .on_click(on_sync_now)
                    .child(if is_syncing { "Syncing..." } else { "Sync Now" }),
            )
    }
}
