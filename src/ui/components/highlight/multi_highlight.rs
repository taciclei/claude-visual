//! Text with multiple highlights component

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Text with multiple highlights
#[derive(Clone)]
pub struct MultiHighlight {
    pub(crate) text: String,
    pub(crate) highlights: Vec<(usize, usize, HighlightColor)>,
}

impl MultiHighlight {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            highlights: Vec::new(),
        }
    }

    pub fn highlight(mut self, start: usize, end: usize, color: HighlightColor) -> Self {
        self.highlights.push((start, end, color));
        self
    }
}

impl RenderOnce for MultiHighlight {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text_color = hsla(0.0, 0.0, 0.9, 1.0);

        if self.highlights.is_empty() {
            return div()
                .text_color(text_color)
                .child(self.text.clone())
                .into_any_element();
        }

        // Sort highlights by start position
        let mut highlights = self.highlights.clone();
        highlights.sort_by_key(|(start, _, _)| *start);

        // Build segments
        let mut segments: Vec<(String, Option<HighlightColor>)> = Vec::new();
        let mut last_end = 0;

        for (start, end, color) in highlights {
            let start = start.min(self.text.len());
            let end = end.min(self.text.len());

            if start > last_end {
                segments.push((self.text[last_end..start].to_string(), None));
            }
            if start < end {
                segments.push((self.text[start..end].to_string(), Some(color)));
            }
            last_end = end;
        }

        if last_end < self.text.len() {
            segments.push((self.text[last_end..].to_string(), None));
        }

        div()
            .flex()
            .flex_wrap()
            .text_color(text_color)
            .children(
                segments.into_iter().map(|(text, color_opt)| {
                    if let Some(color) = color_opt {
                        div()
                            .px(px(1.0))
                            .rounded(px(2.0))
                            .bg(color.background())
                            .child(text)
                            .into_any_element()
                    } else {
                        div().child(text).into_any_element()
                    }
                })
            )
            .into_any_element()
    }
}
