//! Time picker component

use gpui::*;
use gpui::prelude::*;
use super::types::{TimeValue, DatePickerSize};

/// Time picker component
#[derive(IntoElement)]
pub struct TimePicker {
    pub(crate) id: ElementId,
    pub(crate) value: Option<TimeValue>,
    pub(crate) placeholder: SharedString,
    pub(crate) size: DatePickerSize,
    pub(crate) use_12_hour: bool,
    pub(crate) show_seconds: bool,
    pub(crate) disabled: bool,
    pub(crate) min_time: Option<TimeValue>,
    pub(crate) max_time: Option<TimeValue>,
    pub(crate) step: u8, // minute increment
    pub(crate) background: Option<gpui::Hsla>,
    pub(crate) border_color: Option<gpui::Hsla>,
}

impl TimePicker {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: None,
            placeholder: "Select time...".into(),
            size: DatePickerSize::default(),
            use_12_hour: false,
            show_seconds: false,
            disabled: false,
            min_time: None,
            max_time: None,
            step: 15,
            background: None,
            border_color: None,
        }
    }

    pub fn value(mut self, time: TimeValue) -> Self {
        self.value = Some(time);
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

    pub fn use_12_hour(mut self, use_12_hour: bool) -> Self {
        self.use_12_hour = use_12_hour;
        self
    }

    pub fn show_seconds(mut self, show: bool) -> Self {
        self.show_seconds = show;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn min_time(mut self, time: TimeValue) -> Self {
        self.min_time = Some(time);
        self
    }

    pub fn max_time(mut self, time: TimeValue) -> Self {
        self.max_time = Some(time);
        self
    }

    pub fn step(mut self, minutes: u8) -> Self {
        self.step = minutes;
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

impl RenderOnce for TimePicker {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (height, padding_x, font_size) = self.size.styles();

        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.15, 1.0));
        let border = self.border_color.unwrap_or(hsla(0.0, 0.0, 0.3, 1.0));

        let display_text: SharedString = self.value
            .map(|t| {
                if self.use_12_hour {
                    t.format_12h()
                } else {
                    t.format_24h()
                }.into()
            })
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
                div()
                    .text_size(px(font_size))
                    .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                    .child("🕐")
            )
            .child(
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
    }
}
