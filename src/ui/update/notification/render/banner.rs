//! Banner rendering for update notifications

use gpui::*;
use gpui::prelude::*;

use crate::update::UpdateInfo;
use super::super::core::UpdateNotification;
use super::super::types::{SimpleColors, UpdateNotificationEvent};

impl UpdateNotification {
    /// Render the notification banner
    pub(crate) fn render_banner(&self, info: &UpdateInfo, theme: &SimpleColors, cx: &Context<Self>) -> impl IntoElement {
        let version = info.version.clone();
        let version_for_skip = info.version.clone();
        let has_notes = !info.body.is_empty();

        // Extract listeners
        let on_toggle_notes = cx.listener(|this, _, _window, cx| {
            this.toggle_expanded(cx);
        });

        let on_skip_version = cx.listener(move |this, _, _window, cx| {
            this.skip_version(version_for_skip.clone(), cx);
            cx.emit(UpdateNotificationEvent::SkipVersion(version_for_skip.clone()));
        });

        let on_remind_later = cx.listener(|this, _, _window, cx| {
            this.dismiss(cx);
            cx.emit(UpdateNotificationEvent::RemindLater);
        });

        let on_update_now = cx.listener(|_this, _, _window, cx| {
            cx.emit(UpdateNotificationEvent::UpdateNow);
        });

        // Copy theme colors for closures
        let accent = theme.accent;
        let background = theme.background;
        let text = theme.text;
        let text_muted = theme.text_muted;
        let surface_hover = theme.surface_hover;
        let border = theme.border;
        let accent_hover = theme.accent_hover;

        div()
            .w_full()
            .bg(accent.opacity(0.1))
            .border_1()
            .border_color(accent.opacity(0.3))
            .rounded_md()
            .p_3()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap_4()
                    .child(
                        // Left side: icon and message
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                // Update icon
                                div()
                                    .w_8()
                                    .h_8()
                                    .rounded_full()
                                    .bg(accent)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div()
                                            .text_color(background)
                                            .text_sm()
                                            .font_weight(FontWeight::BOLD)
                                            .child("↑")
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_color(text)
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child(format!("Update Available: v{}", version))
                                    )
                                    .child(
                                        div()
                                            .text_color(text_muted)
                                            .text_xs()
                                            .child(if info.prerelease {
                                                "Pre-release version"
                                            } else {
                                                "Stable release"
                                            })
                                    )
                            )
                    )
                    .child(
                        // Right side: actions
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .when(has_notes, |this| {
                                this.child(
                                    div()
                                        .id("toggle-release-notes")
                                        .px_3()
                                        .py_1()
                                        .rounded_md()
                                        .cursor_pointer()
                                        .text_color(accent)
                                        .text_sm()
                                        .hover(|s| s.bg(surface_hover))
                                        .on_click(on_toggle_notes)
                                        .child(if self.expanded { "Hide Notes" } else { "View Notes" })
                                )
                            })
                            .child(
                                div()
                                    .id("skip-version-button")
                                    .px_3()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_color(text_muted)
                                    .text_sm()
                                    .hover(|s| s.bg(surface_hover))
                                    .on_click(on_skip_version)
                                    .child("Skip")
                            )
                            .child(
                                div()
                                    .id("remind-later-button")
                                    .px_3()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_color(text_muted)
                                    .text_sm()
                                    .hover(|s| s.bg(surface_hover))
                                    .on_click(on_remind_later)
                                    .child("Later")
                            )
                            .child(
                                div()
                                    .id("update-now-button")
                                    .px_4()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(accent)
                                    .text_color(background)
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .hover(|s| s.bg(accent_hover))
                                    .on_click(on_update_now)
                                    .child("Update Now")
                            )
                    )
            )
            .when(self.expanded && has_notes, |this| {
                this.child(
                    div()
                        .mt_3()
                        .pt_3()
                        .border_t_1()
                        .border_color(border)
                        .child(
                            div()
                                .text_color(text)
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .mb_2()
                                .child("Release Notes")
                        )
                        .child(
                            div()
                                .text_color(text_muted)
                                .text_xs()
                                .whitespace_nowrap()
                                .max_h_40()
                                .id("scroll-release-notes")
                                .overflow_y_scroll()
                                .child(info.body.clone())
                        )
                )
            })
    }
}
