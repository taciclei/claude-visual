//! Pomodoro timer component

use gpui::prelude::*;
use gpui::*;

/// Pomodoro timer
#[derive(IntoElement)]
pub struct PomodoroTimer {
    id: ElementId,
    minutes_remaining: u32,
    seconds_remaining: u32,
    is_break: bool,
    session_number: u32,
    total_sessions: u32,
}

impl PomodoroTimer {
    pub fn new(id: impl Into<ElementId>, minutes: u32, seconds: u32) -> Self {
        Self {
            id: id.into(),
            minutes_remaining: minutes,
            seconds_remaining: seconds,
            is_break: false,
            session_number: 1,
            total_sessions: 4,
        }
    }

    pub fn is_break(mut self, is_break: bool) -> Self {
        self.is_break = is_break;
        self
    }

    pub fn session(mut self, current: u32, total: u32) -> Self {
        self.session_number = current;
        self.total_sessions = total;
        self
    }
}

impl RenderOnce for PomodoroTimer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = if self.is_break {
            hsla(0.35, 0.7, 0.45, 1.0)
        } else {
            hsla(0.0, 0.7, 0.5, 1.0)
        };

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .items_center()
            .gap(px(16.0))
            .child(
                // Session indicator
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .children((1..=self.total_sessions).map(|i| {
                        div()
                            .size(px(12.0))
                            .rounded_full()
                            .bg(if i <= self.session_number {
                                color
                            } else {
                                hsla(0.0, 0.0, 0.3, 1.0)
                            })
                    })),
            )
            .child(
                // Timer display
                div()
                    .size(px(200.0))
                    .rounded_full()
                    .border_4()
                    .border_color(color)
                    .bg(hsla(0.0, 0.0, 0.1, 1.0))
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_size(px(48.0))
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(hsla(0.0, 0.0, 0.95, 1.0))
                            .font_family("monospace")
                            .child(format!(
                                "{:02}:{:02}",
                                self.minutes_remaining, self.seconds_remaining
                            )),
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                            .child(if self.is_break { "Break" } else { "Focus" }),
                    ),
            )
    }
}
