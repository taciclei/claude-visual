//! Avatar group component for displaying multiple avatars

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Avatar group displaying multiple avatars
#[derive(Clone, IntoElement)]
pub struct AvatarGroup {
    avatars: Vec<GroupAvatar>,
    max_visible: usize,
    size: AvatarGroupSize,
    bordered: bool,
    show_tooltip: bool,
}

impl AvatarGroup {
    pub fn new() -> Self {
        Self {
            avatars: Vec::new(),
            max_visible: 4,
            size: AvatarGroupSize::default(),
            bordered: true,
            show_tooltip: true,
        }
    }

    pub fn avatars(mut self, avatars: Vec<GroupAvatar>) -> Self {
        self.avatars = avatars;
        self
    }

    pub fn avatar(mut self, avatar: GroupAvatar) -> Self {
        self.avatars.push(avatar);
        self
    }

    pub fn max_visible(mut self, max: usize) -> Self {
        self.max_visible = max;
        self
    }

    pub fn size(mut self, size: AvatarGroupSize) -> Self {
        self.size = size;
        self
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    pub fn show_tooltip(mut self, show: bool) -> Self {
        self.show_tooltip = show;
        self
    }
}

impl Default for AvatarGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for AvatarGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.12, 1.0);
        let border_color = hsla(0.0, 0.0, 0.08, 1.0);
        let text = hsla(0.0, 0.0, 0.95, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.6, 1.0);

        let avatar_size = self.size.avatar_size();
        let overlap = self.size.overlap();
        let font_size = self.size.font_size();

        let total_count = self.avatars.len();
        let visible_count = total_count.min(self.max_visible);
        let overflow_count = total_count.saturating_sub(self.max_visible);
        let bordered = self.bordered;

        div()
            .flex()
            .items_center()
            .children(
                self.avatars
                    .into_iter()
                    .take(visible_count)
                    .enumerate()
                    .map(move |(idx, avatar)| {
                        let initials = avatar.get_initials();
                        let bg_color = avatar.get_color(idx);
                        let is_online = avatar.is_online;

                        let mut avatar_el = div()
                            .size(px(avatar_size))
                            .rounded_full()
                            .bg(bg_color)
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(text)
                            .font_weight(FontWeight::MEDIUM)
                            .relative();

                        // Apply negative margin for overlap (except first)
                        if idx > 0 {
                            avatar_el = avatar_el.ml(px(-overlap));
                        }

                        // Border for visual separation
                        if bordered {
                            avatar_el = avatar_el.border_2().border_color(border_color);
                        }

                        // Initials
                        avatar_el = avatar_el.child(div().text_color(text).child(initials));

                        // Online indicator
                        if let Some(online) = is_online {
                            let indicator_color = if online {
                                hsla(0.38, 0.7, 0.5, 1.0) // Green
                            } else {
                                hsla(0.0, 0.0, 0.4, 1.0) // Gray
                            };

                            avatar_el = avatar_el.child(
                                div()
                                    .absolute()
                                    .bottom_0()
                                    .right_0()
                                    .size(px(avatar_size * 0.3))
                                    .rounded_full()
                                    .bg(indicator_color)
                                    .border_2()
                                    .border_color(border_color),
                            );
                        }

                        avatar_el
                    }),
            )
            // Overflow indicator
            .when(overflow_count > 0, |d| {
                d.child(
                    div()
                        .size(px(avatar_size))
                        .ml(px(-overlap))
                        .rounded_full()
                        .bg(surface)
                        .border_2()
                        .border_color(border_color)
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(text_muted)
                        .font_weight(FontWeight::MEDIUM)
                        .child(format!("+{}", overflow_count)),
                )
            })
    }
}
