use super::radio::Radio;
use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// A group of radio buttons
#[derive(IntoElement)]
pub struct RadioGroup {
    name: SharedString,
    options: Vec<RadioOption>,
    selected: Option<SharedString>,
    orientation: RadioGroupOrientation,
    size: RadioSize,
    gap: f32,
    disabled: bool,
    error: Option<SharedString>,
    label: Option<SharedString>,
    required: bool,
    color: Option<Hsla>,
}

impl RadioGroup {
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            options: Vec::new(),
            selected: None,
            orientation: RadioGroupOrientation::Vertical,
            size: RadioSize::Medium,
            gap: 12.0,
            disabled: false,
            error: None,
            label: None,
            required: false,
            color: None,
        }
    }

    pub fn option(mut self, option: RadioOption) -> Self {
        self.options.push(option);
        self
    }

    pub fn options(mut self, options: impl IntoIterator<Item = RadioOption>) -> Self {
        self.options.extend(options);
        self
    }

    pub fn selected(mut self, value: impl Into<SharedString>) -> Self {
        self.selected = Some(value.into());
        self
    }

    pub fn orientation(mut self, orientation: RadioGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn size(mut self, size: RadioSize) -> Self {
        self.size = size;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn error(mut self, error: impl Into<SharedString>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
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

impl RenderOnce for RadioGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let has_error = self.error.is_some();

        let mut container = div().flex().flex_col().gap_2();

        // Label
        if let Some(label) = &self.label {
            let mut label_el = div()
                .flex()
                .items_center()
                .gap_1()
                .text_size(px(14.0))
                .text_color(Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.9,
                    a: 1.0,
                })
                .font_weight(gpui::FontWeight::MEDIUM)
                .child(label.clone());

            if self.required {
                label_el = label_el.child(
                    div()
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.7,
                            l: 0.5,
                            a: 1.0,
                        })
                        .child("*"),
                );
            }

            container = container.child(label_el);
        }

        // Options container
        let mut options_container = div().flex().gap(px(self.gap));

        options_container = match self.orientation {
            RadioGroupOrientation::Vertical => options_container.flex_col(),
            RadioGroupOrientation::Horizontal => options_container.flex_row().flex_wrap(),
        };

        for option in &self.options {
            let is_checked = self.selected.as_ref() == Some(&option.value);
            let is_disabled = self.disabled || option.disabled;

            let mut radio = Radio::new(option.value.clone(), option.label.clone())
                .checked(is_checked)
                .disabled(is_disabled)
                .size(self.size)
                .error(has_error);

            if let Some(desc) = &option.description {
                radio = radio.description(desc.clone());
            }

            if let Some(color) = self.color {
                radio = radio.color(color);
            }

            options_container = options_container.child(radio);
        }

        container = container.child(options_container);

        // Error message
        if let Some(error) = self.error {
            container = container.child(
                div()
                    .text_size(px(12.0))
                    .text_color(Hsla {
                        h: 0.0,
                        s: 0.7,
                        l: 0.5,
                        a: 1.0,
                    })
                    .child(error),
            );
        }

        container
    }
}
