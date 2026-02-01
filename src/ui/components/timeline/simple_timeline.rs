//! Simple timeline display component

use gpui::prelude::*;
use gpui::*;

/// Simple timeline display
#[derive(Clone)]
pub struct SimpleTimeline {
    pub(crate) items: Vec<(String, String, bool)>, // (title, time, completed)
}

impl SimpleTimeline {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn item(
        mut self,
        title: impl Into<String>,
        time: impl Into<String>,
        completed: bool,
    ) -> Self {
        self.items.push((title.into(), time.into(), completed));
        self
    }
}

impl Default for SimpleTimeline {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for SimpleTimeline {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let success = hsla(0.38, 0.7, 0.45, 1.0);

        div()
            .w_full()
            .flex()
            .flex_col()
            .children(
                self.items
                    .into_iter()
                    .enumerate()
                    .map(|(idx, (title, time, completed))| {
                        let is_last = idx == 0; // Check would need length

                        let dot_color = if completed { success } else { text_muted };

                        div()
                            .w_full()
                            .flex()
                            // Timeline column
                            .child(
                                div()
                                    .w(px(32.0))
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    // Dot
                                    .child(div().size(px(12.0)).rounded_full().bg(dot_color))
                                    // Line
                                    .child(div().w(px(2.0)).flex_1().min_h(px(20.0)).bg(border)),
                            )
                            // Content column
                            .child(
                                div()
                                    .flex_1()
                                    .pb_4()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(text)
                                            .child(title),
                                    )
                                    .child(div().text_xs().text_color(text_muted).child(time)),
                            )
                    }),
            )
    }
}
