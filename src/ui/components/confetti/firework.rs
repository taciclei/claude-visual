use gpui::prelude::*;
use gpui::*;

/// Firework burst component
#[derive(IntoElement)]
pub struct Firework {
    id: ElementId,
    x: f32,
    y: f32,
    color: gpui::Hsla,
    particle_count: usize,
    radius: f32,
    active: bool,
    trail: bool,
}

impl Firework {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            x: 200.0,
            y: 200.0,
            color: rgb(0xff0000).into(),
            particle_count: 30,
            radius: 100.0,
            active: false,
            trail: true,
        }
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn color(mut self, color: gpui::Hsla) -> Self {
        self.color = color;
        self
    }

    pub fn particle_count(mut self, count: usize) -> Self {
        self.particle_count = count;
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn trail(mut self, trail: bool) -> Self {
        self.trail = trail;
        self
    }
}

impl RenderOnce for Firework {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        if !self.active {
            return div().id(self.id);
        }

        div()
            .id(self.id)
            .absolute()
            .left(px(self.x - self.radius))
            .top(px(self.y - self.radius))
            .size(px(self.radius * 2.0))
            .overflow_hidden()
            .flex()
            .items_center()
            .justify_center()
            .child(
                // Central burst
                div()
                    .size(px(self.radius))
                    .rounded_full()
                    .bg(self.color.opacity(0.3))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .size(px(self.radius * 0.5))
                            .rounded_full()
                            .bg(self.color),
                    ),
            )
    }
}
