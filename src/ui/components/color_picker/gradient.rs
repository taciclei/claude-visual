//! Gradient display

use gpui::*;
use gpui::prelude::*;

#[derive(Clone)]
pub struct GradientBar {
    start: Hsla,
    end: Hsla,
    height: f32,
}

impl GradientBar {
    pub fn new(start: Hsla, end: Hsla) -> Self {
        Self {
            start,
            end,
            height: 16.0,
        }
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
}

impl RenderOnce for GradientBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        // Approximate gradient with multiple divs
        let steps = 20;

        div()
            .w_full()
            .h(px(self.height))
            .rounded(px(4.0))
            .border_1()
            .border_color(border)
            .overflow_hidden()
            .flex()
            .children((0..steps).map(|i| {
                let t = i as f32 / (steps - 1) as f32;
                let h = self.start.h + (self.end.h - self.start.h) * t;
                let s = self.start.s + (self.end.s - self.start.s) * t;
                let l = self.start.l + (self.end.l - self.start.l) * t;
                let a = self.start.a + (self.end.a - self.start.a) * t;

                div()
                    .flex_1()
                    .h_full()
                    .bg(hsla(h, s, l, a))
            }))
    }
}
