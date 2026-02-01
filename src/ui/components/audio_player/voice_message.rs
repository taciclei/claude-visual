//! Voice message component

use gpui::*;
use gpui::prelude::*;
use super::types::PlaybackState;

/// Voice message component
#[derive(IntoElement)]
pub struct VoiceMessage {
    id: ElementId,
    duration: f64,
    current_time: f64,
    state: PlaybackState,
    waveform: Vec<f32>,
    is_outgoing: bool,
}

impl VoiceMessage {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            duration: 0.0,
            current_time: 0.0,
            state: PlaybackState::default(),
            waveform: Vec::new(),
            is_outgoing: false,
        }
    }

    pub fn duration(mut self, duration: f64) -> Self {
        self.duration = duration;
        self
    }

    pub fn current_time(mut self, time: f64) -> Self {
        self.current_time = time;
        self
    }

    pub fn state(mut self, state: PlaybackState) -> Self {
        self.state = state;
        self
    }

    pub fn waveform(mut self, waveform: Vec<f32>) -> Self {
        self.waveform = waveform;
        self
    }

    pub fn is_outgoing(mut self, is_outgoing: bool) -> Self {
        self.is_outgoing = is_outgoing;
        self
    }
}

impl RenderOnce for VoiceMessage {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let progress = if self.duration > 0.0 {
            (self.current_time / self.duration).clamp(0.0, 1.0) as f32
        } else {
            0.0
        };

        let play_icon = match self.state {
            PlaybackState::Playing => "⏸",
            _ => "▶",
        };

        let bg_color = if self.is_outgoing {
            hsla(0.6, 0.5, 0.4, 0.3)
        } else {
            hsla(0.0, 0.0, 0.15, 1.0)
        };

        let accent_color = if self.is_outgoing {
            hsla(0.6, 0.7, 0.6, 1.0)
        } else {
            hsla(0.6, 0.7, 0.5, 1.0)
        };

        // Generate default waveform if empty
        let waveform: Vec<f32> = if self.waveform.is_empty() {
            (0..30).map(|i| {
                let t = i as f32 / 30.0;
                (t * 10.0).sin().abs() * 0.5 + 0.2
            }).collect()
        } else {
            self.waveform.clone()
        };

        let progress_index = (progress * waveform.len() as f32) as usize;

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(10.0))
            .px(px(12.0))
            .py(px(8.0))
            .bg(bg_color)
            .rounded(px(16.0))
            .max_w(px(280.0))
            // Play button
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(32.0))
                    .h(px(32.0))
                    .bg(accent_color)
                    .rounded_full()
                    .cursor_pointer()
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                            .child(play_icon)
                    )
            )
            // Waveform
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(2.0))
                    .h(px(24.0))
                    .flex_1()
                    .children(waveform.iter().enumerate().map(|(i, &h)| {
                        let is_played = i < progress_index;
                        let bar_color = if is_played {
                            accent_color
                        } else {
                            hsla(0.0, 0.0, 0.3, 1.0)
                        };

                        div()
                            .w(px(3.0))
                            .h(px(h * 24.0))
                            .bg(bar_color)
                            .rounded(px(1.5))
                    }))
            )
            // Duration
            .child(
                div()
                    .text_size(px(11.0))
                    .text_color(hsla(0.0, 0.0, 0.6, 1.0))
                    .child(format!(
                        "{}:{:02}",
                        (self.duration / 60.0).floor() as i32,
                        (self.duration % 60.0).floor() as i32
                    ))
            )
    }
}
