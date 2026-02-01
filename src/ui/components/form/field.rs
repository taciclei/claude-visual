//! Form field wrapper with label and validation

use super::types::{FormLayout, FormSize, ValidationState};
use gpui::prelude::*;
use gpui::*;

#[derive(IntoElement)]
pub struct FormField {
    id: ElementId,
    label: Option<SharedString>,
    description: Option<SharedString>,
    required: bool,
    validation: ValidationState,
    error: Option<SharedString>,
    content: gpui::AnyElement,
    layout: FormLayout,
    size: FormSize,
    label_width: Option<f32>,
}

impl FormField {
    pub fn new(id: impl Into<ElementId>, content: impl IntoElement) -> Self {
        Self {
            id: id.into(),
            label: None,
            description: None,
            required: false,
            validation: ValidationState::None,
            error: None,
            content: content.into_any_element(),
            layout: FormLayout::Vertical,
            size: FormSize::default(),
            label_width: None,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn validation(mut self, state: ValidationState) -> Self {
        self.validation = state;
        self
    }

    pub fn error(mut self, error: impl Into<SharedString>) -> Self {
        self.error = Some(error.into());
        self.validation = ValidationState::Invalid;
        self
    }

    pub fn layout(mut self, layout: FormLayout) -> Self {
        self.layout = layout;
        self
    }

    pub fn size(mut self, size: FormSize) -> Self {
        self.size = size;
        self
    }

    pub fn label_width(mut self, width: f32) -> Self {
        self.label_width = Some(width);
        self
    }

    fn get_font_sizes(&self) -> (f32, f32) {
        match self.size {
            FormSize::Small => (12.0, 11.0),
            FormSize::Medium => (14.0, 12.0),
            FormSize::Large => (16.0, 14.0),
        }
    }
}

impl RenderOnce for FormField {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (label_size, helper_size) = self.get_font_sizes();

        let validation_color = match self.validation {
            ValidationState::None => hsla(0.0, 0.0, 0.5, 1.0),
            ValidationState::Valid => hsla(0.35, 0.7, 0.45, 1.0),
            ValidationState::Invalid => hsla(0.0, 0.7, 0.5, 1.0),
            ValidationState::Warning => hsla(0.12, 0.8, 0.5, 1.0),
        };

        let is_horizontal = self.layout == FormLayout::Horizontal;

        let mut field = div().id(self.id);

        field = if is_horizontal {
            field.flex().flex_row().items_start().gap(px(12.0))
        } else {
            field.flex().flex_col().gap(px(6.0))
        };

        // Label section
        if self.label.is_some() {
            let label_el = div()
                .when(self.label_width.is_some(), |el| {
                    el.w(px(self.label_width.unwrap()))
                })
                .when(is_horizontal, |el| el.pt(px(8.0)))
                .flex()
                .items_center()
                .gap(px(4.0))
                .child(
                    div()
                        .text_size(px(label_size))
                        .font_weight(gpui::FontWeight::MEDIUM)
                        .text_color(hsla(0.0, 0.0, 0.8, 1.0))
                        .child(self.label.unwrap_or_default()),
                )
                .when(self.required, |el| {
                    el.child(
                        div()
                            .text_size(px(label_size))
                            .text_color(hsla(0.0, 0.7, 0.5, 1.0))
                            .child("*"),
                    )
                });

            field = field.child(label_el);
        }

        // Content and validation section
        let content_section = div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .flex_1()
            .child(self.content)
            .when(
                self.description.is_some() && self.validation == ValidationState::None,
                |el| {
                    el.child(
                        div()
                            .text_size(px(helper_size))
                            .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                            .child(self.description.unwrap_or_default()),
                    )
                },
            )
            .when(self.error.is_some(), |el| {
                el.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(4.0))
                        .child(
                            div()
                                .text_size(px(helper_size))
                                .text_color(validation_color)
                                .child(match self.validation {
                                    ValidationState::Invalid => "⚠",
                                    ValidationState::Warning => "⚡",
                                    ValidationState::Valid => "✓",
                                    _ => "",
                                }),
                        )
                        .child(
                            div()
                                .text_size(px(helper_size))
                                .text_color(validation_color)
                                .child(self.error.unwrap_or_default()),
                        ),
                )
            });

        field.child(content_section)
    }
}
