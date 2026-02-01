//! GradientText component - animated gradient text

use gpui::*;
use gpui::prelude::*;

/// Gradient text animation
#[derive(IntoElement)]
pub struct GradientText {
    id: ElementId,
    text: SharedString,
    font_size: f32,
    font_weight: gpui::FontWeight,
    start_color: gpui::Hsla,
    end_color: gpui::Hsla,
    animate: bool,
}

impl GradientText {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            font_size: 24.0,
            font_weight: gpui::FontWeight::BOLD,
            start_color: hsla(0.6, 0.8, 0.6, 1.0),
            end_color: hsla(0.8, 0.8, 0.6, 1.0),
            animate: true,
        }
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn font_weight(mut self, weight: gpui::FontWeight) -> Self {
        self.font_weight = weight;
        self
    }

    pub fn colors(mut self, start: gpui::Hsla, end: gpui::Hsla) -> Self {
        self.start_color = start;
        self.end_color = end;
        self
    }

    pub fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }
}

impl RenderOnce for GradientText {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // Simplified gradient effect using character-by-character coloring
        let chars: Vec<char> = self.text.chars().collect();
        let len = chars.len().max(1) as f32;

        div()
            .id(self.id)
            .flex()
            .children(chars.into_iter().enumerate().map(|(i, c)| {
                let t = i as f32 / len;
                // Interpolate between start and end colors
                let h = self.start_color.h + (self.end_color.h - self.start_color.h) * t;
                let s = self.start_color.s + (self.end_color.s - self.start_color.s) * t;
                let l = self.start_color.l + (self.end_color.l - self.start_color.l) * t;
                let color = hsla(h, s, l, 1.0);

                div()
                    .text_size(px(self.font_size))
                    .font_weight(self.font_weight)
                    .text_color(color)
                    .child(c.to_string())
            }))
    }
}
