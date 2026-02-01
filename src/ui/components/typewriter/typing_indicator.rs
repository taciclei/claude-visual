//! TypingIndicator component - three dots animation

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Typing indicator (three dots animation)
#[derive(IntoElement)]
pub struct TypingIndicator {
    id: ElementId,
    pub(crate) variant: TypingIndicatorVariant,
    pub(crate) size: TypingIndicatorSize,
    label: Option<SharedString>,
    dot_color: gpui::Hsla,
    active_dot: usize,
}

impl TypingIndicator {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            variant: TypingIndicatorVariant::default(),
            size: TypingIndicatorSize::default(),
            label: None,
            dot_color: rgba(0x888888ff).into(),
            active_dot: 0,
        }
    }

    pub fn variant(mut self, variant: TypingIndicatorVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: TypingIndicatorSize) -> Self {
        self.size = size;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn dot_color(mut self, color: gpui::Hsla) -> Self {
        self.dot_color = color;
        self
    }

    pub fn active_dot(mut self, index: usize) -> Self {
        self.active_dot = index % 3;
        self
    }

    fn dot_size(&self) -> f32 {
        match self.size {
            TypingIndicatorSize::Sm => 6.0,
            TypingIndicatorSize::Md => 8.0,
            TypingIndicatorSize::Lg => 10.0,
        }
    }

    fn render_dot(&self, index: usize) -> Div {
        let size = self.dot_size();
        let is_active = index == self.active_dot;
        let opacity = if is_active { 1.0 } else { 0.4 };

        let base = div()
            .size(px(size))
            .rounded_full()
            .bg(self.dot_color.opacity(opacity));

        match self.variant {
            TypingIndicatorVariant::Dots => base,
            TypingIndicatorVariant::Wave => {
                let offset = if is_active { -4.0 } else { 0.0 };
                base.mt(px(offset))
            }
            TypingIndicatorVariant::Pulse => {
                let scale = if is_active { 1.3 } else { 1.0 };
                base.size(px(size * scale))
            }
            TypingIndicatorVariant::Bounce => {
                let offset = if is_active { -6.0 } else { 0.0 };
                base.mt(px(offset))
            }
        }
    }
}

impl RenderOnce for TypingIndicator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let gap = match self.size {
            TypingIndicatorSize::Sm => 4.0,
            TypingIndicatorSize::Md => 6.0,
            TypingIndicatorSize::Lg => 8.0,
        };
        let id = self.id.clone();

        div()
            .id(id)
            .flex()
            .items_center()
            .gap(px(gap))
            .when_some(self.label.clone(), |d, label| {
                d.child(
                    div()
                        .text_sm()
                        .text_color(rgba(0x888888ff))
                        .mr_2()
                        .child(label),
                )
            })
            .child(self.render_dot(0))
            .child(self.render_dot(1))
            .child(self.render_dot(2))
    }
}
