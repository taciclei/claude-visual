use gpui::*;
use gpui::prelude::*;
use super::types::Ratio;

/// Aspect ratio container that maintains proportions
#[derive(IntoElement)]
pub struct AspectRatio {
    ratio: Ratio,
    width: Option<f32>,
    content: Div,
    background: Option<Hsla>,
    border: bool,
    border_color: Option<Hsla>,
    rounded: bool,
}

impl AspectRatio {
    pub fn new(ratio: Ratio) -> Self {
        Self {
            ratio,
            width: None,
            content: div(),
            background: None,
            border: false,
            border_color: None,
            rounded: false,
        }
    }

    pub fn square() -> Self {
        Self::new(Ratio::Square)
    }

    pub fn video() -> Self {
        Self::new(Ratio::Video)
    }

    pub fn classic() -> Self {
        Self::new(Ratio::Classic)
    }

    pub fn portrait() -> Self {
        Self::new(Ratio::Portrait)
    }

    pub fn custom(ratio: f32) -> Self {
        Self::new(Ratio::Custom(ratio))
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.content = div().child(child);
        self
    }

    pub fn background(mut self, color: Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }

    pub fn border_color(mut self, color: Hsla) -> Self {
        self.border_color = Some(color);
        self
    }

    pub fn rounded(mut self, rounded: bool) -> Self {
        self.rounded = rounded;
        self
    }
}

impl RenderOnce for AspectRatio {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let border_color = self.border_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.3,
            a: 1.0,
        });

        let width = self.width.unwrap_or(300.0);
        let height = width / self.ratio.value();

        let mut container = div()
            .relative()
            .w(px(width))
            .h(px(height))
            .overflow_hidden();

        if let Some(bg) = self.background {
            container = container.bg(bg);
        }

        if self.border {
            container = container.border_1().border_color(border_color);
        }

        if self.rounded {
            container = container.rounded_lg();
        }

        // Content fills the container
        let content = div()
            .absolute()
            .inset_0()
            .child(self.content);

        container.child(content)
    }
}
