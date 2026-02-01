//! Diff view rendering

use gpui::*;
use gpui::prelude::*;

use crate::syntax::SyntaxHighlighter;

use super::view::CodeBlockView;

impl CodeBlockView {
    /// Render the diff view
    pub(crate) fn render_diff_view(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let show_line_numbers = self.show_line_numbers;
        let syntax_colors = theme.syntax.clone();
        let language = self.language.clone();

        let diff_lines: Vec<_> = self.diff_lines.iter().map(|l| {
            (l.content.clone(), l.change_type, l.old_line_num, l.new_line_num)
        }).collect();

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
                    // Old line numbers
                    .when(show_line_numbers, |d| {
                        d.child(self.render_old_line_numbers(&diff_lines, &theme))
                    })
                    // New line numbers
                    .when(show_line_numbers, |d| {
                        d.child(self.render_new_line_numbers(&diff_lines, &theme))
                    })
                    // Prefix column (+/-)
                    .child(self.render_diff_prefixes(&diff_lines, &theme))
                    // Diff content
                    .child(self.render_diff_content(diff_lines, language, syntax_colors, &theme)),
            )
    }

    /// Render old line numbers column
    fn render_old_line_numbers(
        &self,
        diff_lines: &[(String, super::types::LineChangeType, Option<usize>, Option<usize>)],
        theme: &crate::app::theme::Theme,
    ) -> impl IntoElement {
        div()
            .flex_shrink_0()
            .w(px(40.0))
            .pr_1()
            .text_right()
            .text_xs()
            .font_family("JetBrains Mono")
            .text_color(theme.colors.text_muted)
            .children(diff_lines.iter().map(|(_, change_type, old_num, _)| {
                let bg = self.diff_line_bg(*change_type, theme);
                div()
                    .bg(bg)
                    .child(old_num.map(|n| n.to_string()).unwrap_or_default())
            }))
    }

    /// Render new line numbers column
    fn render_new_line_numbers(
        &self,
        diff_lines: &[(String, super::types::LineChangeType, Option<usize>, Option<usize>)],
        theme: &crate::app::theme::Theme,
    ) -> impl IntoElement {
        div()
            .flex_shrink_0()
            .w(px(40.0))
            .pr_2()
            .text_right()
            .text_xs()
            .font_family("JetBrains Mono")
            .text_color(theme.colors.text_muted)
            .children(diff_lines.iter().map(|(_, change_type, _, new_num)| {
                let bg = self.diff_line_bg(*change_type, theme);
                div()
                    .bg(bg)
                    .child(new_num.map(|n| n.to_string()).unwrap_or_default())
            }))
    }

    /// Render diff prefix column (+/-)
    fn render_diff_prefixes(
        &self,
        diff_lines: &[(String, super::types::LineChangeType, Option<usize>, Option<usize>)],
        theme: &crate::app::theme::Theme,
    ) -> impl IntoElement {
        div()
            .flex_shrink_0()
            .w(px(16.0))
            .text_xs()
            .font_family("JetBrains Mono")
            .font_weight(FontWeight::BOLD)
            .children(diff_lines.iter().map(|(_, change_type, _, _)| {
                let bg = self.diff_line_bg(*change_type, theme);
                let color = self.diff_prefix_color(*change_type, theme);
                div()
                    .bg(bg)
                    .text_color(color)
                    .child(change_type.prefix())
            }))
    }

    /// Render diff content with syntax highlighting
    fn render_diff_content(
        &self,
        diff_lines: Vec<(String, super::types::LineChangeType, Option<usize>, Option<usize>)>,
        language: Option<String>,
        syntax_colors: crate::app::theme::SyntaxColors,
        theme: &crate::app::theme::Theme,
    ) -> impl IntoElement {
        div()
            .flex_1()
            .text_xs()
            .font_family("JetBrains Mono")
            .children(diff_lines.into_iter().map(|(content, change_type, _, _)| {
                let bg = self.diff_line_bg(change_type, theme);
                // Highlight the line with syntax colors
                let spans = SyntaxHighlighter::highlight(&content, language.as_deref(), &syntax_colors);
                div()
                    .w_full()
                    .whitespace_nowrap()
                    .flex()
                    .flex_row()
                    .bg(bg)
                    .children(spans.into_iter().map(|span| {
                        let mut el = div().child(span.text);
                        if let Some(color) = span.color {
                            el = el.text_color(color);
                        }
                        el
                    }))
            }))
    }
}
