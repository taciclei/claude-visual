//! DateTime picker combining date and time

use super::types::{DatePickerSize, DateValue, TimeValue};
use gpui::prelude::*;
use gpui::*;

/// DateTime picker combining date and time
#[derive(IntoElement)]
pub struct DateTimePicker {
    pub(crate) id: ElementId,
    pub(crate) date: Option<DateValue>,
    pub(crate) time: Option<TimeValue>,
    pub(crate) date_placeholder: SharedString,
    pub(crate) time_placeholder: SharedString,
    pub(crate) size: DatePickerSize,
    pub(crate) disabled: bool,
    pub(crate) background: Option<gpui::Hsla>,
    pub(crate) border_color: Option<gpui::Hsla>,
}

impl DateTimePicker {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            date: None,
            time: None,
            date_placeholder: "Select date".into(),
            time_placeholder: "Select time".into(),
            size: DatePickerSize::default(),
            disabled: false,
            background: None,
            border_color: None,
        }
    }

    pub fn date(mut self, date: DateValue) -> Self {
        self.date = Some(date);
        self
    }

    pub fn time(mut self, time: TimeValue) -> Self {
        self.time = Some(time);
        self
    }

    pub fn date_placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.date_placeholder = placeholder.into();
        self
    }

    pub fn time_placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.time_placeholder = placeholder.into();
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

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn border_color(mut self, color: gpui::Hsla) -> Self {
        self.border_color = Some(color);
        self
    }
}

impl RenderOnce for DateTimePicker {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (height, padding_x, font_size) = self.size.styles();

        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.15, 1.0));
        let border = self.border_color.unwrap_or(hsla(0.0, 0.0, 0.3, 1.0));

        let date_text: SharedString = self
            .date
            .map(|d| d.format_display().into())
            .unwrap_or(self.date_placeholder.clone());
        let time_text: SharedString = self
            .time
            .map(|t| t.format_24h().into())
            .unwrap_or(self.time_placeholder.clone());

        let has_date = self.date.is_some();
        let has_time = self.time.is_some();

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(
                // Date part
                div()
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
                            .text_color(if has_date {
                                hsla(0.0, 0.0, 0.9, 1.0)
                            } else {
                                hsla(0.0, 0.0, 0.5, 1.0)
                            })
                            .child(date_text),
                    ),
            )
            .child(
                // Time part
                div()
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
                            .child("🕐"),
                    )
                    .child(
                        div()
                            .text_size(px(font_size))
                            .text_color(if has_time {
                                hsla(0.0, 0.0, 0.9, 1.0)
                            } else {
                                hsla(0.0, 0.0, 0.5, 1.0)
                            })
                            .child(time_text),
                    ),
            )
    }
}
