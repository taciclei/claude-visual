//! Unified diff viewer component

use super::types::*;
use crate::ui::pct;
use gpui::prelude::*;
use gpui::*;

/// Unified diff viewer component
#[derive(IntoElement)]
pub struct DiffViewer {
    id: ElementId,
    lines: Vec<DiffLine>,
    view_mode: DiffViewMode,
    show_line_numbers: bool,
    syntax_highlight: bool,
    word_diff: bool,
    context_lines: usize,
    file_path: Option<SharedString>,
    old_label: SharedString,
    new_label: SharedString,
    background: gpui::Hsla,
}

impl DiffViewer {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            lines: Vec::new(),
            view_mode: DiffViewMode::default(),
            show_line_numbers: true,
            syntax_highlight: false,
            word_diff: false,
            context_lines: 3,
            file_path: None,
            old_label: "Old".into(),
            new_label: "New".into(),
            background: rgba(0x1a1a1aff).into(),
        }
    }

    pub fn lines(mut self, lines: Vec<DiffLine>) -> Self {
        self.lines = lines;
        self
    }

    pub fn view_mode(mut self, mode: DiffViewMode) -> Self {
        self.view_mode = mode;
        self
    }

    pub fn show_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    pub fn syntax_highlight(mut self, enable: bool) -> Self {
        self.syntax_highlight = enable;
        self
    }

    pub fn word_diff(mut self, enable: bool) -> Self {
        self.word_diff = enable;
        self
    }

    pub fn context_lines(mut self, lines: usize) -> Self {
        self.context_lines = lines;
        self
    }

    pub fn file_path(mut self, path: impl Into<SharedString>) -> Self {
        self.file_path = Some(path.into());
        self
    }

    pub fn labels(mut self, old: impl Into<SharedString>, new: impl Into<SharedString>) -> Self {
        self.old_label = old.into();
        self.new_label = new.into();
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = color;
        self
    }

    /// Count additions in the diff
    pub fn additions(&self) -> usize {
        self.lines
            .iter()
            .filter(|l| l.line_type == DiffLineType::Added)
            .count()
    }

    /// Count deletions in the diff
    pub fn deletions(&self) -> usize {
        self.lines
            .iter()
            .filter(|l| l.line_type == DiffLineType::Removed)
            .count()
    }

    fn render_line_number(&self, num: Option<usize>) -> impl IntoElement {
        div()
            .w(px(40.0))
            .flex_shrink_0()
            .text_right()
            .pr_2()
            .text_xs()
            .text_color(rgba(0x666666ff))
            .child(num.map(|n| format!("{}", n)).unwrap_or_default())
    }

    fn render_unified_line(&self, line: &DiffLine) -> Div {
        let show_line_numbers = self.show_line_numbers;
        let old_line_number = line.old_line_number;
        let new_line_number = line.new_line_number;
        let content = line.content.clone();

        let old_line_el = self.render_line_number(old_line_number);
        let new_line_el = self.render_line_number(new_line_number);

        // Clone values for use in div
        let background = line.line_type.background();
        let text_color = line.line_type.text_color();
        let prefix = line.line_type.prefix().to_string();

        div()
            .flex()
            .w_full()
            .bg(background)
            .when(show_line_numbers, |d| {
                d.child(old_line_el).child(new_line_el)
            })
            .child(
                div()
                    .w(px(16.0))
                    .flex_shrink_0()
                    .text_center()
                    .text_xs()
                    .text_color(text_color)
                    .child(prefix),
            )
            .child(
                div()
                    .flex_1()
                    .pl_2()
                    .text_sm()
                    .font_family("monospace")
                    .text_color(text_color)
                    .child(content),
            )
    }
}

impl RenderOnce for DiffViewer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let additions = self.additions();
        let deletions = self.deletions();
        let id = self.id.clone();

        div()
            .id(id)
            .flex()
            .flex_col()
            .rounded_md()
            .overflow_hidden()
            .bg(self.background)
            .border_1()
            .border_color(rgba(0x8888881a))
            .child(
                // Header
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py_2()
                    .bg(rgba(0x8888880d))
                    .border_b_1()
                    .border_color(rgba(0x8888881a))
                    .child(div().flex().items_center().gap_2().when_some(
                        self.file_path.clone(),
                        |d, path| {
                            d.child(
                                div()
                                    .text_sm()
                                    .font_weight(gpui::FontWeight::MEDIUM)
                                    .text_color(rgba(0xccccccff))
                                    .child(path),
                            )
                        },
                    ))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .text_xs()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .text_color(rgb(0x22c55e))
                                    .child(format!("+{}", additions)),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .text_color(rgb(0xef4444))
                                    .child(format!("-{}", deletions)),
                            ),
                    ),
            )
            .child(
                // Lines
                div()
                    .flex()
                    .flex_col()
                    .id("scroll-diff-lines")
                    .overflow_y_scroll()
                    .max_h(px(400.0))
                    .children(self.lines.iter().map(|line| self.render_unified_line(line))),
            )
    }
}
