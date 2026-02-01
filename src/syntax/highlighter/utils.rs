//! Utility functions for syntax highlighting

use gpui::Hsla;

use crate::app::theme::SyntaxColors;

use super::types::HighlightedSpan;

/// Map capture names to syntax colors
pub(crate) fn capture_name_to_color(name: &str, colors: &SyntaxColors) -> Option<Hsla> {
    match name {
        "keyword" | "keyword.control" | "keyword.function" | "keyword.return" |
        "keyword.operator" | "keyword.import" | "keyword.export" => Some(colors.keyword),

        "string" | "string.special" => Some(colors.string),

        "number" | "constant.numeric" | "float" => Some(colors.number),

        "comment" | "comment.line" | "comment.block" => Some(colors.comment),

        "function" | "function.call" | "function.method" | "method" => Some(colors.function),

        "variable" | "variable.parameter" | "variable.builtin" | "parameter" => Some(colors.variable),

        "constant" | "constant.builtin" | "boolean" => Some(colors.constant),

        "type" | "type.builtin" | "class" | "constructor" => Some(colors.type_name),

        "operator" => Some(colors.operator),

        "punctuation" | "punctuation.bracket" | "punctuation.delimiter" |
        "punctuation.special" => Some(colors.punctuation),

        "property" | "attribute" => Some(colors.variable),

        _ => None,
    }
}

/// Build spans from code and highlight ranges
pub(crate) fn build_spans(code: &str, highlights: &[(usize, usize, Hsla)]) -> Vec<HighlightedSpan> {
    if highlights.is_empty() {
        return vec![HighlightedSpan {
            text: code.to_string(),
            color: None,
        }];
    }

    let mut spans = Vec::new();
    let mut pos = 0;

    for &(start, end, color) in highlights {
        // Clamp to valid range
        let start = start.min(code.len());
        let end = end.min(code.len());

        if start > pos {
            // Add unhighlighted text before this highlight
            spans.push(HighlightedSpan {
                text: code[pos..start].to_string(),
                color: None,
            });
        }

        if end > start && start >= pos {
            // Add highlighted text
            spans.push(HighlightedSpan {
                text: code[start..end].to_string(),
                color: Some(color),
            });
            pos = end;
        }
    }

    // Add any remaining text
    if pos < code.len() {
        spans.push(HighlightedSpan {
            text: code[pos..].to_string(),
            color: None,
        });
    }

    // Filter out empty spans
    spans.into_iter().filter(|s| !s.text.is_empty()).collect()
}
