//! Play overlay component for video player

use super::types::PlaybackState;
use gpui::prelude::*;
use gpui::*;

/// Build the play overlay that appears when video is paused
pub(crate) fn build_play_overlay(
    state: PlaybackState,
    show_controls: bool,
    play_icon: String,
) -> Option<Div> {
    if state != PlaybackState::Playing && show_controls {
        Some(
            div()
                .absolute()
                .inset_0()
                .flex()
                .items_center()
                .justify_center()
                .bg(hsla(0.0, 0.0, 0.0, 0.4))
                .cursor_pointer()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(px(64.0))
                        .h(px(64.0))
                        .bg(hsla(0.0, 0.0, 0.0, 0.6))
                        .rounded_full()
                        .child(
                            div()
                                .text_size(px(28.0))
                                .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                                .child(play_icon),
                        ),
                ),
        )
    } else {
        None
    }
}
