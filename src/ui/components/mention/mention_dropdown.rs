//! Mention dropdown component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Mention dropdown - shows mentionable users
#[derive(IntoElement)]
pub struct MentionDropdown {
    id: ElementId,
    pub(crate) users: Vec<MentionableUser>,
    pub(crate) selected_index: Option<usize>,
    query: SharedString,
    max_height: f32,
}

impl MentionDropdown {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            users: Vec::new(),
            selected_index: None,
            query: "".into(),
            max_height: 240.0,
        }
    }

    pub fn users(mut self, users: Vec<MentionableUser>) -> Self {
        self.users = users;
        self
    }

    pub fn selected_index(mut self, index: usize) -> Self {
        self.selected_index = Some(index);
        self
    }

    pub fn query(mut self, query: impl Into<SharedString>) -> Self {
        self.query = query.into();
        self
    }

    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = height;
        self
    }
}

impl RenderOnce for MentionDropdown {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let filtered_users: Vec<_> = self
            .users
            .iter()
            .filter(|u| {
                let q = self.query.to_lowercase();
                u.name.to_lowercase().contains(&q) || u.username.to_lowercase().contains(&q)
            })
            .collect();

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .w(px(280.0))
            .max_h(px(self.max_height))
            .id("scroll-mention-dropdown")
            .overflow_y_scroll()
            .bg(hsla(0.0, 0.0, 0.12, 1.0))
            .border_1()
            .border_color(hsla(0.0, 0.0, 0.2, 1.0))
            .rounded(px(8.0))
            .shadow_lg()
            .when(filtered_users.is_empty(), |el| {
                el.child(
                    div()
                        .p(px(16.0))
                        .text_size(px(13.0))
                        .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                        .child("No users found"),
                )
            })
            .when(!filtered_users.is_empty(), |el| {
                el.children(filtered_users.iter().enumerate().map(|(i, user)| {
                    let is_selected = self.selected_index == Some(i);
                    let bg = if is_selected {
                        hsla(0.6, 0.5, 0.4, 0.2)
                    } else {
                        hsla(0.0, 0.0, 0.0, 0.0)
                    };

                    div()
                        .flex()
                        .items_center()
                        .gap(px(10.0))
                        .px(px(12.0))
                        .py(px(8.0))
                        .bg(bg)
                        .cursor_pointer()
                        // Avatar
                        .child(
                            div()
                                .relative()
                                .w(px(32.0))
                                .h(px(32.0))
                                .rounded_full()
                                .bg(hsla(0.0, 0.0, 0.25, 1.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                                        .child(user.avatar.clone().unwrap_or_else(|| {
                                            user.name
                                                .chars()
                                                .next()
                                                .map(|c| c.to_string())
                                                .unwrap_or_default()
                                                .into()
                                        })),
                                )
                                .when(user.is_online, |el| {
                                    el.child(
                                        div()
                                            .absolute()
                                            .bottom_0()
                                            .right_0()
                                            .w(px(10.0))
                                            .h(px(10.0))
                                            .rounded_full()
                                            .bg(hsla(0.35, 0.7, 0.45, 1.0))
                                            .border_2()
                                            .border_color(hsla(0.0, 0.0, 0.12, 1.0)),
                                    )
                                }),
                        )
                        // User info
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .flex_1()
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .font_weight(gpui::FontWeight::MEDIUM)
                                        .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                                        .child(user.name.clone()),
                                )
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                        .child(format!("@{}", user.username)),
                                ),
                        )
                        .when_some(user.status.clone(), |el, status| {
                            el.child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(hsla(0.0, 0.0, 0.4, 1.0))
                                    .child(status),
                            )
                        })
                }))
            })
    }
}
