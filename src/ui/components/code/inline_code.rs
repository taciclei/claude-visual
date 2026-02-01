//! Inline code display component

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Inline code element
#[derive(Clone)]
pub struct InlineCode {
    pub(crate) code: String,
    pub(crate) size: CodeSize,
    pub(crate) copyable: bool,
}

impl InlineCode {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            size: CodeSize::default(),
            copyable: false,
        }
    }

    pub fn size(mut self, size: CodeSize) -> Self {
        self.size = size;
        self
    }

    pub fn copyable(mut self) -> Self {
        self.copyable = true;
        self
    }
}

impl RenderOnce for InlineCode {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bg = hsla(0.0, 0.0, 0.18, 1.0);
        let text = hsla(0.6, 0.5, 0.7, 1.0); // Slightly purple for code
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        let (px_h, py_v) = self.size.padding();

        let mut code_el = div()
            .px(px(px_h))
            .py(px(py_v))
            .bg(bg)
            .rounded(px(4.0))
            .border_1()
            .border_color(border)
            .text_color(text)
            .font_family("monospace")
            .child(self.code);

        if self.copyable {
            code_el = code_el
                .cursor_pointer()
                .hover(|s| s.bg(hsla(0.0, 0.0, 0.22, 1.0)));
        }

        code_el
    }
}
