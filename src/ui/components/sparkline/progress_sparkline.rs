//! Progress sparkline component

use gpui::*;
use gpui::prelude::*;

/// Progress sparkline - shows progress over time
#[derive(IntoElement)]
pub struct ProgressSparkline {
    id: ElementId,
    pub(crate) data: Vec<f64>,
    pub(crate) target: Option<f64>,
    height: f32,
    width: f32,
    color: Option<gpui::Hsla>,
    target_color: Option<gpui::Hsla>,
}

impl ProgressSparkline {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            data: Vec::new(),
            target: None,
            height: 32.0,
            width: 80.0,
            color: None,
            target_color: None,
        }
    }

    pub fn data(mut self, data: Vec<f64>) -> Self {
        self.data = data;
        self
    }

    pub fn target(mut self, target: f64) -> Self {
        self.target = Some(target);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn color(mut self, color: gpui::Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn target_color(mut self, color: gpui::Hsla) -> Self {
        self.target_color = Some(color);
        self
    }
}

impl RenderOnce for ProgressSparkline {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or(hsla(0.6, 0.7, 0.5, 1.0));
        let target_color = self.target_color.unwrap_or(hsla(0.0, 0.7, 0.5, 0.5));

        let max_val = self.data.iter().copied()
            .chain(self.target.into_iter())
            .fold(f64::NEG_INFINITY, f64::max)
            .max(0.001);

        let bar_count = self.data.len().max(1);
        let bar_width = self.width / bar_count as f32;

        div()
            .id(self.id)
            .relative()
            .h(px(self.height))
            .w(px(self.width))
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .flex()
                    .items_end()
                    .gap(px(1.0))
                    .children(self.data.iter().map(|&value| {
                        let normalized = (value / max_val) as f32;
                        let bar_height = (normalized * self.height).max(2.0);

                        div()
                            .w(px(bar_width - 1.0))
                            .h(px(bar_height))
                            .bg(color)
                            .rounded(px(1.0))
                    }))
            )
            .when_some(self.target, |el, target| {
                let target_y = ((target / max_val) as f32 * self.height).min(self.height);
                el.child(
                    div()
                        .absolute()
                        .left_0()
                        .right_0()
                        .bottom(px(target_y))
                        .h(px(1.0))
                        .bg(target_color)
                )
            })
    }
}
