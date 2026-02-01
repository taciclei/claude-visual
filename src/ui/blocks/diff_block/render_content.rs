//! Diff block content rendering

use gpui::*;
use gpui::prelude::*;

use super::types::DiffLine;
use super::render_line::render_line;

/// Render the diff content with all hunks and lines
pub fn render_content(
    hunk_data: Vec<(usize, String, bool, Vec<(char, String, Option<usize>, Option<usize>, &'static str)>)>,
    border_color: Hsla,
    surface_color: Hsla,
    text_color: Hsla,
    text_muted_color: Hsla,
    accent_color: Hsla,
    success_color: Hsla,
    error_color: Hsla,
    background_color: Hsla,
) -> impl IntoElement {
    div()
        .w_full()
        .max_h(px(500.0))
        .id("scroll-diff-content")
        .overflow_y_scroll()
        .overflow_x_scroll()
        .bg(background_color)
        .children(hunk_data.into_iter().map(move |(_hunk_idx, _header, _hunk_collapsed, lines)| {
            div()
                .w_full()
                .children(lines.into_iter().map(move |(prefix, content, old_ln, new_ln, line_type)| {
                    render_line(
                        prefix,
                        content,
                        old_ln,
                        new_ln,
                        line_type,
                        border_color,
                        surface_color,
                        text_color,
                        text_muted_color,
                        accent_color,
                        success_color,
                        error_color,
                        background_color,
                    )
                }))
        }))
}

/// Pre-compute hunk data for rendering
pub fn prepare_hunk_data(hunks: &[super::types::DiffHunk]) -> Vec<(usize, String, bool, Vec<(char, String, Option<usize>, Option<usize>, &'static str)>)> {
    hunks
        .iter()
        .enumerate()
        .map(|(idx, hunk)| {
            let lines: Vec<_> = hunk.lines.iter().map(|line| {
                match line {
                    DiffLine::Context { content, old_line, new_line } => (
                        '+', // not used for context
                        content.clone(),
                        Some(*old_line),
                        Some(*new_line),
                        "context",
                    ),
                    DiffLine::Added { content, new_line } => (
                        '+',
                        content.clone(),
                        None,
                        Some(*new_line),
                        "added",
                    ),
                    DiffLine::Removed { content, old_line } => (
                        '-',
                        content.clone(),
                        Some(*old_line),
                        None,
                        "removed",
                    ),
                    DiffLine::HunkHeader { header, .. } => (
                        '@',
                        header.clone(),
                        None,
                        None,
                        "header",
                    ),
                }
            }).collect();
            (idx, hunk.header.clone(), hunk.collapsed, lines)
        })
        .collect()
}
