//! Diff editor component for side-by-side comparison

use gpui::prelude::*;
use gpui::*;

/// Diff editor - side by side comparison
#[derive(IntoElement)]
pub struct DiffEditor {
    id: ElementId,
    left_lines: Vec<SharedString>,
    right_lines: Vec<SharedString>,
    left_title: Option<SharedString>,
    right_title: Option<SharedString>,
    additions: Vec<usize>,
    deletions: Vec<usize>,
    font_size: f32,
}

impl DiffEditor {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            left_lines: Vec::new(),
            right_lines: Vec::new(),
            left_title: None,
            right_title: None,
            additions: Vec::new(),
            deletions: Vec::new(),
            font_size: 13.0,
        }
    }

    pub fn left_content(mut self, content: &str) -> Self {
        self.left_lines = content.lines().map(|l| l.to_string().into()).collect();
        self
    }

    pub fn right_content(mut self, content: &str) -> Self {
        self.right_lines = content.lines().map(|l| l.to_string().into()).collect();
        self
    }

    pub fn left_title(mut self, title: impl Into<SharedString>) -> Self {
        self.left_title = Some(title.into());
        self
    }

    pub fn right_title(mut self, title: impl Into<SharedString>) -> Self {
        self.right_title = Some(title.into());
        self
    }

    pub fn additions(mut self, lines: Vec<usize>) -> Self {
        self.additions = lines;
        self
    }

    pub fn deletions(mut self, lines: Vec<usize>) -> Self {
        self.deletions = lines;
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }
}

impl RenderOnce for DiffEditor {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let line_height = self.font_size * 1.5;
        let max_lines = self.left_lines.len().max(self.right_lines.len());

        div()
            .id(self.id)
            .flex()
            .w_full()
            .bg(hsla(0.0, 0.0, 0.08, 1.0))
            .rounded(px(8.0))
            .overflow_hidden()
            // Left side
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .border_r_1()
                    .border_color(hsla(0.0, 0.0, 0.2, 1.0))
                    .when_some(self.left_title.clone(), |el, title| {
                        el.child(
                            div()
                                .px(px(12.0))
                                .py(px(8.0))
                                .border_b_1()
                                .border_color(hsla(0.0, 0.0, 0.2, 1.0))
                                .text_size(px(12.0))
                                .font_weight(gpui::FontWeight::MEDIUM)
                                .text_color(hsla(0.0, 0.0, 0.7, 1.0))
                                .child(title),
                        )
                    })
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .font_family("monospace")
                            .id("scroll-diff-left")
                            .overflow_y_scroll()
                            .children((0..max_lines).map(|i| {
                                let line = self.left_lines.get(i).cloned();
                                let is_deleted = self.deletions.contains(&(i + 1));

                                let bg = if is_deleted {
                                    hsla(0.0, 0.5, 0.3, 0.2)
                                } else {
                                    hsla(0.0, 0.0, 0.0, 0.0)
                                };

                                div()
                                    .flex()
                                    .h(px(line_height))
                                    .bg(bg)
                                    .when(is_deleted, |el| {
                                        el.border_l_2().border_color(hsla(0.0, 0.7, 0.5, 1.0))
                                    })
                                    .child(
                                        div()
                                            .w(px(40.0))
                                            .h_full()
                                            .flex()
                                            .items_center()
                                            .justify_end()
                                            .pr(px(8.0))
                                            .text_size(px(self.font_size))
                                            .text_color(hsla(0.0, 0.0, 0.4, 1.0))
                                            .child((i + 1).to_string()),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .flex_1()
                                            .pl(px(8.0))
                                            .text_size(px(self.font_size))
                                            .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                                            .when_some(line, |el, l| el.child(l)),
                                    )
                            })),
                    ),
            )
            // Right side
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .when_some(self.right_title.clone(), |el, title| {
                        el.child(
                            div()
                                .px(px(12.0))
                                .py(px(8.0))
                                .border_b_1()
                                .border_color(hsla(0.0, 0.0, 0.2, 1.0))
                                .text_size(px(12.0))
                                .font_weight(gpui::FontWeight::MEDIUM)
                                .text_color(hsla(0.0, 0.0, 0.7, 1.0))
                                .child(title),
                        )
                    })
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .font_family("monospace")
                            .id("scroll-diff-right")
                            .overflow_y_scroll()
                            .children((0..max_lines).map(|i| {
                                let line = self.right_lines.get(i).cloned();
                                let is_added = self.additions.contains(&(i + 1));

                                let bg = if is_added {
                                    hsla(0.35, 0.5, 0.3, 0.2)
                                } else {
                                    hsla(0.0, 0.0, 0.0, 0.0)
                                };

                                div()
                                    .flex()
                                    .h(px(line_height))
                                    .bg(bg)
                                    .when(is_added, |el| {
                                        el.border_l_2().border_color(hsla(0.35, 0.7, 0.5, 1.0))
                                    })
                                    .child(
                                        div()
                                            .w(px(40.0))
                                            .h_full()
                                            .flex()
                                            .items_center()
                                            .justify_end()
                                            .pr(px(8.0))
                                            .text_size(px(self.font_size))
                                            .text_color(hsla(0.0, 0.0, 0.4, 1.0))
                                            .child((i + 1).to_string()),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .flex_1()
                                            .pl(px(8.0))
                                            .text_size(px(self.font_size))
                                            .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                                            .when_some(line, |el, l| el.child(l)),
                                    )
                            })),
                    ),
            )
    }
}
