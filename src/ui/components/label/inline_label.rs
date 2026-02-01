use gpui::prelude::*;
use gpui::*;

/// Inline label for horizontal form layouts
#[derive(IntoElement)]
pub struct InlineLabel {
    label: SharedString,
    width: f32,
    required: bool,
    color: Option<Hsla>,
}

impl InlineLabel {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            width: 120.0,
            required: false,
            color: None,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl RenderOnce for InlineLabel {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.7,
            a: 1.0,
        });

        div()
            .w(px(self.width))
            .flex_shrink_0()
            .text_size(px(13.0))
            .text_color(color)
            .child(self.label)
            .when(self.required, |d| {
                d.child(
                    div()
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.7,
                            l: 0.55,
                            a: 1.0,
                        })
                        .ml(px(2.0))
                        .child("*"),
                )
            })
    }
}
