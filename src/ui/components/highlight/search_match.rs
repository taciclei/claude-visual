//! Search match highlight component

use gpui::*;
use gpui::prelude::*;

/// Search match highlight
#[derive(Clone)]
pub struct SearchMatch {
    text: String,
    query: String,
    case_sensitive: bool,
}

impl SearchMatch {
    pub fn new(text: impl Into<String>, query: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            query: query.into(),
            case_sensitive: false,
        }
    }

    pub fn case_sensitive(mut self) -> Self {
        self.case_sensitive = true;
        self
    }

    pub(crate) fn find_matches(&self) -> Vec<(usize, usize)> {
        let mut matches = Vec::new();

        let text = if self.case_sensitive {
            self.text.clone()
        } else {
            self.text.to_lowercase()
        };

        let query = if self.case_sensitive {
            self.query.clone()
        } else {
            self.query.to_lowercase()
        };

        if query.is_empty() {
            return matches;
        }

        let mut start = 0;
        while let Some(pos) = text[start..].find(&query) {
            let match_start = start + pos;
            let match_end = match_start + query.len();
            matches.push((match_start, match_end));
            start = match_end;
        }

        matches
    }
}

impl RenderOnce for SearchMatch {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text_color = hsla(0.0, 0.0, 0.9, 1.0);
        let highlight_bg = hsla(0.14, 0.9, 0.5, 0.4);

        let matches = self.find_matches();

        if matches.is_empty() {
            return div()
                .text_color(text_color)
                .child(self.text.clone())
                .into_any_element();
        }

        // Build segments
        let mut segments: Vec<(String, bool)> = Vec::new();
        let mut last_end = 0;

        for (start, end) in matches {
            if start > last_end {
                segments.push((self.text[last_end..start].to_string(), false));
            }
            segments.push((self.text[start..end].to_string(), true));
            last_end = end;
        }

        if last_end < self.text.len() {
            segments.push((self.text[last_end..].to_string(), false));
        }

        div()
            .flex()
            .flex_wrap()
            .text_color(text_color)
            .children(
                segments.into_iter().map(|(text, is_match)| {
                    if is_match {
                        div()
                            .px(px(1.0))
                            .rounded(px(2.0))
                            .bg(highlight_bg)
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
