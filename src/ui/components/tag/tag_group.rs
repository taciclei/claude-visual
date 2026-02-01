//! TagGroup component for displaying multiple tags

use std::sync::Arc;
use gpui::prelude::*;
use gpui::*;

use crate::app::state::AppState;
use super::types::*;

/// A group of tags
pub struct TagGroup {
    app_state: Arc<AppState>,
    /// Tags in the group
    tags: Vec<TagGroupItem>,
    /// Size for all tags
    size: TagSize,
    /// Gap between tags
    gap: f32,
    /// Whether tags wrap to next line
    wrap: bool,
}

impl TagGroup {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            tags: Vec::new(),
            size: TagSize::Small,
            gap: 4.0,
            wrap: true,
        }
    }

    pub fn set_tags(&mut self, tags: Vec<TagGroupItem>, cx: &mut Context<Self>) {
        self.tags = tags;
        cx.notify();
    }

    pub fn set_size(&mut self, size: TagSize, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }

    pub fn set_gap(&mut self, gap: f32, cx: &mut Context<Self>) {
        self.gap = gap;
        cx.notify();
    }

    pub fn set_wrap(&mut self, wrap: bool, cx: &mut Context<Self>) {
        self.wrap = wrap;
        cx.notify();
    }
}

impl Render for TagGroup {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let height = self.size.height();
        let font_size = self.size.font_size();
        let padding_x = self.size.padding_x();

        div()
            .id("tag-group")
            .flex()
            .when(self.wrap, |d| d.flex_wrap())
            .gap(px(self.gap))
            .children(self.tags.iter().map(|item| {
                let (bg_color, fg_color) = match item.color {
                    TagColor::Default => (theme.colors.surface, theme.colors.text_muted),
                    TagColor::Primary => (theme.colors.accent.opacity(0.15), theme.colors.accent),
                    TagColor::Success => (theme.colors.success.opacity(0.15), theme.colors.success),
                    TagColor::Warning => (theme.colors.warning.opacity(0.15), theme.colors.warning),
                    TagColor::Error => (theme.colors.error.opacity(0.15), theme.colors.error),
                    TagColor::Info => (hsla(210.0 / 360.0, 0.8, 0.5, 0.15), hsla(210.0 / 360.0, 0.8, 0.5, 1.0)),
                    TagColor::Purple => (hsla(270.0 / 360.0, 0.6, 0.5, 0.15), hsla(270.0 / 360.0, 0.6, 0.5, 1.0)),
                    TagColor::Pink => (hsla(330.0 / 360.0, 0.7, 0.6, 0.15), hsla(330.0 / 360.0, 0.7, 0.6, 1.0)),
                };

                div()
                    .h(px(height))
                    .px(px(padding_x))
                    .flex()
                    .items_center()
                    .gap_1()
                    .rounded(px(height / 2.0))
                    .bg(bg_color)
                    .text_color(fg_color)
                    .text_size(px(font_size))
                    .font_weight(FontWeight::MEDIUM)
                    .when_some(item.icon.clone(), |d, icon| {
                        d.child(div().text_size(px(font_size - 2.0)).child(icon))
                    })
                    .child(item.label.clone())
            }))
    }
}
