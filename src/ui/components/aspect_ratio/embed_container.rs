use gpui::*;
use gpui::prelude::*;
use super::types::Ratio;

/// Embed container for iframes (video, maps, etc)
#[derive(IntoElement)]
pub struct EmbedContainer {
    ratio: Ratio,
    width: Option<f32>,
    src: SharedString,
    title: SharedString,
    loading: bool,
    background: Option<Hsla>,
}

impl EmbedContainer {
    pub fn new(src: impl Into<SharedString>, title: impl Into<SharedString>) -> Self {
        Self {
            ratio: Ratio::Video,
            width: None,
            src: src.into(),
            title: title.into(),
            loading: false,
            background: None,
        }
    }

    pub fn ratio(mut self, ratio: Ratio) -> Self {
        self.ratio = ratio;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn background(mut self, color: Hsla) -> Self {
        self.background = Some(color);
        self
    }
}

impl RenderOnce for EmbedContainer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let background = self.background.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.1,
            a: 1.0,
        });

        let width = self.width.unwrap_or(560.0);
        let height = width / self.ratio.value();

        let mut container = div()
            .relative()
            .w(px(width))
            .h(px(height))
            .bg(background)
            .rounded_lg()
            .overflow_hidden();

        if self.loading {
            container = container.child(
                div()
                    .absolute()
                    .inset_0()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(Hsla {
                                h: 0.0,
                                s: 0.0,
                                l: 0.5,
                                a: 1.0,
                            })
                            .child("Loading..."),
                    ),
            );
        } else {
            // In a real implementation, this would render an iframe
            // For now, show placeholder with the embed info
            container = container.child(
                div()
                    .absolute()
                    .inset_0()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(Hsla {
                                h: 0.0,
                                s: 0.0,
                                l: 0.6,
                                a: 1.0,
                            })
                            .child(self.title),
                    )
                    .child(
                        div()
                            .text_size(px(10.0))
                            .text_color(Hsla {
                                h: 0.0,
                                s: 0.0,
                                l: 0.4,
                                a: 1.0,
                            })
                            .max_w(px(width - 40.0))
                            .truncate()
                            .child(self.src),
                    ),
            );
        }

        container
    }
}
