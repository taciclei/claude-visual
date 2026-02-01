//! Notification center panel component

use super::notification::Notification;
use gpui::prelude::*;
use gpui::*;

/// Notification center panel
#[derive(Clone)]
pub struct NotificationCenter {
    notifications: Vec<Notification>,
    show_header: bool,
    show_clear_all: bool,
}

impl NotificationCenter {
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
            show_header: true,
            show_clear_all: true,
        }
    }

    pub fn notifications(mut self, notifications: Vec<Notification>) -> Self {
        self.notifications = notifications;
        self
    }

    pub fn show_header(mut self, show: bool) -> Self {
        self.show_header = show;
        self
    }

    pub fn show_clear_all(mut self, show: bool) -> Self {
        self.show_clear_all = show;
        self
    }
}

impl Default for NotificationCenter {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for NotificationCenter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.12, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);

        let unread_count = self.notifications.iter().filter(|n| !n.read).count();
        let has_notifications = !self.notifications.is_empty();

        div()
            .w(px(380.0))
            .max_h(px(500.0))
            .bg(surface)
            .rounded(px(12.0))
            .border_1()
            .border_color(border)
            .shadow_xl()
            .flex()
            .flex_col()
            // Header
            .when(self.show_header, |d| {
                d.child(
                    div()
                        .w_full()
                        .px_4()
                        .py_3()
                        .border_b_1()
                        .border_color(border)
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(
                                    div()
                                        .text_base()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(text)
                                        .child("Notifications"),
                                )
                                .when(unread_count > 0, |d| {
                                    d.child(
                                        div()
                                            .px_2()
                                            .py(px(2.0))
                                            .rounded_full()
                                            .bg(accent)
                                            .text_xs()
                                            .text_color(gpui::white())
                                            .child(format!("{}", unread_count)),
                                    )
                                }),
                        )
                        .when(self.show_clear_all && has_notifications, |d| {
                            d.child(
                                div()
                                    .text_xs()
                                    .text_color(accent)
                                    .cursor_pointer()
                                    .hover(|s| s.underline())
                                    .child("Clear all"),
                            )
                        }),
                )
            })
            // Notifications list
            .child(
                div()
                    .flex_1()
                    .id("scroll-notifications")
                    .overflow_y_scroll()
                    .when(has_notifications, |d| {
                        d.child(
                            div()
                                .w_full()
                                .p_2()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .children(self.notifications),
                        )
                    })
                    .when(!has_notifications, |d| {
                        d.child(
                            div()
                                .w_full()
                                .py_12()
                                .flex()
                                .flex_col()
                                .items_center()
                                .gap_2()
                                .child(div().text_3xl().text_color(text_muted).child("🔔"))
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(text_muted)
                                        .child("No notifications"),
                                ),
                        )
                    }),
            )
    }
}
