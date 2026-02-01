//! Diff-style text change component

use gpui::*;
use gpui::prelude::*;

/// Diff-style text change
#[derive(Clone)]
pub struct TextDiff {
    old_text: String,
    new_text: String,
    inline: bool,
}

impl TextDiff {
    pub fn new(old: impl Into<String>, new: impl Into<String>) -> Self {
        Self {
            old_text: old.into(),
            new_text: new.into(),
            inline: true,
        }
    }

    pub fn stacked(mut self) -> Self {
        self.inline = false;
        self
    }
}

impl RenderOnce for TextDiff {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let removed_bg = hsla(0.0, 0.7, 0.5, 0.2);
        let added_bg = hsla(0.38, 0.7, 0.45, 0.2);
        let removed_text = hsla(0.0, 0.6, 0.6, 1.0);
        let added_text = hsla(0.38, 0.6, 0.5, 1.0);

        if self.inline {
            div()
                .flex()
                .items_center()
                .gap_1()
                .child(
                    div()
                        .px_1()
                        .rounded(px(2.0))
                        .bg(removed_bg)
                        .text_color(removed_text)
                        .line_through()
                        .child(self.old_text)
                )
                .child(
                    div()
                        .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                        .child("→")
                )
                .child(
                    div()
                        .px_1()
                        .rounded(px(2.0))
                        .bg(added_bg)
                        .text_color(added_text)
                        .child(self.new_text)
                )
        } else {
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .w(px(16.0))
                                .text_center()
                                .text_color(removed_text)
                                .child("-")
                        )
                        .child(
                            div()
                                .flex_1()
                                .px_2()
                                .py_1()
                                .rounded(px(2.0))
                                .bg(removed_bg)
                                .text_color(removed_text)
                                .line_through()
                                .child(self.old_text)
                        )
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .w(px(16.0))
                                .text_center()
                                .text_color(added_text)
                                .child("+")
                        )
                        .child(
                            div()
                                .flex_1()
                                .px_2()
                                .py_1()
                                .rounded(px(2.0))
                                .bg(added_bg)
                                .text_color(added_text)
                                .child(self.new_text)
                        )
                )
        }
    }
}
