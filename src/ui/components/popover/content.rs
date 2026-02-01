//! Simple popover content wrapper

use gpui::prelude::*;
use gpui::*;

/// Simple popover content wrapper
#[derive(Clone)]
pub struct PopoverContent {
    pub(crate) title: Option<String>,
    pub(crate) content: String,
    pub(crate) width: f32,
}

impl PopoverContent {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            title: None,
            content: content.into(),
            width: 200.0,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

impl RenderOnce for PopoverContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.6, 1.0);
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        div()
            .w(px(self.width))
            .rounded(px(8.0))
            .bg(surface)
            .border_1()
            .border_color(border)
            .shadow_lg()
            .p_3()
            .flex()
            .flex_col()
            .gap_2()
            .when_some(self.title, |d, title| {
                d.child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(text)
                        .child(title),
                )
            })
            .child(div().text_sm().text_color(text_muted).child(self.content))
    }
}
