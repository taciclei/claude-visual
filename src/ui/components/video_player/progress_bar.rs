//! Progress bar component for video player

use gpui::*;
use gpui::prelude::*;
use crate::ui::pct;

/// Build the progress bar with buffered and playback progress
pub(crate) fn build_progress_bar(progress: f32, buffered: f32) -> Div {
    div()
        .relative()
        .h(px(4.0))
        .bg(hsla(0.0, 0.0, 0.3, 1.0))
        .rounded_full()
        .cursor_pointer()
        .mb(px(8.0))
        // Buffered
        .child(
            div()
                .absolute()
                .top_0()
                .left_0()
                .bottom_0()
                .w(pct(buffered * 100.0))
                .bg(hsla(0.0, 0.0, 0.4, 1.0))
                .rounded_full()
        )
        // Progress
        .child(
            div()
                .absolute()
                .top_0()
                .left_0()
                .bottom_0()
                .w(pct(progress * 100.0))
                .bg(hsla(0.0, 0.7, 0.5, 1.0))
                .rounded_full()
        )
}

/// Build a simple progress bar for thumbnails
pub(crate) fn build_thumbnail_progress(progress: f32) -> Div {
    div()
        .absolute()
        .bottom_0()
        .left_0()
        .right_0()
        .h(px(3.0))
        .bg(hsla(0.0, 0.0, 0.3, 1.0))
        .child(
            div()
                .h_full()
                .w(pct(progress * 100.0))
                .bg(hsla(0.0, 0.7, 0.5, 1.0))
        )
}
