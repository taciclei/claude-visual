//! Mention chip component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Mention chip component - displays a mentioned entity
#[derive(IntoElement)]
pub struct Mention {
    id: ElementId,
    name: SharedString,
    variant: MentionVariant,
    size: MentionSize,
    avatar: Option<SharedString>,
    is_self: bool,
    clickable: bool,
}

impl Mention {
    pub fn new(id: impl Into<ElementId>, name: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            variant: MentionVariant::default(),
            size: MentionSize::default(),
            avatar: None,
            is_self: false,
            clickable: true,
        }
    }

    pub fn variant(mut self, variant: MentionVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: MentionSize) -> Self {
        self.size = size;
        self
    }

    pub fn avatar(mut self, avatar: impl Into<SharedString>) -> Self {
        self.avatar = Some(avatar.into());
        self
    }

    pub fn is_self(mut self, is_self: bool) -> Self {
        self.is_self = is_self;
        self
    }

    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }
}

impl RenderOnce for Mention {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (prefix, bg_color, text_color) = match self.variant {
            MentionVariant::User => ("@", hsla(0.6, 0.5, 0.4, 0.2), hsla(0.6, 0.7, 0.6, 1.0)),
            MentionVariant::Channel => ("#", hsla(0.55, 0.5, 0.4, 0.2), hsla(0.55, 0.7, 0.6, 1.0)),
            MentionVariant::Team => ("@", hsla(0.8, 0.5, 0.4, 0.2), hsla(0.8, 0.7, 0.6, 1.0)),
            MentionVariant::Document => ("📄", hsla(0.1, 0.5, 0.4, 0.2), hsla(0.1, 0.7, 0.6, 1.0)),
            MentionVariant::Link => ("🔗", hsla(0.5, 0.5, 0.4, 0.2), hsla(0.5, 0.7, 0.6, 1.0)),
        };

        let bg = if self.is_self {
            hsla(0.15, 0.7, 0.5, 0.3)
        } else {
            bg_color
        };

        let text = if self.is_self {
            hsla(0.15, 0.8, 0.7, 1.0)
        } else {
            text_color
        };

        let font_size = self.size.font_size();
        let px_x = self.size.padding_x();
        let py = self.size.padding_y();

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(2.0))
            .px(px(px_x))
            .py(px(py))
            .bg(bg)
            .rounded(px(4.0))
            .when(self.clickable, |el| el.cursor_pointer())
            .when_some(self.avatar, |el, avatar| {
                let avatar_size = font_size + 4.0;
                el.child(
                    div()
                        .w(px(avatar_size))
                        .h(px(avatar_size))
                        .rounded_full()
                        .bg(hsla(0.0, 0.0, 0.3, 1.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(avatar_size * 0.6))
                        .child(avatar),
                )
            })
            .child(
                div()
                    .text_size(px(font_size))
                    .text_color(text)
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .child(format!("{}{}", prefix, self.name)),
            )
    }
}
