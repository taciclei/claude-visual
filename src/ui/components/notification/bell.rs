//! Notification bell icon with badge component

use super::badge::NotificationBadge;
use gpui::prelude::*;
use gpui::*;

/// Notification bell icon with badge
#[derive(Clone)]
pub struct NotificationBell {
    count: usize,
}

impl NotificationBell {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}

impl RenderOnce for NotificationBell {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.18, 1.0);

        div()
            .relative()
            .size(px(36.0))
            .rounded(px(6.0))
            .flex()
            .items_center()
            .justify_center()
            .text_lg()
            .text_color(text)
            .cursor_pointer()
            .hover(|s| s.bg(surface_hover))
            .child("🔔")
            .when(self.count > 0, |d| {
                d.child(
                    div()
                        .absolute()
                        .top(px(-2.0))
                        .right(px(-2.0))
                        .child(NotificationBadge::new(self.count)),
                )
            })
    }
}
