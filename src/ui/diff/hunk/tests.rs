//! Tests for hunk management

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::super::manager::DiffHunkManager;
    use super::super::managed::ManagedHunk;
    use super::super::types::{HunkAction, HunkStatus};

    #[test]
    fn test_hunk_status_color() {
        assert_eq!(HunkStatus::Applied.color(), (98, 181, 67));
        assert_eq!(HunkStatus::Rejected.color(), (224, 108, 117));
    }

    #[test]
    fn test_parse_diff() {
        let diff = r#"@@ -1,3 +1,4 @@
 context line
-removed line
+added line
+another added
 more context"#;

        let mut manager = DiffHunkManager::new(PathBuf::from("test.rs"));
        manager.parse_diff(diff);

        assert_eq!(manager.hunks.len(), 1);
        assert_eq!(manager.total_additions(), 2);
        assert_eq!(manager.total_deletions(), 1);
    }

    #[test]
    fn test_apply_action() {
        let mut manager = DiffHunkManager::new(PathBuf::from("test.rs"));
        manager.hunks.push(ManagedHunk::new(0, "@@ -1,1 +1,1 @@", 1, 1, 1, 1));

        manager.apply_action(0, HunkAction::Apply);
        assert_eq!(manager.hunks[0].status, HunkStatus::Applied);

        manager.apply_action(0, HunkAction::Reject);
        assert_eq!(manager.hunks[0].status, HunkStatus::Rejected);

        manager.apply_action(0, HunkAction::Reset);
        assert_eq!(manager.hunks[0].status, HunkStatus::Pending);
    }

    #[test]
    fn test_line_selection() {
        let mut hunk = ManagedHunk::new(0, "@@ -1,2 +1,2 @@", 1, 2, 1, 2);
        hunk.add_line("unchanged", ' ', Some(1), Some(1));
        hunk.add_line("removed", '-', Some(2), None);
        hunk.add_line("added", '+', None, Some(2));

        assert_eq!(hunk.selected_additions(), 1);
        assert_eq!(hunk.selected_deletions(), 1);

        hunk.toggle_line(1);
        assert_eq!(hunk.selected_deletions(), 0);

        hunk.select_all();
        assert_eq!(hunk.selected_deletions(), 1);
    }
}
