//! Strikethrough text component

use gpui::prelude::*;
use gpui::*;

/// Strikethrough text
#[derive(Clone)]
pub struct Strikethrough {
    pub(crate) text: String,
    pub(crate) replacement: Option<String>,
}

impl Strikethrough {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            replacement: None,
        }
    }

    pub fn replaced_with(mut self, replacement: impl Into<String>) -> Self {
        self.replacement = Some(replacement.into());
        self
    }
}

impl RenderOnce for Strikethrough {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let green = hsla(0.38, 0.7, 0.5, 1.0);

        div()
            .flex()
            .items_center()
            .gap_2()
            .child(div().text_color(text_muted).line_through().child(self.text))
            .when_some(self.replacement, |d, repl| {
                d.child(div().text_color(green).child(repl))
            })
    }
}
