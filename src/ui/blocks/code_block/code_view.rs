//! Normal code view rendering

use gpui::*;
use gpui::prelude::*;

use crate::syntax::{SyntaxHighlighter, HighlightedSpan};

use super::types::HighlightStyle;
use super::view::CodeBlockView;

impl CodeBlockView {
    /// Render the normal code view
    pub(crate) fn render_code_view(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let lines: Vec<&str> = self.code.lines().collect();
        let line_count = lines.len();
        let show_line_numbers = self.show_line_numbers;
        let syntax_colors = theme.syntax.clone();
        let language = self.language.clone();

        // Pre-highlight all lines
        let highlighted_lines: Vec<_> = lines
            .iter()
            .map(|line| {
                SyntaxHighlighter::highlight(line, language.as_deref(), &syntax_colors)
            })
            .collect();

        // Determine which lines have matches and which is current
        let lines_with_matches: Vec<usize> = self
            .search_matches
            .iter()
            .map(|m| m.line)
            .collect();
        let current_match_line = self
            .current_match_index
            .and_then(|idx| self.search_matches.get(idx))
            .map(|m| m.line);

        // Pre-compute line highlights (1-based line numbers)
        let line_highlights: Vec<Option<HighlightStyle>> = (0..line_count)
            .map(|i| self.get_line_highlight(i + 1))
            .collect();
        let has_any_highlights = self.has_highlights();

        div()
            .w_full()
            .overflow_hidden()
            .bg(theme.colors.background)
            .px_3()
            .py_2()
            .child(
                div()
                    .flex()
                    .flex_row()
                    // Highlight indicator gutter (only when highlights exist)
                    .when(has_any_highlights, |d| {
                        d.child(self.render_highlight_gutter(line_count, &line_highlights, &theme))
                    })
                    // Line numbers
                    .when(show_line_numbers, |d| {
                        d.child(self.render_line_numbers(
                            line_count,
                            &lines_with_matches,
                            current_match_line,
                            &line_highlights,
                            &theme,
                        ))
                    })
                    // Code lines with syntax highlighting
                    .child(self.render_code_lines(
                        highlighted_lines,
                        &lines_with_matches,
                        current_match_line,
                        &line_highlights,
                        &theme,
                    )),
            )
    }

    /// Render highlight indicator gutter
    fn render_highlight_gutter(
        &self,
        line_count: usize,
        line_highlights: &[Option<HighlightStyle>],
        theme: &crate::app::theme::Theme,
    ) -> impl IntoElement {
        div()
            .flex_shrink_0()
            .w(px(3.0))
            .mr_1()
            .children((0..line_count).map(|i| {
                let highlight = line_highlights.get(i).copied().flatten();
                div()
                    .h(px(18.0)) // Match line height
                    .bg(if let Some(style) = highlight {
                        self.highlight_border_color(style, theme)
                    } else {
                        gpui::transparent_black()
                    })
            }))
    }

    /// Render line numbers column
    fn render_line_numbers(
        &self,
        line_count: usize,
        lines_with_matches: &[usize],
        current_match_line: Option<usize>,
        line_highlights: &[Option<HighlightStyle>],
        theme: &crate::app::theme::Theme,
    ) -> impl IntoElement {
        div()
            .flex_shrink_0()
            .pr_4()
            .text_right()
            .text_xs()
            .font_family("JetBrains Mono")
            .text_color(theme.colors.text_muted)
            .children((0..line_count).map(|i| {
                let has_match = lines_with_matches.contains(&i);
                let is_current = current_match_line == Some(i);
                let highlight = line_highlights.get(i).copied().flatten();
                div()
                    .bg(if is_current {
                        theme.colors.accent.opacity(0.3)
                    } else if has_match {
                        theme.colors.warning.opacity(0.2)
                    } else if let Some(style) = highlight {
                        self.highlight_bg_color(style, theme)
                    } else {
                        gpui::transparent_black()
                    })
                    .child(format!("{}", i + 1))
            }))
    }

    /// Render code lines with syntax highlighting
    fn render_code_lines(
        &self,
        highlighted_lines: Vec<Vec<HighlightedSpan>>,
        lines_with_matches: &[usize],
        current_match_line: Option<usize>,
        line_highlights: &[Option<HighlightStyle>],
        theme: &crate::app::theme::Theme,
    ) -> impl IntoElement {
        div()
            .flex_1()
            .text_xs()
            .font_family("JetBrains Mono")
            .children(
                highlighted_lines
                    .into_iter()
                    .enumerate()
                    .map(|(line_idx, spans)| {
                        let has_match = lines_with_matches.contains(&line_idx);
                        let is_current = current_match_line == Some(line_idx);
                        let highlight = line_highlights.get(line_idx).copied().flatten();
                        div()
                            .w_full()
                            .whitespace_nowrap()
                            .flex()
                            .flex_row()
                            .bg(if is_current {
                                theme.colors.accent.opacity(0.15)
                            } else if has_match {
                                theme.colors.warning.opacity(0.1)
                            } else if let Some(style) = highlight {
                                self.highlight_bg_color(style, theme)
                            } else {
                                gpui::transparent_black()
                            })
                            .children(spans.into_iter().map(|span| {
                                let mut el = div().child(span.text);
                                if let Some(color) = span.color {
                                    el = el.text_color(color);
                                }
                                el
                            }))
                    })
            )
    }
}
