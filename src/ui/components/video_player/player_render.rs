//! VideoPlayer render implementation

use gpui::*;
use gpui::prelude::*;
use crate::ui::pct;

use super::types::{PlaybackState, VideoPlayerSize, VideoQuality};
use super::controls::{format_time};
use super::{VideoPlayer, overlay, progress_bar};

impl RenderOnce for VideoPlayer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (width, height) = if self.size == VideoPlayerSize::Full {
            (0.0, 0.0)
        } else {
            self.size.dimensions()
        };

        let progress = if self.duration > 0.0 {
            (self.current_time / self.duration).clamp(0.0, 1.0) as f32
        } else {
            0.0
        };

        let play_icon = match self.state {
            PlaybackState::Playing => "⏸",
            PlaybackState::Loading => "⏳",
            _ => "▶",
        };

        let volume_icon = if self.muted || self.volume == 0.0 {
            "🔇"
        } else if self.volume < 0.5 {
            "🔉"
        } else {
            "🔊"
        };

        div()
            .id(self.id)
            .relative()
            .when(self.size != VideoPlayerSize::Full, |el| {
                el.w(px(width)).h(px(height))
            })
            .when(self.size == VideoPlayerSize::Full, |el| {
                el.w_full()
            })
            .bg(hsla(0.0, 0.0, 0.05, 1.0))
            .rounded(px(8.0))
            .overflow_hidden()
            // Video area / thumbnail
            .child(build_video_area(self.thumbnail.clone()))
            // Play overlay (when paused)
            .children(overlay::build_play_overlay(self.state, self.show_controls, play_icon.to_string()))
            // Controls overlay
            .when(self.show_controls, |el| {
                el.child(build_controls_overlay(
                    progress,
                    self.buffered,
                    play_icon.to_string(),
                    volume_icon.to_string(),
                    self.muted,
                    self.volume,
                    self.current_time,
                    self.duration,
                    self.quality,
                    self.fullscreen,
                ))
            })
    }
}

/// Build the video area with thumbnail
fn build_video_area(thumbnail: Option<SharedString>) -> Div {
    div()
        .absolute()
        .inset_0()
        .flex()
        .items_center()
        .justify_center()
        .bg(hsla(0.0, 0.0, 0.1, 1.0))
        .child(
            div()
                .text_size(px(48.0))
                .text_color(hsla(0.0, 0.0, 0.3, 1.0))
                .child(thumbnail.unwrap_or("🎬".into()))
        )
}

/// Build the controls overlay with progress bar and buttons
fn build_controls_overlay(
    progress: f32,
    buffered: f32,
    play_icon: String,
    volume_icon: String,
    muted: bool,
    volume: f32,
    current_time: f64,
    duration: f64,
    quality: VideoQuality,
    fullscreen: bool,
) -> Div {
    div()
        .absolute()
        .bottom_0()
        .left_0()
        .right_0()
        .flex()
        .flex_col()
        .bg(hsla(0.0, 0.0, 0.0, 0.7))
        .p(px(12.0))
        // Progress bar
        .child(progress_bar::build_progress_bar(progress, buffered))
        // Control buttons
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(build_left_controls(play_icon.clone(), volume_icon, muted, volume, current_time, duration))
                .child(build_right_controls(quality, fullscreen))
        )
}

/// Build left side controls (play, volume, time)
fn build_left_controls(
    play_icon: String,
    volume_icon: String,
    muted: bool,
    volume: f32,
    current_time: f64,
    duration: f64,
) -> Div {
    div()
        .flex()
        .items_center()
        .gap(px(12.0))
        // Play/pause
        .child(
            div()
                .text_size(px(16.0))
                .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                .cursor_pointer()
                .child(play_icon)
        )
        // Volume
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(6.0))
                .child(
                    div()
                        .text_size(px(14.0))
                        .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                        .cursor_pointer()
                        .child(volume_icon)
                )
                .child(
                    div()
                        .w(px(60.0))
                        .h(px(4.0))
                        .bg(hsla(0.0, 0.0, 0.3, 1.0))
                        .rounded_full()
                        .cursor_pointer()
                        .child(
                            div()
                                .h_full()
                                .w(pct(if muted { 0.0 } else { volume * 100.0 }))
                                .bg(hsla(0.0, 0.0, 0.8, 1.0))
                                .rounded_full()
                        )
                )
        )
        // Time
        .child(
            div()
                .text_size(px(12.0))
                .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                .child(format!(
                    "{} / {}",
                    format_time(current_time),
                    format_time(duration)
                ))
        )
}

/// Build right side controls (quality, fullscreen)
fn build_right_controls(quality: VideoQuality, fullscreen: bool) -> Div {
    div()
        .flex()
        .items_center()
        .gap(px(12.0))
        // Quality
        .child(
            div()
                .px(px(8.0))
                .py(px(2.0))
                .bg(hsla(0.0, 0.0, 0.2, 1.0))
                .rounded(px(4.0))
                .text_size(px(11.0))
                .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                .cursor_pointer()
                .child(quality.label())
        )
        // Fullscreen
        .child(
            div()
                .text_size(px(14.0))
                .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                .cursor_pointer()
                .child(if fullscreen { "⤓" } else { "⤢" })
        )
}
