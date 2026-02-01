//! Activity feed components

use gpui::prelude::*;
use gpui::*;

/// Activity feed item
#[derive(Clone)]
pub struct ActivityItem {
    pub(crate) avatar: String,
    pub(crate) user: String,
    pub(crate) action: String,
    pub(crate) target: Option<String>,
    pub(crate) time: String,
}

impl ActivityItem {
    pub fn new(
        avatar: impl Into<String>,
        user: impl Into<String>,
        action: impl Into<String>,
        time: impl Into<String>,
    ) -> Self {
        Self {
            avatar: avatar.into(),
            user: user.into(),
            action: action.into(),
            target: None,
            time: time.into(),
        }
    }

    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }
}

/// Activity feed
#[derive(Clone)]
pub struct ActivityFeed {
    pub(crate) items: Vec<ActivityItem>,
}

impl ActivityFeed {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn item(mut self, item: ActivityItem) -> Self {
        self.items.push(item);
        self
    }
}

impl Default for ActivityFeed {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ActivityFeed {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        div()
            .w_full()
            .flex()
            .flex_col()
            .children(self.items.into_iter().enumerate().map(|(idx, item)| {
                div()
                    .w_full()
                    .py_3()
                    .when(idx > 0, |d| d.border_t_1().border_color(border))
                    .flex()
                    .items_start()
                    .gap_3()
                    // Avatar
                    .child(
                        div()
                            .size(px(32.0))
                            .rounded_full()
                            .bg(accent.opacity(0.2))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_sm()
                            .child(item.avatar),
                    )
                    // Content
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap_1()
                            // Action text
                            .child(
                                div().text_sm().child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .items_center()
                                        .gap(px(4.0))
                                        .child(
                                            div()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(text)
                                                .child(item.user),
                                        )
                                        .child(div().text_color(text_muted).child(item.action))
                                        .when_some(item.target, |d, target| {
                                            d.child(
                                                div()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(accent)
                                                    .child(target),
                                            )
                                        }),
                                ),
                            )
                            // Time
                            .child(div().text_xs().text_color(text_muted).child(item.time)),
                    )
            }))
    }
}
