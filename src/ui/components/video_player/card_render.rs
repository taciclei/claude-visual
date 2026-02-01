//! VideoCard render implementation

use gpui::prelude::*;
use gpui::*;

use super::controls::{format_time, format_views};
use super::VideoCard;

impl RenderOnce for VideoCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let thumbnail_height = self.width * 9.0 / 16.0;

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap(px(12.0))
            .w(px(self.width))
            .cursor_pointer()
            // Thumbnail
            .child(
                div()
                    .relative()
                    .w_full()
                    .h(px(thumbnail_height))
                    .bg(hsla(0.0, 0.0, 0.15, 1.0))
                    .rounded(px(12.0))
                    .overflow_hidden()
                    .child(
                        div()
                            .absolute()
                            .inset_0()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(48.0))
                            .text_color(hsla(0.0, 0.0, 0.3, 1.0))
                            .child(self.thumbnail.clone().unwrap_or("🎬".into())),
                    )
                    .child(
                        div()
                            .absolute()
                            .bottom(px(8.0))
                            .right(px(8.0))
                            .px(px(6.0))
                            .py(px(2.0))
                            .bg(hsla(0.0, 0.0, 0.0, 0.8))
                            .rounded(px(4.0))
                            .text_size(px(12.0))
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                            .child(format_time(self.duration)),
                    ),
            )
            // Info
            .child(
                div()
                    .flex()
                    .gap(px(12.0))
                    // Channel avatar
                    .child(
                        div()
                            .w(px(36.0))
                            .h(px(36.0))
                            .rounded_full()
                            .bg(hsla(0.0, 0.0, 0.2, 1.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(14.0))
                            .text_color(hsla(0.0, 0.0, 0.7, 1.0))
                            .child(self.channel.chars().next().unwrap_or('?').to_string()),
                    )
                    // Metadata
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .flex_1()
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(gpui::FontWeight::MEDIUM)
                                    .text_color(hsla(0.0, 0.0, 0.95, 1.0))
                                    .line_clamp(2)
                                    .child(self.title.clone()),
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(hsla(0.0, 0.0, 0.6, 1.0))
                                    .child(self.channel.clone()),
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                    .child(format!(
                                        "{} • {}",
                                        format_views(self.views),
                                        self.uploaded_at
                                    )),
                            ),
                    ),
            )
    }
}
