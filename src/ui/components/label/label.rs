use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// A form label component
#[derive(IntoElement)]
pub struct Label {
    text: SharedString,
    for_id: Option<SharedString>,
    size: LabelSize,
    required: bool,
    optional: bool,
    disabled: bool,
    color: Option<Hsla>,
    required_color: Option<Hsla>,
}

impl Label {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            for_id: None,
            size: LabelSize::Medium,
            required: false,
            optional: false,
            disabled: false,
            color: None,
            required_color: None,
        }
    }

    pub fn for_id(mut self, id: impl Into<SharedString>) -> Self {
        self.for_id = Some(id.into());
        self
    }

    pub fn size(mut self, size: LabelSize) -> Self {
        self.size = size;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn optional(mut self, optional: bool) -> Self {
        self.optional = optional;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn required_color(mut self, color: Hsla) -> Self {
        self.required_color = Some(color);
        self
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.9,
            a: 1.0,
        });
        let required_color = self.required_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.7,
            l: 0.55,
            a: 1.0,
        });
        let optional_color = Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.5,
            a: 1.0,
        };

        let mut label = div()
            .flex()
            .items_center()
            .gap_1()
            .text_size(px(self.size.font_size()))
            .text_color(color)
            .font_weight(gpui::FontWeight::MEDIUM);

        if self.disabled {
            label = label.opacity(0.5);
        }

        label = label.child(self.text);

        if self.required {
            label = label.child(
                div()
                    .text_color(required_color)
                    .text_size(px(self.size.font_size()))
                    .child("*"),
            );
        } else if self.optional {
            label = label.child(
                div()
                    .text_color(optional_color)
                    .text_size(px(self.size.font_size() - 1.0))
                    .child("(optional)"),
            );
        }

        label
    }
}
