//! Avatar stack component (vertical arrangement)

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Avatar stack (vertical arrangement)
#[derive(Clone)]
pub struct AvatarStack {
    avatars: Vec<GroupAvatar>,
    size: AvatarGroupSize,
    show_names: bool,
}

impl AvatarStack {
    pub fn new() -> Self {
        Self {
            avatars: Vec::new(),
            size: AvatarGroupSize::default(),
            show_names: true,
        }
    }

    pub fn avatars(mut self, avatars: Vec<GroupAvatar>) -> Self {
        self.avatars = avatars;
        self
    }

    pub fn size(mut self, size: AvatarGroupSize) -> Self {
        self.size = size;
        self
    }

    pub fn show_names(mut self, show: bool) -> Self {
        self.show_names = show;
        self
    }
}

impl Default for AvatarStack {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for AvatarStack {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let border_color = hsla(0.0, 0.0, 0.08, 1.0);
        let text = hsla(0.0, 0.0, 0.95, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.6, 1.0);

        let avatar_size = self.size.avatar_size();
        let show_names = self.show_names;

        div()
            .flex()
            .flex_col()
            .gap_2()
            .children(
                self.avatars.into_iter().enumerate().map(move |(idx, avatar)| {
                    let initials = avatar.get_initials();
                    let bg_color = avatar.get_color(idx);
                    let name = avatar.name.clone();
                    let is_online = avatar.is_online;

                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        // Avatar
                        .child(
                            div()
                                .relative()
                                .size(px(avatar_size))
                                .rounded_full()
                                .bg(bg_color)
                                .border_2()
                                .border_color(border_color)
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_color(text)
                                .font_weight(FontWeight::MEDIUM)
                                .child(initials)
                                // Online indicator
                                .when_some(is_online, |d, online| {
                                    d.child(
                                        div()
                                            .absolute()
                                            .bottom_0()
                                            .right_0()
                                            .size(px(avatar_size * 0.3))
                                            .rounded_full()
                                            .bg(if online {
                                                hsla(0.38, 0.7, 0.5, 1.0)
                                            } else {
                                                hsla(0.0, 0.0, 0.4, 1.0)
                                            })
                                            .border_2()
                                            .border_color(border_color)
                                    )
                                })
                        )
                        // Name
                        .when(show_names, |d| {
                            d.child(
                                div()
                                    .text_sm()
                                    .text_color(text_muted)
                                    .child(name)
                            )
                        })
                })
            )
    }
}
