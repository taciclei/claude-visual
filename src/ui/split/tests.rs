//! Tests for split view system

#[cfg(test)]
mod tests {
    use super::super::types::*;

    #[test]
    fn test_pane_count() {
        let single = SplitNode::pane();
        assert_eq!(single.pane_count(), 1);

        let horizontal = SplitNode::horizontal(vec![
            SplitNode::pane(),
            SplitNode::pane(),
        ]);
        assert_eq!(horizontal.pane_count(), 2);

        let nested = SplitNode::vertical(vec![
            SplitNode::horizontal(vec![
                SplitNode::pane(),
                SplitNode::pane(),
            ]),
            SplitNode::pane(),
        ]);
        assert_eq!(nested.pane_count(), 3);
    }

    #[test]
    fn test_focus_management() {
        let mut root = SplitNode::horizontal(vec![
            SplitNode::pane(),
            SplitNode::pane(),
        ]);

        assert!(root.set_focus(0));
        assert_eq!(root.focused_pane_index(), Some(0));

        assert!(root.set_focus(1));
        assert_eq!(root.focused_pane_index(), Some(1));

        root.clear_focus();
        assert_eq!(root.focused_pane_index(), None);
    }
}
