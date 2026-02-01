use gpui::prelude::*;
use gpui::*;

/// Celebration emoji burst
#[derive(IntoElement)]
pub struct EmojiBurst {
    id: ElementId,
    emojis: Vec<String>,
    count: usize,
    active: bool,
    origin_x: f32,
    origin_y: f32,
    spread: f32,
    size: f32,
}

impl EmojiBurst {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            emojis: vec![
                "🎉".to_string(),
                "🎊".to_string(),
                "✨".to_string(),
                "⭐".to_string(),
                "🌟".to_string(),
            ],
            count: 20,
            active: false,
            origin_x: 0.5,
            origin_y: 0.5,
            spread: 200.0,
            size: 24.0,
        }
    }

    pub fn emojis(mut self, emojis: Vec<String>) -> Self {
        self.emojis = emojis;
        self
    }

    pub fn count(mut self, count: usize) -> Self {
        self.count = count;
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn origin(mut self, x: f32, y: f32) -> Self {
        self.origin_x = x;
        self.origin_y = y;
        self
    }

    pub fn spread(mut self, spread: f32) -> Self {
        self.spread = spread;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl RenderOnce for EmojiBurst {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        if !self.active {
            return div().id(self.id);
        }

        div()
            .id(self.id)
            .absolute()
            .inset_0()
            .overflow_hidden()
            .flex()
            .items_center()
            .justify_center()
            .children((0..self.count.min(self.emojis.len())).map(|i| {
                let emoji = &self.emojis[i % self.emojis.len()];
                div().text_size(px(self.size)).child(emoji.clone())
            }))
    }
}
