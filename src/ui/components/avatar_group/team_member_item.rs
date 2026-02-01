//! Team member list item component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Team member list item
#[derive(Clone)]
pub struct TeamMemberItem {
    avatar: GroupAvatar,
    role: Option<String>,
    email: Option<String>,
}

impl TeamMemberItem {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            avatar: GroupAvatar::new(name),
            role: None,
            email: None,
        }
    }

    pub fn role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn online(mut self, is_online: bool) -> Self {
        self.avatar = self.avatar.online(is_online);
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.avatar = self.avatar.color(color);
        self
    }
}

impl RenderOnce for TeamMemberItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface_hover = hsla(0.0, 0.0, 0.15, 1.0);
        let border_color = hsla(0.0, 0.0, 0.08, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);

        let initials = self.avatar.get_initials();
        let bg_color = self.avatar.get_color(0);
        let name = self.avatar.name.clone();
        let is_online = self.avatar.is_online;

        div()
            .w_full()
            .px_3()
            .py_2()
            .flex()
            .items_center()
            .gap_3()
            .rounded(px(6.0))
            .cursor_pointer()
            .hover(|s| s.bg(surface_hover))
            // Avatar
            .child(
                div()
                    .relative()
                    .size(px(36.0))
                    .rounded_full()
                    .bg(bg_color)
                    .border_2()
                    .border_color(border_color)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(text)
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .child(initials)
                    .when_some(is_online, |d, online| {
                        d.child(
                            div()
                                .absolute()
                                .bottom_0()
                                .right_0()
                                .size(px(10.0))
                                .rounded_full()
                                .bg(if online {
                                    hsla(0.38, 0.7, 0.5, 1.0)
                                } else {
                                    hsla(0.0, 0.0, 0.4, 1.0)
                                })
                                .border_2()
                                .border_color(border_color),
                        )
                    }),
            )
            // Info
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(text)
                            .child(name),
                    )
                    .when_some(self.role, |d, role| {
                        d.child(div().text_xs().text_color(text_muted).child(role))
                    }),
            )
            // Email
            .when_some(self.email, |d, email| {
                d.child(div().text_xs().text_color(text_muted).child(email))
            })
    }
}
