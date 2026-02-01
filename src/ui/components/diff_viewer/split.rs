//! Split diff viewer (side-by-side)

use gpui::prelude::*;
use gpui::*;

/// Split diff viewer (side-by-side)
#[derive(IntoElement)]
pub struct SplitDiffViewer {
    id: ElementId,
    old_lines: Vec<(Option<usize>, SharedString)>,
    new_lines: Vec<(Option<usize>, SharedString)>,
    old_label: SharedString,
    new_label: SharedString,
    show_line_numbers: bool,
    background: gpui::Hsla,
}

impl SplitDiffViewer {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            old_lines: Vec::new(),
            new_lines: Vec::new(),
            old_label: "Original".into(),
            new_label: "Modified".into(),
            show_line_numbers: true,
            background: rgba(0x1a1a1aff).into(),
        }
    }

    pub fn old_lines(mut self, lines: Vec<(Option<usize>, impl Into<SharedString>)>) -> Self {
        self.old_lines = lines.into_iter().map(|(n, s)| (n, s.into())).collect();
        self
    }

    pub fn new_lines(mut self, lines: Vec<(Option<usize>, impl Into<SharedString>)>) -> Self {
        self.new_lines = lines.into_iter().map(|(n, s)| (n, s.into())).collect();
        self
    }

    pub fn labels(mut self, old: impl Into<SharedString>, new: impl Into<SharedString>) -> Self {
        self.old_label = old.into();
        self.new_label = new.into();
        self
    }

    pub fn show_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = color;
        self
    }
}

impl RenderOnce for SplitDiffViewer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let max_lines = self.old_lines.len().max(self.new_lines.len());

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .rounded_md()
            .overflow_hidden()
            .bg(self.background)
            .border_1()
            .border_color(rgba(0x8888881a))
            .child(
                // Headers
                div()
                    .flex()
                    .border_b_1()
                    .border_color(rgba(0x8888881a))
                    .child(
                        div()
                            .flex_1()
                            .px_3()
                            .py_2()
                            .bg(rgba(0xef44440d))
                            .text_sm()
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .text_color(rgb(0xef4444))
                            .child(self.old_label.clone()),
                    )
                    .child(
                        div()
                            .flex_1()
                            .px_3()
                            .py_2()
                            .bg(rgba(0x22c55e0d))
                            .text_sm()
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .text_color(rgb(0x22c55e))
                            .child(self.new_label.clone()),
                    ),
            )
            .child(
                // Split content
                div()
                    .flex()
                    .id("scroll-split-diff")
                    .overflow_y_scroll()
                    .max_h(px(400.0))
                    .child(
                        // Old side
                        div()
                            .flex_1()
                            .border_r_1()
                            .border_color(rgba(0x8888881a))
                            .children((0..max_lines).map(|i| {
                                let (line_num, content) = self
                                    .old_lines
                                    .get(i)
                                    .cloned()
                                    .unwrap_or((None, SharedString::default()));
                                div()
                                    .flex()
                                    .bg(rgba(0xef44440d))
                                    .when(self.show_line_numbers, |d| {
                                        d.child(
                                            div()
                                                .w(px(40.0))
                                                .text_right()
                                                .pr_2()
                                                .text_xs()
                                                .text_color(rgba(0x666666ff))
                                                .child(
                                                    line_num
                                                        .map(|n| format!("{}", n))
                                                        .unwrap_or_default(),
                                                ),
                                        )
                                    })
                                    .child(
                                        div()
                                            .flex_1()
                                            .pl_2()
                                            .text_sm()
                                            .font_family("monospace")
                                            .text_color(rgba(0xccccccff))
                                            .child(content),
                                    )
                            })),
                    )
                    .child(
                        // New side
                        div().flex_1().children((0..max_lines).map(|i| {
                            let (line_num, content) = self
                                .new_lines
                                .get(i)
                                .cloned()
                                .unwrap_or((None, SharedString::default()));
                            div()
                                .flex()
                                .bg(rgba(0x22c55e0d))
                                .when(self.show_line_numbers, |d| {
                                    d.child(
                                        div()
                                            .w(px(40.0))
                                            .text_right()
                                            .pr_2()
                                            .text_xs()
                                            .text_color(rgba(0x666666ff))
                                            .child(
                                                line_num
                                                    .map(|n| format!("{}", n))
                                                    .unwrap_or_default(),
                                            ),
                                    )
                                })
                                .child(
                                    div()
                                        .flex_1()
                                        .pl_2()
                                        .text_sm()
                                        .font_family("monospace")
                                        .text_color(rgba(0xccccccff))
                                        .child(content),
                                )
                        })),
                    ),
            )
    }
}
