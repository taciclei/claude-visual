use gpui::*;
use gpui::prelude::*;
use super::types::Ratio;

/// Video placeholder with play button
#[derive(IntoElement)]
pub struct VideoPlaceholder {
    ratio: Ratio,
    width: f32,
    thumbnail: Option<SharedString>,
    duration: Option<SharedString>,
    title: Option<SharedString>,
    background: Option<Hsla>,
    play_button_color: Option<Hsla>,
}

impl VideoPlaceholder {
    pub fn new() -> Self {
        Self {
            ratio: Ratio::Video,
            width: 400.0,
            thumbnail: None,
            duration: None,
            title: None,
            background: None,
            play_button_color: None,
        }
    }

    pub fn ratio(mut self, ratio: Ratio) -> Self {
        self.ratio = ratio;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn thumbnail(mut self, src: impl Into<SharedString>) -> Self {
        self.thumbnail = Some(src.into());
        self
    }

    pub fn duration(mut self, duration: impl Into<SharedString>) -> Self {
        self.duration = Some(duration.into());
        self
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn background(mut self, color: Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn play_button_color(mut self, color: Hsla) -> Self {
        self.play_button_color = Some(color);
        self
    }
}

impl Default for VideoPlaceholder {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for VideoPlaceholder {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let background = self.background.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.15,
            a: 1.0,
        });
        let play_color = self.play_button_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 1.0,
            a: 0.9,
        });

        let height = self.width / self.ratio.value();

        div()
            .relative()
            .w(px(self.width))
            .h(px(height))
            .bg(background)
            .rounded_lg()
            .overflow_hidden()
            .cursor_pointer()
            // Play button
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .w(px(64.0))
                            .h(px(64.0))
                            .bg(Hsla {
                                h: 0.0,
                                s: 0.0,
                                l: 0.0,
                                a: 0.5,
                            })
                            .rounded_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .text_color(play_color)
                                    .ml(px(4.0))
                                    .child("▶"),
                            ),
                    ),
            )
            // Duration badge
            .when_some(self.duration, |d, duration| {
                d.child(
                    div()
                        .absolute()
                        .bottom_2()
                        .right_2()
                        .bg(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.0,
                            a: 0.8,
                        })
                        .px_2()
                        .py(px(2.0))
                        .rounded(px(4.0))
                        .text_size(px(11.0))
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 1.0,
                            a: 1.0,
                        })
                        .child(duration),
                )
            })
            // Title
            .when_some(self.title, |d, title| {
                d.child(
                    div()
                        .absolute()
                        .bottom_0()
                        .left_0()
                        .right_0()
                        .bg(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.0,
                            a: 0.6,
                        })
                        .px_3()
                        .py_2()
                        .text_size(px(13.0))
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 1.0,
                            a: 1.0,
                        })
                        .child(title),
                )
            })
    }
}
