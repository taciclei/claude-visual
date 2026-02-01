//! Grouped notifications by date component

use super::notification::Notification;
use gpui::prelude::*;
use gpui::*;

/// Grouped notifications by date
#[derive(Clone)]
pub struct NotificationGroup {
    date: String,
    notifications: Vec<Notification>,
}

impl NotificationGroup {
    pub fn new(date: impl Into<String>, notifications: Vec<Notification>) -> Self {
        Self {
            date: date.into(),
            notifications,
        }
    }
}

impl RenderOnce for NotificationGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        div()
            .w_full()
            .flex()
            .flex_col()
            .gap_2()
            // Date header
            .child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .gap_3()
                    .py_2()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(text_muted)
                            .child(self.date.to_uppercase()),
                    )
                    .child(div().flex_1().h(px(1.0)).bg(border)),
            )
            // Notifications
            .children(self.notifications)
    }
}
