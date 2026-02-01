//! Tree action methods

use gpui::*;
use super::tree_state::Tree;
use super::types::{TreeNode, TreeEvent};

impl Tree {
    /// Expand node
    pub fn expand(&mut self, id: &str, cx: &mut Context<Self>) {
        if !self.expanded.contains(&id.to_string()) {
            self.expanded.push(id.to_string());
            cx.emit(TreeEvent::Expanded(id.to_string()));
            cx.notify();
        }
    }

    /// Collapse node
    pub fn collapse(&mut self, id: &str, cx: &mut Context<Self>) {
        if self.expanded.contains(&id.to_string()) {
            self.expanded.retain(|i| i != id);
            cx.emit(TreeEvent::Collapsed(id.to_string()));
            cx.notify();
        }
    }

    /// Toggle node expansion
    pub fn toggle(&mut self, id: &str, cx: &mut Context<Self>) {
        if self.expanded.contains(&id.to_string()) {
            self.collapse(id, cx);
        } else {
            self.expand(id, cx);
        }
    }

    /// Select node
    pub fn select(&mut self, id: &str, cx: &mut Context<Self>) {
        if self.multi_select {
            if !self.selected_multiple.contains(&id.to_string()) {
                self.selected_multiple.push(id.to_string());
            }
        } else {
            self.selected = Some(id.to_string());
        }
        cx.emit(TreeEvent::Selected(id.to_string()));
        cx.notify();
    }

    /// Deselect node
    pub fn deselect(&mut self, id: &str, cx: &mut Context<Self>) {
        if self.multi_select {
            self.selected_multiple.retain(|i| i != id);
        } else if self.selected.as_deref() == Some(id) {
            self.selected = None;
        }
        cx.notify();
    }

    /// Expand all nodes
    pub fn expand_all(&mut self, cx: &mut Context<Self>) {
        fn collect_ids(nodes: &[TreeNode], ids: &mut Vec<String>) {
            for node in nodes {
                if node.has_children() {
                    ids.push(node.id.clone());
                    collect_ids(&node.children, ids);
                }
            }
        }

        let mut ids = Vec::new();
        collect_ids(&self.nodes, &mut ids);
        self.expanded = ids;
        cx.notify();
    }

    /// Collapse all nodes
    pub fn collapse_all(&mut self, cx: &mut Context<Self>) {
        self.expanded.clear();
        cx.notify();
    }
}
