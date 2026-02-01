//! Image gallery carousel component

use gpui::*;
use gpui::prelude::*;

use super::types::GalleryImage;

/// Image gallery carousel
#[derive(Clone)]
pub struct ImageGallery {
    images: Vec<GalleryImage>,
    current_index: usize,
    show_thumbnails: bool,
}

impl ImageGallery {
    pub fn new() -> Self {
        Self {
            images: Vec::new(),
            current_index: 0,
            show_thumbnails: true,
        }
    }

    pub fn images(mut self, images: Vec<GalleryImage>) -> Self {
        self.images = images;
        self
    }

    pub fn current(mut self, index: usize) -> Self {
        self.current_index = index;
        self
    }

    pub fn hide_thumbnails(mut self) -> Self {
        self.show_thumbnails = false;
        self
    }
}

impl Default for ImageGallery {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ImageGallery {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.1, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);

        let total = self.images.len();
        let current = self.current_index.min(total.saturating_sub(1));
        let current_image = self.images.get(current).cloned();

        div()
            .w_full()
            .flex()
            .flex_col()
            .gap_2()
            // Main image
            .child(
                div()
                    .w_full()
                    .h(px(400.0))
                    .bg(surface)
                    .rounded(px(8.0))
                    .overflow_hidden()
                    .relative()
                    .flex()
                    .items_center()
                    .justify_center()
                    .when_some(current_image.clone(), |d, img| {
                        d.child(
                            div()
                                .text_3xl()
                                .text_color(text_muted)
                                .child(img.src) // Placeholder - would be actual image
                        )
                    })
                    // Navigation
                    .when(total > 1, |d| {
                        d.child(
                            div()
                                .absolute()
                                .left_2()
                                .top_1_2()

                                .size(px(40.0))
                                .rounded_full()
                                .bg(hsla(0.0, 0.0, 0.0, 0.5))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_color(text)
                                .cursor_pointer()
                                .hover(|s| s.bg(hsla(0.0, 0.0, 0.0, 0.7)))
                                .child("‹")
                        )
                        .child(
                            div()
                                .absolute()
                                .right_2()
                                .top_1_2()

                                .size(px(40.0))
                                .rounded_full()
                                .bg(hsla(0.0, 0.0, 0.0, 0.5))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_color(text)
                                .cursor_pointer()
                                .hover(|s| s.bg(hsla(0.0, 0.0, 0.0, 0.7)))
                                .child("›")
                        )
                    })
                    // Caption
                    .when_some(current_image.and_then(|i| i.caption), |d, caption| {
                        d.child(
                            div()
                                .absolute()
                                .bottom_0()
                                .left_0()
                                .right_0()
                                .px_4()
                                .py_3()
                                .bg(hsla(0.0, 0.0, 0.0, 0.7))
                                .text_sm()
                                .text_color(text)
                                .child(caption)
                        )
                    })
                    // Counter
                    .when(total > 1, |d| {
                        d.child(
                            div()
                                .absolute()
                                .top_2()
                                .right_2()
                                .px_2()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(hsla(0.0, 0.0, 0.0, 0.5))
                                .text_xs()
                                .text_color(text)
                                .child(format!("{} / {}", current + 1, total))
                        )
                    })
            )
            // Thumbnails
            .when(self.show_thumbnails && total > 1, |d| {
                d.child(
                    div()
                        .w_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .gap_2()
                        .overflow_hidden()
                        .children(
                            self.images.into_iter().enumerate().map(move |(idx, img)| {
                                let is_current = idx == current;
                                div()
                                    .size(px(60.0))
                                    .rounded(px(4.0))
                                    .bg(surface)
                                    .border_2()
                                    .border_color(if is_current { accent } else { border })
                                    .overflow_hidden()
                                    .cursor_pointer()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_xs()
                                    .text_color(text_muted)
                                    .hover(|s| s.border_color(accent.opacity(0.5)))
                                    .child(format!("{}", idx + 1))
                            })
                        )
                )
            })
    }
}
