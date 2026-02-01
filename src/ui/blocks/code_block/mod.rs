//! Syntax highlighted code block component

mod code_view;
mod diff_view;
mod header;
mod render;
mod search_bar;
mod types;
mod view;

pub use types::*;
pub use view::CodeBlockView;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_match_creation() {
        let m = SearchMatch {
            line: 5,
            start: 10,
            end: 15,
        };
        assert_eq!(m.line, 5);
        assert_eq!(m.start, 10);
        assert_eq!(m.end, 15);
    }

    #[test]
    fn test_line_change_type_prefix() {
        assert_eq!(LineChangeType::Context.prefix(), " ");
        assert_eq!(LineChangeType::Added.prefix(), "+");
        assert_eq!(LineChangeType::Removed.prefix(), "-");
        assert_eq!(LineChangeType::ModifiedOld.prefix(), "-");
        assert_eq!(LineChangeType::ModifiedNew.prefix(), "+");
    }

    #[test]
    fn test_compute_diff_identical() {
        let old = "line1\nline2\nline3";
        let new = "line1\nline2\nline3";
        let diff = CodeBlockView::compute_diff(old, new);
        assert_eq!(diff.len(), 3);
        assert!(diff
            .iter()
            .all(|l| l.change_type == LineChangeType::Context));
    }

    #[test]
    fn test_compute_diff_addition() {
        let old = "line1\nline3";
        let new = "line1\nline2\nline3";
        let diff = CodeBlockView::compute_diff(old, new);

        let added_count = diff
            .iter()
            .filter(|l| {
                matches!(
                    l.change_type,
                    LineChangeType::Added | LineChangeType::ModifiedNew
                )
            })
            .count();
        assert!(added_count >= 1);
    }

    #[test]
    fn test_compute_diff_removal() {
        let old = "line1\nline2\nline3";
        let new = "line1\nline3";
        let diff = CodeBlockView::compute_diff(old, new);

        let removed_count = diff
            .iter()
            .filter(|l| {
                matches!(
                    l.change_type,
                    LineChangeType::Removed | LineChangeType::ModifiedOld
                )
            })
            .count();
        assert!(removed_count >= 1);
    }

    #[test]
    fn test_compute_diff_modification() {
        let old = "line1\nold_content\nline3";
        let new = "line1\nnew_content\nline3";
        let diff = CodeBlockView::compute_diff(old, new);

        // Should have old and new versions
        let has_old = diff
            .iter()
            .any(|l| l.change_type == LineChangeType::ModifiedOld);
        let has_new = diff
            .iter()
            .any(|l| l.change_type == LineChangeType::ModifiedNew);
        assert!(has_old && has_new);
    }

    #[test]
    fn test_diff_line_creation() {
        let line = DiffLine {
            content: "test content".to_string(),
            change_type: LineChangeType::Added,
            old_line_num: None,
            new_line_num: Some(5),
        };
        assert_eq!(line.content, "test content");
        assert_eq!(line.change_type, LineChangeType::Added);
        assert!(line.old_line_num.is_none());
        assert_eq!(line.new_line_num, Some(5));
    }

    #[test]
    fn test_display_mode_default() {
        let mode = CodeDisplayMode::default();
        assert_eq!(mode, CodeDisplayMode::Normal);
    }

    #[test]
    fn test_highlight_style_default() {
        let style = HighlightStyle::default();
        assert_eq!(style, HighlightStyle::Reference);
    }

    #[test]
    fn test_highlighted_range_single() {
        let range = HighlightedRange::single(5, HighlightStyle::Error);
        assert_eq!(range.start_line, 5);
        assert_eq!(range.end_line, 5);
        assert_eq!(range.style, HighlightStyle::Error);
        assert!(range.label.is_none());
    }

    #[test]
    fn test_highlighted_range_range() {
        let range = HighlightedRange::range(10, 15, HighlightStyle::Warning);
        assert_eq!(range.start_line, 10);
        assert_eq!(range.end_line, 15);
        assert_eq!(range.style, HighlightStyle::Warning);
    }

    #[test]
    fn test_highlighted_range_with_label() {
        let range = HighlightedRange::single(1, HighlightStyle::Info).with_label("Error location");
        assert_eq!(range.label, Some("Error location".to_string()));
    }

    #[test]
    fn test_highlighted_range_contains() {
        let range = HighlightedRange::range(5, 10, HighlightStyle::Success);
        assert!(!range.contains(4));
        assert!(range.contains(5));
        assert!(range.contains(7));
        assert!(range.contains(10));
        assert!(!range.contains(11));
    }

    #[test]
    fn test_highlighted_range_single_contains() {
        let range = HighlightedRange::single(5, HighlightStyle::Reference);
        assert!(!range.contains(4));
        assert!(range.contains(5));
        assert!(!range.contains(6));
    }
}
