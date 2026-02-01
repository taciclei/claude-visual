use gpui::prelude::*;
use gpui::*;

/// Sparkle effect overlay
#[derive(IntoElement)]
pub struct SparkleEffect {
    id: ElementId,
    sparkle_count: usize,
    active: bool,
    color: gpui::Hsla,
    size_range: (f32, f32),
    duration: u32,
}

impl SparkleEffect {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            sparkle_count: 15,
            active: false,
            color: rgb(0xffd700).into(),
            size_range: (4.0, 12.0),
            duration: 1500,
        }
    }

    pub fn sparkle_count(mut self, count: usize) -> Self {
        self.sparkle_count = count;
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn color(mut self, color: gpui::Hsla) -> Self {
        self.color = color;
        self
    }

    pub fn size_range(mut self, min: f32, max: f32) -> Self {
        self.size_range = (min, max);
        self
    }

    pub fn duration(mut self, ms: u32) -> Self {
        self.duration = ms;
        self
    }
}

impl RenderOnce for SparkleEffect {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        if !self.active {
            return div().id(self.id);
        }

        let avg_size = (self.size_range.0 + self.size_range.1) / 2.0;

        div()
            .id(self.id)
            .absolute()
            .inset_0()
            .overflow_hidden()
            .children((0..self.sparkle_count).map(|_i| {
                // Simple star/sparkle representation
                div()
                    .absolute()
                    .size(px(avg_size))
                    .bg(self.color)
                    .rounded_full()
            }))
    }
}
