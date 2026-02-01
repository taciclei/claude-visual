//! Date picker input component

use gpui::*;
use gpui::prelude::*;
use super::types::{DateValue, DatePickerMode, DatePickerSize};

/// Date picker input component
#[derive(IntoElement)]
pub struct DatePicker {
    pub(crate) id: ElementId,
    pub(crate) value: Option<DateValue>,
    pub(crate) placeholder: SharedString,
    pub(crate) size: DatePickerSize,
    pub(crate) mode: DatePickerMode,
    pub(crate) disabled: bool,
    pub(crate) clearable: bool,
    pub(crate) min_date: Option<DateValue>,
    pub(crate) max_date: Option<DateValue>,
    pub(crate) open: bool,
    pub(crate) format: SharedString,
    pub(crate) background: Option<gpui::Hsla>,
    pub(crate) border_color: Option<gpui::Hsla>,
}

impl DatePicker {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: None,
            placeholder: "Select date...".into(),
            size: DatePickerSize::default(),
            mode: DatePickerMode::default(),
            disabled: false,
            clearable: true,
            min_date: None,
            max_date: None,
            open: false,
            format: "YYYY-MM-DD".into(),
            background: None,
            border_color: None,
        }
    }

    pub fn value(mut self, date: DateValue) -> Self {
        self.value = Some(date);
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn size(mut self, size: DatePickerSize) -> Self {
        self.size = size;
        self
    }

    pub fn mode(mut self, mode: DatePickerMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }

    pub fn min_date(mut self, date: DateValue) -> Self {
        self.min_date = Some(date);
        self
    }

    pub fn max_date(mut self, date: DateValue) -> Self {
        self.max_date = Some(date);
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn format(mut self, format: impl Into<SharedString>) -> Self {
        self.format = format.into();
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn border_color(mut self, color: gpui::Hsla) -> Self {
        self.border_color = Some(color);
        self
    }
}

impl RenderOnce for DatePicker {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (height, padding_x, font_size) = self.size.styles();
        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.15, 1.0));
        let border = self.border_color.unwrap_or(hsla(0.0, 0.0, 0.3, 1.0));

        let display_text = self.value
            .map(|d| d.format_display().into())
            .unwrap_or(self.placeholder.clone());
        let has_value = self.value.is_some();

        div()
            .id(self.id)
            .h(px(height))
            .px(px(padding_x))
            .flex()
            .items_center()
            .justify_between()
            .gap(px(8.0))
            .rounded(px(6.0))
            .border_1()
            .border_color(border)
            .bg(bg)
            .cursor_pointer()
            .when(self.disabled, |el| el.opacity(0.5).cursor_not_allowed())
            .child(
                // Calendar icon
                div()
                    .text_size(px(font_size))
                    .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                    .child("📅")
            )
            .child(
                // Date text
                div()
                    .flex_1()
                    .text_size(px(font_size))
                    .text_color(if has_value {
                        hsla(0.0, 0.0, 0.9, 1.0)
                    } else {
                        hsla(0.0, 0.0, 0.5, 1.0)
                    })
                    .child(display_text)
            )
            .when(has_value && self.clearable, |el| {
                el.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                        .cursor_pointer()
                        .child("✕")
                )
            })
    }
}
