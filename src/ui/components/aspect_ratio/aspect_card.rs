use gpui::*;
use gpui::prelude::*;
use super::types::Ratio;

/// Card with fixed aspect ratio header
#[derive(IntoElement)]
pub struct AspectCard {
    header_ratio: Ratio,
    header_content: Div,
    body: Div,
    width: f32,
    background: Option<Hsla>,
    header_background: Option<Hsla>,
}

impl AspectCard {
    pub fn new() -> Self {
        Self {
            header_ratio: Ratio::Video,
            header_content: div(),
            body: div(),
            width: 300.0,
            background: None,
            header_background: None,
        }
    }

    pub fn header_ratio(mut self, ratio: Ratio) -> Self {
        self.header_ratio = ratio;
        self
    }

    pub fn header(mut self, content: impl IntoElement) -> Self {
        self.header_content = div().child(content);
        self
    }

    pub fn body(mut self, content: impl IntoElement) -> Self {
        self.body = div().child(content);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn background(mut self, color: Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn header_background(mut self, color: Hsla) -> Self {
        self.header_background = Some(color);
        self
    }
}

impl Default for AspectCard {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for AspectCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let background = self.background.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.12,
            a: 1.0,
        });
        let header_bg = self.header_background.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.18,
            a: 1.0,
        });

        let header_height = self.width / self.header_ratio.value();

        div()
            .w(px(self.width))
            .bg(background)
            .rounded_lg()
            .overflow_hidden()
            .border_1()
            .border_color(Hsla {
                h: 0.0,
                s: 0.0,
                l: 0.25,
                a: 1.0,
            })
            .child(
                div()
                    .w(px(self.width))
                    .h(px(header_height))
                    .bg(header_bg)
                    .child(self.header_content),
            )
            .child(div().p_3().child(self.body))
    }
}
