//! VideoThumbnail render implementation

use gpui::prelude::*;
use gpui::*;

use super::controls::format_time;
use super::{progress_bar, VideoThumbnail};

impl RenderOnce for VideoThumbnail {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let height = self.width * 9.0 / 16.0;

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap(px(8.0))
            .w(px(self.width))
            .cursor_pointer()
            // Thumbnail
            .child(
                div()
                    .relative()
                    .w_full()
                    .h(px(height))
                    .bg(hsla(0.0, 0.0, 0.15, 1.0))
                    .rounded(px(8.0))
                    .overflow_hidden()
                    .child(
                        div()
                            .absolute()
                            .inset_0()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(40.0))
                            .text_color(hsla(0.0, 0.0, 0.3, 1.0))
                            .child(self.thumbnail.clone().unwrap_or("🎬".into())),
                    )
                    // Play icon overlay
                    .when(self.show_play_icon, |el| {
                        el.child(
                            div()
                                .absolute()
                                .inset_0()
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .w(px(40.0))
                                        .h(px(40.0))
                                        .bg(hsla(0.0, 0.0, 0.0, 0.7))
                                        .rounded_full()
                                        .child(
                                            div()
                                                .text_size(px(16.0))
                                                .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                                                .child("▶"),
                                        ),
                                ),
                        )
                    })
                    // Duration badge
                    .child(
                        div()
                            .absolute()
                            .bottom(px(8.0))
                            .right(px(8.0))
                            .px(px(6.0))
                            .py(px(2.0))
                            .bg(hsla(0.0, 0.0, 0.0, 0.8))
                            .rounded(px(4.0))
                            .text_size(px(11.0))
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                            .child(format_time(self.duration)),
                    )
                    // Progress bar
                    .when_some(self.progress, |el, progress| {
                        el.child(progress_bar::build_thumbnail_progress(progress))
                    }),
            )
            // Title
            .when_some(self.title, |el, title| {
                el.child(
                    div()
                        .text_size(px(14.0))
                        .font_weight(gpui::FontWeight::MEDIUM)
                        .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                        .text_ellipsis()
                        .child(title),
                )
            })
    }
}
