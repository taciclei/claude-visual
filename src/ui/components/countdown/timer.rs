//! Simple timer display

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Simple timer display
#[derive(IntoElement)]
pub struct Timer {
    id: ElementId,
    elapsed_seconds: u64,
    running: bool,
    size: CountdownSize,
    show_milliseconds: bool,
}

impl Timer {
    pub fn new(id: impl Into<ElementId>, elapsed_seconds: u64) -> Self {
        Self {
            id: id.into(),
            elapsed_seconds,
            running: false,
            size: CountdownSize::default(),
            show_milliseconds: false,
        }
    }

    pub fn running(mut self, running: bool) -> Self {
        self.running = running;
        self
    }

    pub fn size(mut self, size: CountdownSize) -> Self {
        self.size = size;
        self
    }

    pub fn show_milliseconds(mut self, show: bool) -> Self {
        self.show_milliseconds = show;
        self
    }
}

impl RenderOnce for Timer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let font_size = match self.size {
            CountdownSize::Small => 20.0,
            CountdownSize::Medium => 28.0,
            CountdownSize::Large => 40.0,
            CountdownSize::XLarge => 56.0,
        };

        let hours = self.elapsed_seconds / 3600;
        let minutes = (self.elapsed_seconds % 3600) / 60;
        let seconds = self.elapsed_seconds % 60;

        let time_str = if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{:02}:{:02}", minutes, seconds)
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(
                div()
                    .text_size(px(font_size))
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(if self.running {
                        hsla(0.35, 0.7, 0.5, 1.0)
                    } else {
                        hsla(0.0, 0.0, 0.8, 1.0)
                    })
                    .font_family("monospace")
                    .child(time_str),
            )
            .when(self.running, |el| {
                el.child(
                    div()
                        .size(px(8.0))
                        .rounded_full()
                        .bg(hsla(0.0, 0.8, 0.5, 1.0)),
                )
            })
    }
}
