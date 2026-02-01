//! Simple info hover card component

use gpui::prelude::*;
use gpui::*;

/// Simple info hover card
#[derive(IntoElement)]
pub struct InfoCard {
    title: SharedString,
    content: SharedString,
    icon: Option<SharedString>,
    footer: Option<SharedString>,
}

impl InfoCard {
    pub fn new(title: impl Into<SharedString>, content: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            icon: None,
            footer: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn footer(mut self, footer: impl Into<SharedString>) -> Self {
        self.footer = Some(footer.into());
        self
    }
}

impl RenderOnce for InfoCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .max_w(px(280.0))
            // Header
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .when_some(self.icon, |d, icon| {
                        d.child(div().text_size(px(16.0)).child(icon))
                    })
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(Hsla {
                                h: 0.0,
                                s: 0.0,
                                l: 0.95,
                                a: 1.0,
                            })
                            .child(self.title),
                    ),
            )
            // Content
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 0.7,
                        a: 1.0,
                    })
                    .line_height(px(18.0))
                    .child(self.content),
            )
            // Footer
            .when_some(self.footer, |d, footer| {
                d.child(
                    div()
                        .pt_2()
                        .border_t_1()
                        .border_color(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.25,
                            a: 1.0,
                        })
                        .text_size(px(11.0))
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.5,
                            a: 1.0,
                        })
                        .child(footer),
                )
            })
    }
}
