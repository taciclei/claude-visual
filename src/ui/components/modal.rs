//! Modal dialog component

use gpui::prelude::*;
use gpui::prelude::*;
use gpui::prelude::*;
use gpui::*;

/// Modal dialog component
pub struct Modal {
    title: String,
    content: AnyElement,
    show_close: bool,
}

impl Modal {
    pub fn new(title: impl Into<String>, content: impl IntoElement) -> Self {
        Self {
            title: title.into(),
            content: content.into_any_element(),
            show_close: true,
        }
    }

    pub fn show_close(mut self, show: bool) -> Self {
        self.show_close = show;
        self
    }
}

impl RenderOnce for Modal {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bg = hsla(220.0 / 360.0, 0.13, 0.09, 1.0);
        let surface = hsla(220.0 / 360.0, 0.13, 0.12, 1.0);
        let border = hsla(220.0 / 360.0, 0.13, 0.20, 1.0);
        let text = hsla(0.0, 0.0, 0.93, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.60, 1.0);

        // Backdrop
        div()
            .absolute()
            .inset_0()
            .bg(bg.opacity(0.8))
            .flex()
            .items_center()
            .justify_center()
            // Modal container
            .child(
                div()
                    .min_w(px(400.0))
                    .max_w(px(600.0))
                    .rounded_lg()
                    .bg(surface)
                    .border_1()
                    .border_color(border)
                    .overflow_hidden()
                    // Header
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .px_4()
                            .py_3()
                            .border_b_1()
                            .border_color(border)
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(text)
                                    .child(self.title.clone()),
                            )
                            .when(self.show_close, |d| {
                                d.child(
                                    div()
                                        .size(px(24.0))
                                        .rounded_md()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_color(text_muted)
                                        .hover(|style| style.bg(border))
                                        .cursor_pointer()
                                        .child("x"),
                                )
                            }),
                    )
                    // Content
                    .child(div().px_4().py_4().child(self.content)),
            )
    }
}
