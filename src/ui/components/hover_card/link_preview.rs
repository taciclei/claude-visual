//! Link preview hover card component

use gpui::prelude::*;
use gpui::*;

/// Link preview hover card
#[derive(IntoElement)]
pub struct LinkPreviewCard {
    url: SharedString,
    title: Option<SharedString>,
    description: Option<SharedString>,
    image: Option<SharedString>,
    favicon: Option<SharedString>,
    loading: bool,
}

impl LinkPreviewCard {
    pub fn new(url: impl Into<SharedString>) -> Self {
        Self {
            url: url.into(),
            title: None,
            description: None,
            image: None,
            favicon: None,
            loading: false,
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn image(mut self, image: impl Into<SharedString>) -> Self {
        self.image = Some(image.into());
        self
    }

    pub fn favicon(mut self, favicon: impl Into<SharedString>) -> Self {
        self.favicon = Some(favicon.into());
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
}

impl RenderOnce for LinkPreviewCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        if self.loading {
            return div()
                .w(px(300.0))
                .flex()
                .items_center()
                .justify_center()
                .py_4()
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.5,
                            a: 1.0,
                        })
                        .child("Loading preview..."),
                );
        }

        div()
            .flex()
            .flex_col()
            .w(px(300.0))
            // Image
            .when_some(self.image, |d, _img| {
                d.child(
                    div()
                        .h(px(150.0))
                        .bg(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.2,
                            a: 1.0,
                        })
                        .rounded_t_lg()
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(11.0))
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.5,
                            a: 1.0,
                        })
                        .child("Preview Image"),
                )
            })
            // Content
            .child(
                div()
                    .p_3()
                    .flex()
                    .flex_col()
                    .gap_2()
                    // URL with favicon
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .when_some(self.favicon, |d, _| {
                                d.child(div().w(px(14.0)).h(px(14.0)).rounded(px(2.0)).bg(Hsla {
                                    h: 0.0,
                                    s: 0.0,
                                    l: 0.3,
                                    a: 1.0,
                                }))
                            })
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(Hsla {
                                        h: 0.0,
                                        s: 0.0,
                                        l: 0.5,
                                        a: 1.0,
                                    })
                                    .truncate()
                                    .child(self.url),
                            ),
                    )
                    // Title
                    .when_some(self.title, |d, title| {
                        d.child(
                            div()
                                .text_size(px(14.0))
                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                .text_color(Hsla {
                                    h: 0.0,
                                    s: 0.0,
                                    l: 0.95,
                                    a: 1.0,
                                })
                                .line_height(px(18.0))
                                .child(title),
                        )
                    })
                    // Description
                    .when_some(self.description, |d, desc| {
                        d.child(
                            div()
                                .text_size(px(12.0))
                                .text_color(Hsla {
                                    h: 0.0,
                                    s: 0.0,
                                    l: 0.6,
                                    a: 1.0,
                                })
                                .line_height(px(16.0))
                                .max_h(px(48.0))
                                .overflow_hidden()
                                .child(desc),
                        )
                    }),
            )
    }
}
