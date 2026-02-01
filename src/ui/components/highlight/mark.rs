//! Mark/annotation component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Mark/annotation on text
#[derive(Clone)]
pub struct Mark {
    pub(crate) text: String,
    pub(crate) annotation: Option<String>,
    pub(crate) color: HighlightColor,
}

impl Mark {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            annotation: None,
            color: HighlightColor::Yellow,
        }
    }

    pub fn annotate(mut self, note: impl Into<String>) -> Self {
        self.annotation = Some(note.into());
        self
    }

    pub fn color(mut self, color: HighlightColor) -> Self {
        self.color = color;
        self
    }
}

impl RenderOnce for Mark {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text_color = hsla(0.0, 0.0, 0.9, 1.0);
        let bg = self.color.background();
        let annotation_color = hsla(0.0, 0.0, 0.6, 1.0);

        let mut container = div()
            .relative()
            .px_1()
            .rounded(px(2.0))
            .bg(bg)
            .text_color(text_color)
            .child(self.text);

        if let Some(note) = self.annotation {
            container = container.child(
                div()
                    .absolute()
                    .top(px(-16.0))
                    .left_0()
                    .px_1()
                    .py(px(1.0))
                    .bg(hsla(0.0, 0.0, 0.15, 1.0))
                    .rounded(px(2.0))
                    .text_xs()
                    .text_color(annotation_color)
                    .child(note),
            );
        }

        container
    }
}
