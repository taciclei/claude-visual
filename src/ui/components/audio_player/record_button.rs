//! Audio recording button component

use gpui::*;
use gpui::prelude::*;

/// Audio recording button
#[derive(IntoElement)]
pub struct AudioRecordButton {
    id: ElementId,
    is_recording: bool,
    recording_time: f64,
    size: f32,
}

impl AudioRecordButton {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            is_recording: false,
            recording_time: 0.0,
            size: 48.0,
        }
    }

    pub fn is_recording(mut self, is_recording: bool) -> Self {
        self.is_recording = is_recording;
        self
    }

    pub fn recording_time(mut self, time: f64) -> Self {
        self.recording_time = time;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl RenderOnce for AudioRecordButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bg_color = if self.is_recording {
            hsla(0.0, 0.7, 0.5, 1.0)
        } else {
            hsla(0.0, 0.0, 0.2, 1.0)
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(12.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(self.size))
                    .h(px(self.size))
                    .bg(bg_color)
                    .rounded_full()
                    .cursor_pointer()
                    .child(
                        div()
                            .text_size(px(self.size * 0.4))
                            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                            .child(if self.is_recording { "⏹" } else { "🎤" })
                    )
            )
            .when(self.is_recording, |el| {
                el.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .child(
                            div()
                                .w(px(8.0))
                                .h(px(8.0))
                                .rounded_full()
                                .bg(hsla(0.0, 0.8, 0.5, 1.0))
                        )
                        .child(
                            div()
                                .text_size(px(14.0))
                                .font_weight(gpui::FontWeight::MEDIUM)
                                .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                                .child(format!(
                                    "{}:{:02}",
                                    (self.recording_time / 60.0).floor() as i32,
                                    (self.recording_time % 60.0).floor() as i32
                                ))
                        )
                )
            })
    }
}
