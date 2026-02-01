//! Tests for diff viewer components

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_diff_viewer_creation() {
        let viewer = DiffViewer::new("dv-1")
            .file_path("src/main.rs")
            .view_mode(DiffViewMode::Unified);
        assert_eq!(viewer.view_mode, DiffViewMode::Unified);
        assert!(viewer.file_path.is_some());
    }

    #[test]
    fn test_diff_line_types() {
        let added = DiffLine::added("new line", 5);
        let removed = DiffLine::removed("old line", 4);
        let context = DiffLine::context("unchanged", 3, 4);

        assert_eq!(added.line_type, DiffLineType::Added);
        assert_eq!(removed.line_type, DiffLineType::Removed);
        assert_eq!(context.line_type, DiffLineType::Context);
    }

    #[test]
    fn test_diff_viewer_stats() {
        let viewer = DiffViewer::new("dv-2").lines(vec![
            DiffLine::added("a", 1),
            DiffLine::added("b", 2),
            DiffLine::removed("c", 1),
            DiffLine::context("d", 2, 3),
        ]);
        assert_eq!(viewer.additions(), 2);
        assert_eq!(viewer.deletions(), 1);
    }

    #[test]
    fn test_split_diff_viewer() {
        let viewer = SplitDiffViewer::new("sdv-1")
            .old_lines(vec![(Some(1), "old")])
            .new_lines(vec![(Some(1), "new")])
            .labels("Before", "After");
        assert_eq!(viewer.old_label.as_ref(), "Before");
    }

    #[test]
    fn test_diff_stat() {
        let stat = DiffStat::new("ds-1")
            .files_changed(5)
            .additions(100)
            .deletions(50);
        assert_eq!(stat.total(), 150);
        assert_eq!(stat.files_changed, 5);
    }

    #[test]
    fn test_file_change_badge() {
        let badge = FileChangeBadge::new("fcb-1", "src/lib.rs")
            .change_type(FileChangeType::Added);
        assert_eq!(badge.change_type, FileChangeType::Added);
    }

    #[test]
    fn test_inline_change() {
        let change = InlineChange::new("ic-1", "old value", "new value").strikethrough(true);
        assert!(change.strikethrough);
        assert_eq!(change.old_text.as_ref(), "old value");
    }
}
