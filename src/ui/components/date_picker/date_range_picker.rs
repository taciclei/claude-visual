//! Date range picker component

use super::types::{DatePickerSize, DateRange, DateValue};
use gpui::prelude::*;
use gpui::*;

/// Date range picker component
#[derive(IntoElement)]
pub struct DateRangePicker {
    pub(crate) id: ElementId,
    pub(crate) range: Option<DateRange>,
    pub(crate) start_placeholder: SharedString,
    pub(crate) end_placeholder: SharedString,
    pub(crate) size: DatePickerSize,
    pub(crate) disabled: bool,
    pub(crate) min_date: Option<DateValue>,
    pub(crate) max_date: Option<DateValue>,
    pub(crate) background: Option<gpui::Hsla>,
    pub(crate) border_color: Option<gpui::Hsla>,
}

impl DateRangePicker {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            range: None,
            start_placeholder: "Start date".into(),
            end_placeholder: "End date".into(),
            size: DatePickerSize::default(),
            disabled: false,
            min_date: None,
            max_date: None,
            background: None,
            border_color: None,
        }
    }

    pub fn range(mut self, range: DateRange) -> Self {
        self.range = Some(range);
        self
    }

    pub fn start_placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.start_placeholder = placeholder.into();
        self
    }

    pub fn end_placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.end_placeholder = placeholder.into();
        self
    }

    pub fn size(mut self, size: DatePickerSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn border_color(mut self, color: gpui::Hsla) -> Self {
        self.border_color = Some(color);
        self
    }
}

impl RenderOnce for DateRangePicker {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (height, padding_x, font_size) = self.size.styles();

        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.15, 1.0));
        let border = self.border_color.unwrap_or(hsla(0.0, 0.0, 0.3, 1.0));

        let (start_text, end_text) = if let Some(range) = &self.range {
            (
                SharedString::from(range.start.format_display()),
                SharedString::from(range.end.format_display()),
            )
        } else {
            (self.start_placeholder.clone(), self.end_placeholder.clone())
        };

        let has_value = self.range.is_some();

        div()
            .id(self.id)
            .h(px(height))
            .px(px(padding_x))
            .flex()
            .items_center()
            .gap(px(8.0))
            .rounded(px(6.0))
            .border_1()
            .border_color(border)
            .bg(bg)
            .cursor_pointer()
            .when(self.disabled, |el| el.opacity(0.5).cursor_not_allowed())
            .child(
                div()
                    .text_size(px(font_size))
                    .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                    .child("📅"),
            )
            .child(
                div()
                    .text_size(px(font_size))
                    .text_color(if has_value {
                        hsla(0.0, 0.0, 0.9, 1.0)
                    } else {
                        hsla(0.0, 0.0, 0.5, 1.0)
                    })
                    .child(start_text),
            )
            .child(
                div()
                    .text_size(px(font_size))
                    .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                    .child("→"),
            )
            .child(
                div()
                    .text_size(px(font_size))
                    .text_color(if has_value {
                        hsla(0.0, 0.0, 0.9, 1.0)
                    } else {
                        hsla(0.0, 0.0, 0.5, 1.0)
                    })
                    .child(end_text),
            )
    }
}
