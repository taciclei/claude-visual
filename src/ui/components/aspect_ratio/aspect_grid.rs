use super::aspect_ratio::AspectRatio;
use gpui::prelude::*;
use gpui::*;

/// Grid of aspect-ratio maintained items
#[derive(IntoElement)]
pub struct AspectGrid {
    items: Vec<AspectRatio>,
    columns: usize,
    gap: f32,
}

impl AspectGrid {
    pub fn new(columns: usize) -> Self {
        Self {
            items: Vec::new(),
            columns,
            gap: 8.0,
        }
    }

    pub fn item(mut self, item: AspectRatio) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = AspectRatio>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }
}

impl RenderOnce for AspectGrid {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut container = div().flex().flex_wrap().gap(px(self.gap));

        for item in self.items {
            container = container.child(item);
        }

        container
    }
}
