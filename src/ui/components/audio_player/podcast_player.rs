//! Podcast player component

use super::types::PlaybackState;
use crate::ui::pct;
use gpui::prelude::*;
use gpui::*;

/// Podcast player component
#[derive(IntoElement)]
pub struct PodcastPlayer {
    id: ElementId,
    title: SharedString,
    podcast_name: SharedString,
    artwork: Option<SharedString>,
    duration: f64,
    current_time: f64,
    state: PlaybackState,
    playback_speed: f32,
}

impl PodcastPlayer {
    pub fn new(
        id: impl Into<ElementId>,
        title: impl Into<SharedString>,
        podcast_name: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            podcast_name: podcast_name.into(),
            artwork: None,
            duration: 0.0,
            current_time: 0.0,
            state: PlaybackState::default(),
            playback_speed: 1.0,
        }
    }

    pub fn artwork(mut self, artwork: impl Into<SharedString>) -> Self {
        self.artwork = Some(artwork.into());
        self
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

    pub fn playback_speed(mut self, speed: f32) -> Self {
        self.playback_speed = speed;
        self
    }
}

impl RenderOnce for PodcastPlayer {
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

        div()
            .id(self.id)
            .flex()
            .gap(px(16.0))
            .p(px(16.0))
            .bg(hsla(0.0, 0.0, 0.1, 1.0))
            .border_1()
            .border_color(hsla(0.0, 0.0, 0.2, 1.0))
            .rounded(px(12.0))
            // Artwork
            .child(
                div()
                    .w(px(80.0))
                    .h(px(80.0))
                    .rounded(px(8.0))
                    .bg(hsla(0.0, 0.0, 0.2, 1.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_size(px(32.0))
                            .child(self.artwork.unwrap_or("🎙️".into())),
                    ),
            )
            // Content
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .gap(px(8.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .text_color(hsla(0.0, 0.0, 0.95, 1.0))
                                    .text_ellipsis()
                                    .child(self.title.clone()),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(hsla(0.0, 0.0, 0.6, 1.0))
                                    .child(self.podcast_name.clone()),
                            ),
                    )
                    // Progress
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .h(px(4.0))
                                    .bg(hsla(0.0, 0.0, 0.2, 1.0))
                                    .rounded_full()
                                    .cursor_pointer()
                                    .child(
                                        div()
                                            .h_full()
                                            .w(pct(progress * 100.0))
                                            .bg(hsla(0.6, 0.7, 0.5, 1.0))
                                            .rounded_full(),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .justify_between()
                                    .text_size(px(11.0))
                                    .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                    .child(format!(
                                        "{}:{:02}",
                                        (self.current_time / 60.0).floor() as i32,
                                        (self.current_time % 60.0).floor() as i32
                                    ))
                                    .child(format!(
                                        "-{}:{:02}",
                                        ((self.duration - self.current_time) / 60.0).floor() as i32,
                                        ((self.duration - self.current_time) % 60.0).floor() as i32
                                    )),
                            ),
                    )
                    // Controls
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(16.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(hsla(0.0, 0.0, 0.7, 1.0))
                                            .cursor_pointer()
                                            .child("-15s"),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .w(px(44.0))
                                            .h(px(44.0))
                                            .bg(hsla(0.6, 0.7, 0.5, 1.0))
                                            .rounded_full()
                                            .cursor_pointer()
                                            .child(
                                                div()
                                                    .text_size(px(18.0))
                                                    .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                                                    .child(play_icon),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(hsla(0.0, 0.0, 0.7, 1.0))
                                            .cursor_pointer()
                                            .child("+30s"),
                                    ),
                            )
                            .child(
                                div()
                                    .px(px(8.0))
                                    .py(px(4.0))
                                    .bg(hsla(0.0, 0.0, 0.15, 1.0))
                                    .rounded(px(4.0))
                                    .text_size(px(12.0))
                                    .font_weight(gpui::FontWeight::MEDIUM)
                                    .text_color(hsla(0.0, 0.0, 0.7, 1.0))
                                    .cursor_pointer()
                                    .child(format!("{}x", self.playback_speed)),
                            ),
                    ),
            )
    }
}
