//! Split view layout management

mod node;
mod render;
mod types;
mod view;

pub use node::SplitNode;
pub use types::{SplitDirection, SplitViewEvent};
pub use view::SplitView;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_node_pane_count() {
        let single = SplitNode::leaf();
        assert_eq!(single.pane_count(), 1);

        let split = SplitNode::horizontal(vec![SplitNode::leaf(), SplitNode::leaf()]);
        assert_eq!(split.pane_count(), 2);

        let nested = SplitNode::vertical(vec![
            SplitNode::horizontal(vec![SplitNode::leaf(), SplitNode::leaf()]),
            SplitNode::leaf(),
        ]);
        assert_eq!(nested.pane_count(), 3);
    }

    #[test]
    fn test_focus_management() {
        let mut root = SplitNode::horizontal(vec![SplitNode::leaf(), SplitNode::leaf()]);

        assert!(root.set_focus(0));
        assert_eq!(root.focused_index(), Some(0));

        assert!(root.set_focus(1));
        assert_eq!(root.focused_index(), Some(1));

        root.clear_focus();
        assert_eq!(root.focused_index(), None);
    }
}
