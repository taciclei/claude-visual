//! Avatar group component for displaying multiple avatars

use std::sync::Arc;

use gpui::*;
use gpui::prelude::*;

use crate::app::state::AppState;
use super::{types::*, avatar::Avatar};

#[derive(Clone)]
pub struct AvatarGroupItem {
    pub name: String,
    pub role: AvatarRole,
    pub presence: PresenceStatus,
}

/// Avatar group for showing multiple avatars with overlap
pub struct AvatarGroup {
    app_state: Arc<AppState>,
    /// Avatars to display
    avatars: Vec<AvatarGroupItem>,
    /// Maximum visible avatars
    max_visible: usize,
    /// Size for all avatars
    size: AvatarSize,
    /// Overlap amount (negative spacing)
    overlap: f32,
}

impl AvatarGroup {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            avatars: Vec::new(),
            max_visible: 5,
            size: AvatarSize::Small,
            overlap: 8.0,
        }
    }

    pub fn set_avatars(&mut self, avatars: Vec<AvatarGroupItem>, cx: &mut Context<Self>) {
        self.avatars = avatars;
        cx.notify();
    }

    pub fn set_max_visible(&mut self, max: usize, cx: &mut Context<Self>) {
        self.max_visible = max;
        cx.notify();
    }

    pub fn set_size(&mut self, size: AvatarSize, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }

    pub fn set_overlap(&mut self, overlap: f32, cx: &mut Context<Self>) {
        self.overlap = overlap;
        cx.notify();
    }
}

impl Render for AvatarGroup {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let avatar_size = self.size.pixels();
        let total = self.avatars.len();
        let visible_count = total.min(self.max_visible);
        let overflow = total.saturating_sub(self.max_visible);

        div()
            .id("avatar-group")
            .flex()
            .items_center()
            // Visible avatars
            .children((0..visible_count).map(|i| {
                let item = &self.avatars[i];
                let initials = Avatar::extract_initials(&item.name);

                let bg_color = match item.role {
                    AvatarRole::User => theme.colors.accent.opacity(0.2),
                    AvatarRole::Assistant => theme.colors.success.opacity(0.2),
                    _ => theme.colors.text_muted.opacity(0.2),
                };

                let fg_color = match item.role {
                    AvatarRole::User => theme.colors.accent,
                    AvatarRole::Assistant => theme.colors.success,
                    _ => theme.colors.text_muted,
                };

                div()
                    .when(i > 0, |d| d.ml(px(-self.overlap)))
                    .size(px(avatar_size))
                    .rounded_full()
                    .bg(bg_color)
                    .border_2()
                    .border_color(theme.colors.surface)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(fg_color)
                    .text_size(px(self.size.font_size()))
                    .font_weight(FontWeight::MEDIUM)
                    .child(initials)
            }))
            // Overflow indicator
            .when(overflow > 0, |this| {
                this.child(
                    div()
                        .ml(px(-self.overlap))
                        .size(px(avatar_size))
                        .rounded_full()
                        .bg(theme.colors.surface)
                        .border_2()
                        .border_color(theme.colors.border)
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(theme.colors.text_muted)
                        .text_size(px(self.size.font_size() * 0.8))
                        .font_weight(FontWeight::MEDIUM)
                        .child(format!("+{}", overflow))
                )
            })
    }
}
