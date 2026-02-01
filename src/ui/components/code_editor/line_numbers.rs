//! Line number gutter component

use gpui::prelude::*;
use gpui::*;

/// Line number gutter only
#[derive(IntoElement)]
pub struct LineNumbers {
    id: ElementId,
    total_lines: usize,
    current_line: Option<usize>,
    font_size: f32,
}

impl LineNumbers {
    pub fn new(id: impl Into<ElementId>, total_lines: usize) -> Self {
        Self {
            id: id.into(),
            total_lines,
            current_line: None,
            font_size: 13.0,
        }
    }

    pub fn current_line(mut self, line: usize) -> Self {
        self.current_line = Some(line);
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }
}

impl RenderOnce for LineNumbers {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let line_height = self.font_size * 1.5;
        let width = (self.total_lines.to_string().len() as f32 * self.font_size * 0.6) + 16.0;

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .w(px(width))
            .font_family("monospace")
            .children((1..=self.total_lines).map(|n| {
                let is_current = self.current_line == Some(n);
                div()
                    .flex()
                    .items_center()
                    .justify_end()
                    .h(px(line_height))
                    .pr(px(8.0))
                    .text_size(px(self.font_size))
                    .text_color(if is_current {
                        hsla(0.0, 0.0, 0.9, 1.0)
                    } else {
                        hsla(0.0, 0.0, 0.4, 1.0)
                    })
                    .child(n.to_string())
            }))
    }
}
