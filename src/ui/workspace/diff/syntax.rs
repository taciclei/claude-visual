//! Syntax highlighting for diff lines

use crate::ui::workspace::core::Workspace;

impl Workspace {
    /// Highlight a diff line, stripping the leading +/- and applying syntax colors
    pub(in crate::ui::workspace) fn highlight_diff_line(
        &self,
        line: &str,
        language: Option<&str>,
        theme: &crate::app::theme::Theme,
    ) -> Vec<crate::syntax::HighlightedSpan> {
        // Strip the diff prefix (+, -, or space)
        let (prefix, code) =
            if line.starts_with('+') || line.starts_with('-') || line.starts_with(' ') {
                (line.chars().next(), &line[1..])
            } else {
                (None, line)
            };

        // If no language, return plain text
        let Some(lang) = language else {
            return vec![crate::syntax::HighlightedSpan {
                text: line.to_string(),
                color: None,
            }];
        };

        // Get syntax highlighted spans
        let mut highlighter = self.syntax_highlighter.write().unwrap();
        let spans = highlighter.highlight_line(code, Some(lang), &theme.syntax);

        // If we stripped a prefix, add it back
        if let Some(p) = prefix {
            let mut result = vec![crate::syntax::HighlightedSpan {
                text: p.to_string(),
                color: None,
            }];
            result.extend(spans);
            result
        } else {
            spans
        }
    }
}
