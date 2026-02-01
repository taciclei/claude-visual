//! Mini bar chart component

use gpui::*;
use gpui::prelude::*;

/// Mini bar chart component
#[derive(IntoElement)]
pub struct MiniBarChart {
    id: ElementId,
    pub(crate) data: Vec<f64>,
    pub(crate) labels: Vec<SharedString>,
    height: f32,
    bar_width: f32,
    gap: f32,
    color: Option<gpui::Hsla>,
    pub(crate) show_labels: bool,
}

impl MiniBarChart {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            data: Vec::new(),
            labels: Vec::new(),
            height: 40.0,
            bar_width: 8.0,
            gap: 2.0,
            color: None,
            show_labels: false,
        }
    }

    pub fn data(mut self, data: Vec<f64>) -> Self {
        self.data = data;
        self
    }

    pub fn labels(mut self, labels: Vec<impl Into<SharedString>>) -> Self {
        self.labels = labels.into_iter().map(|l| l.into()).collect();
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn bar_width(mut self, width: f32) -> Self {
        self.bar_width = width;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn color(mut self, color: gpui::Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }
}

impl RenderOnce for MiniBarChart {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or(hsla(0.6, 0.7, 0.5, 1.0));
        let max_val = self.data.iter().copied().fold(f64::NEG_INFINITY, f64::max).max(0.001);

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .flex()
                    .items_end()
                    .gap(px(self.gap))
                    .h(px(self.height))
                    .children(self.data.iter().map(|&value| {
                        let normalized = (value / max_val) as f32;
                        let bar_height = (normalized * self.height).max(2.0);

                        div()
                            .w(px(self.bar_width))
                            .h(px(bar_height))
                            .bg(color)
                            .rounded(px(2.0))
                    }))
            )
            .when(self.show_labels && !self.labels.is_empty(), |el| {
                el.child(
                    div()
                        .flex()
                        .gap(px(self.gap))
                        .children(self.labels.iter().map(|label| {
                            div()
                                .w(px(self.bar_width))
                                .text_size(px(10.0))
                                .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                .child(label.clone())
                        }))
                )
            })
    }
}
