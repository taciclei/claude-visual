//! Main code editor component

use gpui::*;
use gpui::prelude::*;

use super::types::{EditorTheme, EditorFontSize, EditorLine, Selection};

/// Code editor component
#[derive(IntoElement)]
pub struct CodeEditor {
    id: ElementId,
    lines: Vec<EditorLine>,
    selection: Option<Selection>,
    theme: EditorTheme,
    font_size: EditorFontSize,
    show_line_numbers: bool,
    show_gutter: bool,
    show_minimap: bool,
    word_wrap: bool,
    read_only: bool,
    highlight_current_line: bool,
}

impl CodeEditor {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            lines: Vec::new(),
            selection: None,
            theme: EditorTheme::default(),
            font_size: EditorFontSize::default(),
            show_line_numbers: true,
            show_gutter: true,
            show_minimap: false,
            word_wrap: false,
            read_only: false,
            highlight_current_line: true,
        }
    }

    pub fn lines(mut self, lines: Vec<EditorLine>) -> Self {
        self.lines = lines;
        self
    }

    pub fn content(mut self, content: &str) -> Self {
        self.lines = content
            .lines()
            .enumerate()
            .map(|(i, line)| EditorLine::new(i + 1, &line.to_string()))
            .collect();
        self
    }

    pub fn selection(mut self, selection: Selection) -> Self {
        self.selection = Some(selection);
        self
    }

    pub fn theme(mut self, theme: EditorTheme) -> Self {
        self.theme = theme;
        self
    }

    pub fn font_size(mut self, font_size: EditorFontSize) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn show_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    pub fn show_gutter(mut self, show: bool) -> Self {
        self.show_gutter = show;
        self
    }

    pub fn show_minimap(mut self, show: bool) -> Self {
        self.show_minimap = show;
        self
    }

    pub fn word_wrap(mut self, wrap: bool) -> Self {
        self.word_wrap = wrap;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    pub fn highlight_current_line(mut self, highlight: bool) -> Self {
        self.highlight_current_line = highlight;
        self
    }
}

impl RenderOnce for CodeEditor {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bg = self.theme.background();
        let text_color = self.theme.text_color();
        let line_number_color = self.theme.line_number_color();
        let selection_color = self.theme.selection_color();
        let font_size = self.font_size.size();
        let line_height = self.font_size.line_height();

        let max_line_num = self.lines.len().max(1);
        let line_num_width = (max_line_num.to_string().len() as f32 * font_size * 0.6) + 24.0;

        let cursor_line = self.selection.as_ref().map(|s| s.start_line);

        div()
            .id(self.id)
            .flex()
            .w_full()
            .h_full()
            .bg(bg)
            .overflow_hidden()
            // Main editor area
            .child(
                div()
                    .flex()
                    .flex_1()
                    .id("scroll-editor")
                    .overflow_y_scroll()
                    .font_family("monospace")
                    // Lines
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .children(self.lines.iter().map(|line| {
                                let is_current = self.highlight_current_line
                                    && cursor_line == Some(line.number);

                                let line_bg = if is_current {
                                    hsla(0.0, 0.0, 0.12, 1.0)
                                } else {
                                    hsla(0.0, 0.0, 0.0, 0.0)
                                };

                                div()
                                    .flex()
                                    .items_start()
                                    .h(px(line_height))
                                    .bg(line_bg)
                                    // Gutter
                                    .when(self.show_gutter, |el| {
                                        el.child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .w(px(16.0))
                                                .h_full()
                                                .when(line.is_breakpoint, |el| {
                                                    el.child(
                                                        div()
                                                            .w(px(10.0))
                                                            .h(px(10.0))
                                                            .rounded_full()
                                                            .bg(hsla(0.0, 0.7, 0.5, 1.0))
                                                    )
                                                })
                                                .when(line.has_error, |el| {
                                                    el.child(
                                                        div()
                                                            .text_size(px(10.0))
                                                            .text_color(hsla(0.0, 0.8, 0.5, 1.0))
                                                            .child("●")
                                                    )
                                                })
                                                .when(line.has_warning && !line.has_error, |el| {
                                                    el.child(
                                                        div()
                                                            .text_size(px(10.0))
                                                            .text_color(hsla(0.12, 0.9, 0.5, 1.0))
                                                            .child("●")
                                                    )
                                                })
                                        )
                                    })
                                    // Line numbers
                                    .when(self.show_line_numbers, |el| {
                                        el.child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .justify_end()
                                                .w(px(line_num_width - 16.0))
                                                .h_full()
                                                .pr(px(12.0))
                                                .text_size(px(font_size))
                                                .text_color(if is_current {
                                                    text_color
                                                } else {
                                                    line_number_color
                                                })
                                                .child(line.number.to_string())
                                        )
                                    })
                                    // Content
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .flex_1()
                                            .h_full()
                                            .pl(px(8.0))
                                            .pr(px(16.0))
                                            .text_size(px(font_size))
                                            .text_color(text_color)
                                            .when(self.word_wrap, |el| el.flex_wrap())
                                            .when(line.is_modified, |el| {
                                                el.border_l_2()
                                                    .border_color(hsla(0.15, 0.8, 0.5, 1.0))
                                            })
                                            .child(
                                                div()
                                                    .whitespace_nowrap()
                                                    .child(line.content.clone())
                                            )
                                    )
                            }))
                    )
            )
            // Minimap
            .when(self.show_minimap, |el| {
                el.child(
                    div()
                        .w(px(80.0))
                        .h_full()
                        .bg(hsla(0.0, 0.0, 0.06, 1.0))
                        .border_l_1()
                        .border_color(hsla(0.0, 0.0, 0.15, 1.0))
                        .p(px(4.0))
                        .overflow_hidden()
                        .children(self.lines.iter().take(100).map(|line| {
                            let width = (line.content.len().min(50) as f32 / 50.0) * 72.0;
                            div()
                                .h(px(2.0))
                                .w(px(width))
                                .mb(px(1.0))
                                .bg(hsla(0.0, 0.0, 0.3, 1.0))
                                .rounded(px(1.0))
                        }))
                )
            })
    }
}
