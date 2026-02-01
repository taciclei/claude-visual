//! Countdown timer component

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Countdown timer component
#[derive(IntoElement)]
pub struct Countdown {
    id: ElementId,
    time: TimeRemaining,
    size: CountdownSize,
    variant: CountdownVariant,
    show_days: bool,
    show_labels: bool,
    separator: SharedString,
    finished_text: SharedString,
    urgent_threshold: Option<u64>,
    background: Option<gpui::Hsla>,
    text_color: Option<gpui::Hsla>,
    urgent_color: Option<gpui::Hsla>,
}

impl Countdown {
    pub fn new(id: impl Into<ElementId>, time: TimeRemaining) -> Self {
        Self {
            id: id.into(),
            time,
            size: CountdownSize::default(),
            variant: CountdownVariant::default(),
            show_days: true,
            show_labels: true,
            separator: ":".into(),
            finished_text: "Time's up!".into(),
            urgent_threshold: Some(60),
            background: None,
            text_color: None,
            urgent_color: None,
        }
    }

    pub fn from_seconds(id: impl Into<ElementId>, seconds: u64) -> Self {
        Self::new(id, TimeRemaining::from_seconds(seconds))
    }

    pub fn size(mut self, size: CountdownSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: CountdownVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn show_days(mut self, show: bool) -> Self {
        self.show_days = show;
        self
    }

    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    pub fn separator(mut self, separator: impl Into<SharedString>) -> Self {
        self.separator = separator.into();
        self
    }

    pub fn finished_text(mut self, text: impl Into<SharedString>) -> Self {
        self.finished_text = text.into();
        self
    }

    pub fn urgent_threshold(mut self, seconds: u64) -> Self {
        self.urgent_threshold = Some(seconds);
        self
    }

    pub fn no_urgent(mut self) -> Self {
        self.urgent_threshold = None;
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn text_color(mut self, color: gpui::Hsla) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn urgent_color(mut self, color: gpui::Hsla) -> Self {
        self.urgent_color = Some(color);
        self
    }

    fn get_size_styles(&self) -> (f32, f32, f32) {
        match self.size {
            CountdownSize::Small => (24.0, 10.0, 32.0),
            CountdownSize::Medium => (32.0, 12.0, 48.0),
            CountdownSize::Large => (48.0, 14.0, 64.0),
            CountdownSize::XLarge => (64.0, 16.0, 80.0),
        }
    }

    fn is_urgent(&self) -> bool {
        self.urgent_threshold
            .map(|threshold| self.time.total_seconds() <= threshold)
            .unwrap_or(false)
    }
}

impl RenderOnce for Countdown {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (font_size, label_size, box_size) = self.get_size_styles();
        let is_urgent = self.is_urgent();
        let is_finished = self.time.is_zero();

        let text_color = if is_urgent {
            self.urgent_color.unwrap_or(hsla(0.0, 0.7, 0.5, 1.0))
        } else {
            self.text_color.unwrap_or(hsla(0.0, 0.0, 0.95, 1.0))
        };

        if is_finished {
            return div()
                .id(self.id)
                .text_size(px(font_size))
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(self.urgent_color.unwrap_or(hsla(0.0, 0.7, 0.5, 1.0)))
                .child(self.finished_text.clone())
                .into_any_element();
        }

        let mut time_parts: Vec<(u32, &str)> = Vec::new();
        if self.show_days && self.time.days > 0 {
            time_parts.push((self.time.days, "Days"));
        }
        time_parts.push((self.time.hours, "Hours"));
        time_parts.push((self.time.minutes, "Min"));
        time_parts.push((self.time.seconds, "Sec"));

        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.12, 1.0));

        match self.variant {
            CountdownVariant::Default | CountdownVariant::Minimal => {
                div()
                    .id(self.id)
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .children(time_parts.iter().enumerate().map(|(i, (value, label))| {
                        div()
                            .flex()
                            .items_center()
                            .when(i > 0, |el| {
                                el.child(
                                    div()
                                        .text_size(px(font_size))
                                        .text_color(hsla(0.0, 0.0, 0.4, 1.0))
                                        .mx(px(4.0))
                                        .child(self.separator.clone())
                                )
                            })
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .child(
                                        div()
                                            .text_size(px(font_size))
                                            .font_weight(gpui::FontWeight::BOLD)
                                            .text_color(text_color)
                                            .child(format!("{:02}", value))
                                    )
                                    .when(self.show_labels && self.variant != CountdownVariant::Minimal, |el| {
                                        el.child(
                                            div()
                                                .text_size(px(label_size))
                                                .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                                .child(*label)
                                        )
                                    })
                            )
                    }))
                    .into_any_element()
            }
            CountdownVariant::Boxed => {
                div()
                    .id(self.id)
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .children(time_parts.iter().enumerate().map(|(i, (value, label))| {
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .when(i > 0, |el| {
                                el.child(
                                    div()
                                        .text_size(px(font_size * 0.8))
                                        .text_color(hsla(0.0, 0.0, 0.4, 1.0))
                                        .child(self.separator.clone())
                                )
                            })
                            .child(
                                div()
                                    .w(px(box_size))
                                    .py(px(8.0))
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .gap(px(4.0))
                                    .rounded(px(8.0))
                                    .bg(bg)
                                    .child(
                                        div()
                                            .text_size(px(font_size))
                                            .font_weight(gpui::FontWeight::BOLD)
                                            .text_color(text_color)
                                            .child(format!("{:02}", value))
                                    )
                                    .when(self.show_labels, |el| {
                                        el.child(
                                            div()
                                                .text_size(px(label_size))
                                                .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                                .child(*label)
                                        )
                                    })
                            )
                    }))
                    .into_any_element()
            }
            CountdownVariant::Circular => {
                div()
                    .id(self.id)
                    .flex()
                    .items_center()
                    .gap(px(12.0))
                    .children(time_parts.iter().map(|(value, label)| {
                        div()
                            .size(px(box_size))
                            .rounded_full()
                            .border_3()
                            .border_color(if is_urgent {
                                self.urgent_color.unwrap_or(hsla(0.0, 0.7, 0.5, 1.0))
                            } else {
                                hsla(0.6, 0.7, 0.5, 1.0)
                            })
                            .bg(bg)
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_size(px(font_size * 0.8))
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .text_color(text_color)
                                    .child(format!("{:02}", value))
                            )
                            .when(self.show_labels, |el| {
                                el.child(
                                    div()
                                        .text_size(px(label_size * 0.8))
                                        .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                        .child(*label)
                                )
                            })
                    }))
                    .into_any_element()
            }
        }
    }
}
