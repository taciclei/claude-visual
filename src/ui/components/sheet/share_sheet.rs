//! ShareSheet component

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Share sheet
#[derive(Clone)]
pub struct ShareSheet {
    title: String,
    items: Vec<ShareItem>,
}

impl ShareSheet {
    pub fn new() -> Self {
        Self {
            title: "Share".to_string(),
            items: vec![
                ShareItem { label: "Copy Link".to_string(), icon: "🔗".to_string() },
                ShareItem { label: "Email".to_string(), icon: "📧".to_string() },
                ShareItem { label: "Message".to_string(), icon: "💬".to_string() },
                ShareItem { label: "More...".to_string(), icon: "⋯".to_string() },
            ],
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn items(mut self, items: Vec<ShareItem>) -> Self {
        self.items = items;
        self
    }
}

impl Default for ShareSheet {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ShareSheet {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.2, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let backdrop = hsla(0.0, 0.0, 0.0, 0.5);

        div()
            .absolute()
            .inset_0()
            .bg(backdrop)
            .flex()
            .flex_col()
            .justify_end()
            .child(
                div()
                    .w_full()
                    .bg(surface)
                    .rounded_t(px(16.0))
                    .border_t_1()
                    .border_color(border)
                    // Drag handle
                    .child(
                        div()
                            .w_full()
                            .py_3()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .w(px(36.0))
                                    .h(px(4.0))
                                    .rounded(px(2.0))
                                    .bg(hsla(0.0, 0.0, 0.35, 1.0))
                            )
                    )
                    // Title
                    .child(
                        div()
                            .w_full()
                            .px_4()
                            .pb_3()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(text)
                                    .child(self.title)
                            )
                    )
                    // Share items grid
                    .child(
                        div()
                            .w_full()
                            .px_4()
                            .pb_6()
                            .flex()
                            .flex_wrap()
                            .justify_center()
                            .gap_4()
                            .children(
                                self.items.into_iter().map(|item| {
                                    div()
                                        .w(px(70.0))
                                        .flex()
                                        .flex_col()
                                        .items_center()
                                        .gap_2()
                                        .cursor_pointer()
                                        // Icon circle
                                        .child(
                                            div()
                                                .size(px(50.0))
                                                .rounded_full()
                                                .bg(hsla(0.0, 0.0, 0.2, 1.0))
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .text_xl()
                                                .hover(|s| s.bg(hsla(0.0, 0.0, 0.25, 1.0)))
                                                .child(item.icon)
                                        )
                                        // Label
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(text_muted)
                                                .text_center()
                                                .child(item.label)
                                        )
                                })
                            )
                    )
            )
    }
}
