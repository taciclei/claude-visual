//! Clock display component

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Clock display component
#[derive(IntoElement)]
pub struct Clock {
    id: ElementId,
    hours: u8,
    minutes: u8,
    seconds: Option<u8>,
    use_12_hour: bool,
    size: CountdownSize,
    show_seconds: bool,
    blinking_colon: bool,
    colon_visible: bool,
}

impl Clock {
    pub fn new(id: impl Into<ElementId>, hours: u8, minutes: u8) -> Self {
        Self {
            id: id.into(),
            hours,
            minutes,
            seconds: None,
            use_12_hour: false,
            size: CountdownSize::default(),
            show_seconds: false,
            blinking_colon: true,
            colon_visible: true,
        }
    }

    pub fn with_seconds(mut self, seconds: u8) -> Self {
        self.seconds = Some(seconds);
        self.show_seconds = true;
        self
    }

    pub fn use_12_hour(mut self, use_12_hour: bool) -> Self {
        self.use_12_hour = use_12_hour;
        self
    }

    pub fn size(mut self, size: CountdownSize) -> Self {
        self.size = size;
        self
    }

    pub fn show_seconds(mut self, show: bool) -> Self {
        self.show_seconds = show;
        self
    }

    pub fn blinking_colon(mut self, blink: bool) -> Self {
        self.blinking_colon = blink;
        self
    }

    pub fn colon_visible(mut self, visible: bool) -> Self {
        self.colon_visible = visible;
        self
    }
}

impl RenderOnce for Clock {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let font_size = match self.size {
            CountdownSize::Small => 24.0,
            CountdownSize::Medium => 36.0,
            CountdownSize::Large => 52.0,
            CountdownSize::XLarge => 72.0,
        };

        let (display_hours, period) = if self.use_12_hour {
            let (h, p) = if self.hours == 0 {
                (12, "AM")
            } else if self.hours < 12 {
                (self.hours, "AM")
            } else if self.hours == 12 {
                (12, "PM")
            } else {
                (self.hours - 12, "PM")
            };
            (h, Some(p))
        } else {
            (self.hours, None)
        };

        let colon_opacity = if self.blinking_colon && !self.colon_visible {
            0.0
        } else {
            1.0
        };

        div()
            .id(self.id)
            .flex()
            .items_baseline()
            .gap(px(4.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(font_size))
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(hsla(0.0, 0.0, 0.95, 1.0))
                            .font_family("monospace")
                            .child(format!("{:02}", display_hours))
                    )
                    .child(
                        div()
                            .text_size(px(font_size))
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(hsla(0.0, 0.0, 0.95, 1.0))
                            .opacity(colon_opacity)
                            .child(":")
                    )
                    .child(
                        div()
                            .text_size(px(font_size))
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(hsla(0.0, 0.0, 0.95, 1.0))
                            .font_family("monospace")
                            .child(format!("{:02}", self.minutes))
                    )
                    .when(self.show_seconds, |el| {
                        el.child(
                            div()
                                .text_size(px(font_size))
                                .font_weight(gpui::FontWeight::BOLD)
                                .text_color(hsla(0.0, 0.0, 0.95, 1.0))
                                .opacity(colon_opacity)
                                .child(":")
                        )
                        .child(
                            div()
                                .text_size(px(font_size))
                                .font_weight(gpui::FontWeight::BOLD)
                                .text_color(hsla(0.0, 0.0, 0.95, 1.0))
                                .font_family("monospace")
                                .child(format!("{:02}", self.seconds.unwrap_or(0)))
                        )
                    })
            )
            .when(period.is_some(), |el| {
                el.child(
                    div()
                        .text_size(px(font_size * 0.4))
                        .font_weight(gpui::FontWeight::MEDIUM)
                        .text_color(hsla(0.0, 0.0, 0.6, 1.0))
                        .ml(px(4.0))
                        .child(period.unwrap_or(""))
                )
            })
    }
}
