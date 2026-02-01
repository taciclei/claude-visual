use super::types::PopperDirection;
use gpui::prelude::*;
use gpui::*;

/// Party popper effect
#[derive(IntoElement)]
pub struct PartyPopper {
    id: ElementId,
    direction: PopperDirection,
    colors: Vec<gpui::Hsla>,
    active: bool,
    size: f32,
}

impl PartyPopper {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            direction: PopperDirection::default(),
            colors: vec![
                rgb(0xef4444).into(),
                rgb(0xeab308).into(),
                rgb(0x22c55e).into(),
                rgb(0x3b82f6).into(),
                rgb(0xa855f7).into(),
            ],
            active: false,
            size: 100.0,
        }
    }

    pub fn direction(mut self, direction: PopperDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn colors(mut self, colors: Vec<gpui::Hsla>) -> Self {
        self.colors = colors;
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl RenderOnce for PartyPopper {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        if !self.active {
            return div().id(self.id);
        }

        div()
            .id(self.id)
            .relative()
            .size(px(self.size))
            .overflow_hidden()
            .children(self.colors.iter().enumerate().map(|(i, &color)| {
                let offset = i as f32 * 15.0;
                div()
                    .absolute()
                    .left(px(self.size * 0.3 + offset))
                    .top(px(self.size * 0.3 - offset * 0.5))
                    .size(px(8.0))
                    .rounded_full()
                    .bg(color)
            }))
            .child(
                // Popper base
                div()
                    .absolute()
                    .bottom_0()
                    .left_0()
                    .w(px(20.0))
                    .h(px(30.0))
                    .bg(rgb(0xffd700))
                    .rounded_md(),
            )
    }
}
