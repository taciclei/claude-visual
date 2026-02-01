//! Assignee picker component

use super::avatar_group::AvatarGroup;
use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Assignee picker display
#[derive(Clone)]
pub struct AssigneePicker {
    assigned: Vec<GroupAvatar>,
    available: Vec<GroupAvatar>,
    max_display: usize,
}

impl AssigneePicker {
    pub fn new() -> Self {
        Self {
            assigned: Vec::new(),
            available: Vec::new(),
            max_display: 3,
        }
    }

    pub fn assigned(mut self, avatars: Vec<GroupAvatar>) -> Self {
        self.assigned = avatars;
        self
    }

    pub fn available(mut self, avatars: Vec<GroupAvatar>) -> Self {
        self.available = avatars;
        self
    }

    pub fn max_display(mut self, max: usize) -> Self {
        self.max_display = max;
        self
    }
}

impl Default for AssigneePicker {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for AssigneePicker {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.2, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);

        div()
            .flex()
            .items_center()
            .gap_2()
            // Assigned avatars
            .when(!self.assigned.is_empty(), |d| {
                d.child(
                    AvatarGroup::new()
                        .avatars(self.assigned)
                        .max_visible(self.max_display)
                        .size(AvatarGroupSize::Small),
                )
            })
            // Add button
            .child(
                div()
                    .size(px(24.0))
                    .rounded_full()
                    .bg(surface)
                    .border_1()
                    .border_color(border)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_sm()
                    .text_color(text_muted)
                    .cursor_pointer()
                    .hover(|s| s.bg(surface_hover).text_color(text))
                    .child("+"),
            )
    }
}
