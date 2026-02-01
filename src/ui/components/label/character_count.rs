use gpui::*;
use gpui::prelude::*;

/// Character counter for text inputs
#[derive(IntoElement)]
pub struct CharacterCount {
    current: usize,
    max: usize,
    show_remaining: bool,
    warning_threshold: Option<usize>,
    color: Option<Hsla>,
    warning_color: Option<Hsla>,
    error_color: Option<Hsla>,
}

impl CharacterCount {
    pub fn new(current: usize, max: usize) -> Self {
        Self {
            current,
            max,
            show_remaining: false,
            warning_threshold: None,
            color: None,
            warning_color: None,
            error_color: None,
        }
    }

    pub fn show_remaining(mut self, show: bool) -> Self {
        self.show_remaining = show;
        self
    }

    pub fn warning_threshold(mut self, threshold: usize) -> Self {
        self.warning_threshold = Some(threshold);
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn warning_color(mut self, color: Hsla) -> Self {
        self.warning_color = Some(color);
        self
    }

    pub fn error_color(mut self, color: Hsla) -> Self {
        self.error_color = Some(color);
        self
    }
}

impl RenderOnce for CharacterCount {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let normal_color = self.color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.5,
            a: 1.0,
        });
        let warning_color = self.warning_color.unwrap_or(Hsla {
            h: 0.12,
            s: 0.9,
            l: 0.5,
            a: 1.0,
        });
        let error_color = self.error_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.7,
            l: 0.55,
            a: 1.0,
        });

        let color = if self.current > self.max {
            error_color
        } else if let Some(threshold) = self.warning_threshold {
            if self.current >= threshold {
                warning_color
            } else {
                normal_color
            }
        } else {
            normal_color
        };

        let text = if self.show_remaining {
            format!("{} remaining", self.max.saturating_sub(self.current))
        } else {
            format!("{}/{}", self.current, self.max)
        };

        div()
            .text_size(px(11.0))
            .text_color(color)
            .child(text)
    }
}
