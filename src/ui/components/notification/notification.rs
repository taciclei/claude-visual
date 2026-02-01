//! Single notification component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Single notification
#[derive(Clone, IntoElement)]
pub struct Notification {
    /// Unique ID
    pub id: String,
    /// Title
    pub title: String,
    /// Message/body
    pub message: Option<String>,
    /// Type
    pub notification_type: NotificationType,
    /// Timestamp
    pub timestamp: String,
    /// Whether read
    pub read: bool,
    /// Actions
    pub actions: Vec<NotificationAction>,
    /// Avatar/icon
    pub avatar: Option<String>,
    /// Source/app name
    pub source: Option<String>,
}

impl Notification {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            message: None,
            notification_type: NotificationType::default(),
            timestamp: "Just now".to_string(),
            read: false,
            actions: Vec::new(),
            avatar: None,
            source: None,
        }
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn notification_type(mut self, notification_type: NotificationType) -> Self {
        self.notification_type = notification_type;
        self
    }

    pub fn timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = timestamp.into();
        self
    }

    pub fn read(mut self, read: bool) -> Self {
        self.read = read;
        self
    }

    pub fn action(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.actions.push(NotificationAction {
            id: id.into(),
            label: label.into(),
            primary: self.actions.is_empty(),
        });
        self
    }

    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.avatar = Some(avatar.into());
        self
    }

    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }
}

impl RenderOnce for Notification {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.18, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let unread_indicator = hsla(0.6, 0.8, 0.6, 1.0);

        let type_color = self.notification_type.color();
        let has_actions = !self.actions.is_empty();

        div()
            .w_full()
            .p_3()
            .bg(surface)
            .rounded(px(8.0))
            .border_1()
            .border_color(border)
            .flex()
            .gap_3()
            .cursor_pointer()
            .hover(|s| s.bg(surface_hover))
            // Unread indicator
            .when(!self.read, |d| {
                d.child(
                    div()
                        .w(px(8.0))
                        .h(px(8.0))
                        .rounded_full()
                        .bg(unread_indicator)
                        .mt_1(),
                )
            })
            .when(self.read, |d| d.child(div().w(px(8.0))))
            // Avatar or icon
            .child(
                div()
                    .size(px(36.0))
                    .rounded_full()
                    .bg(type_color.opacity(0.15))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(type_color)
                    .flex_shrink_0()
                    .child(
                        self.avatar
                            .unwrap_or_else(|| self.notification_type.icon().to_string()),
                    ),
            )
            // Content
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .overflow_hidden()
                    // Header row (title + timestamp)
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap_2()
                            .child(
                                div()
                                    .flex_1()
                                    .text_sm()
                                    .font_weight(if self.read {
                                        FontWeight::NORMAL
                                    } else {
                                        FontWeight::SEMIBOLD
                                    })
                                    .text_color(text)
                                    .overflow_hidden()
                                    .text_ellipsis()
                                    .child(self.title),
                            )
                            .child(
                                div()
                                    .flex_shrink_0()
                                    .text_xs()
                                    .text_color(text_muted)
                                    .child(self.timestamp),
                            ),
                    )
                    // Source (if any)
                    .when_some(self.source, |d, source| {
                        d.child(div().text_xs().text_color(text_muted).child(source))
                    })
                    // Message
                    .when_some(self.message, |d, msg| {
                        d.child(
                            div()
                                .text_sm()
                                .text_color(text_muted)
                                .line_clamp(2)
                                .child(msg),
                        )
                    })
                    // Actions
                    .when(has_actions, |d| {
                        d.child(div().pt_2().flex().items_center().gap_2().children(
                            self.actions.into_iter().map(|action| {
                                let mut btn = div()
                                    .px_3()
                                    .py_1()
                                    .rounded(px(4.0))
                                    .text_xs()
                                    .cursor_pointer();

                                if action.primary {
                                    btn = btn
                                        .bg(type_color)
                                        .text_color(gpui::white())
                                        .hover(|s| s.opacity(0.9));
                                } else {
                                    btn = btn
                                        .border_1()
                                        .border_color(border)
                                        .text_color(text_muted)
                                        .hover(|s| s.bg(surface_hover).text_color(text));
                                }

                                btn.child(action.label)
                            }),
                        ))
                    }),
            )
            // Dismiss button
            .child(
                div()
                    .size(px(24.0))
                    .rounded(px(4.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_xs()
                    .text_color(text_muted)
                    .cursor_pointer()
                    .hover(|s| s.bg(surface_hover).text_color(text))
                    .child("×"),
            )
    }
}
