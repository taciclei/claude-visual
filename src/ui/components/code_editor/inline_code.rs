//! Inline code snippet component

use gpui::prelude::*;
use gpui::*;

/// Inline code snippet
#[derive(IntoElement)]
pub struct InlineCode {
    id: ElementId,
    code: SharedString,
    language: Option<SharedString>,
}

impl InlineCode {
    pub fn new(id: impl Into<ElementId>, code: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            code: code.into(),
            language: None,
        }
    }

    pub fn language(mut self, lang: impl Into<SharedString>) -> Self {
        self.language = Some(lang.into());
        self
    }
}

impl RenderOnce for InlineCode {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .px(px(6.0))
            .py(px(2.0))
            .bg(hsla(0.0, 0.0, 0.15, 1.0))
            .rounded(px(4.0))
            .font_family("monospace")
            .text_size(px(13.0))
            .text_color(hsla(0.0, 0.7, 0.65, 1.0))
            .child(self.code.clone())
    }
}
