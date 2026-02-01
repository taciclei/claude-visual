//! MiniPlayer render implementation

use gpui::prelude::*;
use gpui::*;

use super::types::PlaybackState;
use super::MiniPlayer;

impl RenderOnce for MiniPlayer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let play_icon = match self.state {
            PlaybackState::Playing => "⏸",
            _ => "▶",
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(12.0))
            .w(px(320.0))
            .p(px(8.0))
            .bg(hsla(0.0, 0.0, 0.12, 1.0))
            .border_1()
            .border_color(hsla(0.0, 0.0, 0.2, 1.0))
            .rounded(px(8.0))
            .shadow_lg()
            // Thumbnail
            .child(
                div()
                    .relative()
                    .w(px(80.0))
                    .h(px(45.0))
                    .bg(hsla(0.0, 0.0, 0.2, 1.0))
                    .rounded(px(4.0))
                    .overflow_hidden()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_size(px(20.0))
                            .child(self.thumbnail.clone().unwrap_or("🎬".into())),
                    )
                    // Progress
                    .child(
                        div()
                            .absolute()
                            .bottom_0()
                            .left_0()
                            .right_0()
                            .h(px(2.0))
                            .bg(hsla(0.0, 0.0, 0.3, 1.0))
                            .child(
                                div()
                                    .h_full()
                                    .w(relative(self.progress))
                                    .bg(hsla(0.0, 0.7, 0.5, 1.0)),
                            ),
                    ),
            )
            // Title
            .child(
                div()
                    .flex_1()
                    .text_size(px(13.0))
                    .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                    .text_ellipsis()
                    .child(self.title.clone()),
            )
            // Controls
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(32.0))
                            .h(px(32.0))
                            .rounded_full()
                            .bg(hsla(0.0, 0.0, 0.2, 1.0))
                            .cursor_pointer()
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                                    .child(play_icon),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(hsla(0.0, 0.0, 0.6, 1.0))
                            .cursor_pointer()
                            .child("✕"),
                    ),
            )
    }
}
