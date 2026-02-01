//! Image comparison slider component (before/after)

use gpui::*;
use gpui::prelude::*;

/// Image comparison slider (before/after)
#[derive(IntoElement)]
pub struct ImageComparison {
    before_src: SharedString,
    after_src: SharedString,
    before_label: SharedString,
    after_label: SharedString,
    width: f32,
    height: f32,
    slider_position: f32, // 0.0 to 1.0
    label_color: Option<Hsla>,
}

impl ImageComparison {
    pub fn new(
        before: impl Into<SharedString>,
        after: impl Into<SharedString>,
    ) -> Self {
        Self {
            before_src: before.into(),
            after_src: after.into(),
            before_label: "Before".into(),
            after_label: "After".into(),
            width: 400.0,
            height: 300.0,
            slider_position: 0.5,
            label_color: None,
        }
    }

    pub fn before_label(mut self, label: impl Into<SharedString>) -> Self {
        self.before_label = label.into();
        self
    }

    pub fn after_label(mut self, label: impl Into<SharedString>) -> Self {
        self.after_label = label.into();
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn slider_position(mut self, position: f32) -> Self {
        self.slider_position = position.clamp(0.0, 1.0);
        self
    }

    pub fn label_color(mut self, color: Hsla) -> Self {
        self.label_color = Some(color);
        self
    }
}

impl RenderOnce for ImageComparison {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let label_color = self.label_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 1.0,
            a: 1.0,
        });
        let divider_x = self.width * self.slider_position;

        div()
            .relative()
            .w(px(self.width))
            .h(px(self.height))
            .overflow_hidden()
            .rounded_lg()
            // Before image (full)
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .bg(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 0.2,
                        a: 1.0,
                    })
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 0.5,
                        a: 1.0,
                    })
                    .text_size(px(11.0))
                    .child(self.after_src),
            )
            // After image (clipped)
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .bottom_0()
                    .w(px(divider_x))
                    .overflow_hidden()
                    .child(
                        div()
                            .w(px(self.width))
                            .h(px(self.height))
                            .bg(Hsla {
                                h: 0.0,
                                s: 0.0,
                                l: 0.25,
                                a: 1.0,
                            })
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(Hsla {
                                h: 0.0,
                                s: 0.0,
                                l: 0.5,
                                a: 1.0,
                            })
                            .text_size(px(11.0))
                            .child(self.before_src),
                    ),
            )
            // Slider handle
            .child(
                div()
                    .absolute()
                    .top_0()
                    .bottom_0()
                    .left(px(divider_x - 2.0))
                    .w(px(4.0))
                    .bg(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 1.0,
                        a: 1.0,
                    })
                    .cursor_ew_resize(),
            )
            // Labels
            .child(
                div()
                    .absolute()
                    .top_2()
                    .left_2()
                    .bg(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 0.0,
                        a: 0.6,
                    })
                    .px_2()
                    .py(px(2.0))
                    .rounded(px(4.0))
                    .text_size(px(11.0))
                    .text_color(label_color)
                    .child(self.before_label),
            )
            .child(
                div()
                    .absolute()
                    .top_2()
                    .right_2()
                    .bg(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 0.0,
                        a: 0.6,
                    })
                    .px_2()
                    .py(px(2.0))
                    .rounded(px(4.0))
                    .text_size(px(11.0))
                    .text_color(label_color)
                    .child(self.after_label),
            )
    }
}
