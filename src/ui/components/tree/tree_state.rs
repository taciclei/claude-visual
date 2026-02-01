//! Tree state management

use std::sync::Arc;
use gpui::*;
use crate::app::state::AppState;
use super::types::*;

/// Tree component
pub struct Tree {
    pub(super) app_state: Arc<AppState>,
    /// Root nodes
    pub(super) nodes: Vec<TreeNode>,
    /// Expanded node IDs
    pub(super) expanded: Vec<String>,
    /// Selected node ID
    pub(super) selected: Option<String>,
    /// Focused node ID
    pub(super) focused: Option<String>,
    /// Style variant
    pub(super) style: TreeStyle,
    /// Indent size in pixels
    pub(super) indent: f32,
    /// Show root nodes
    pub(super) show_root: bool,
    /// Whether to allow multi-select
    pub(super) multi_select: bool,
    /// Multiple selected nodes
    pub(super) selected_multiple: Vec<String>,
    /// Focus handle
    pub(super) focus_handle: FocusHandle,
}

impl Tree {
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            nodes: Vec::new(),
            expanded: Vec::new(),
            selected: None,
            focused: None,
            style: TreeStyle::default(),
            indent: 20.0,
            show_root: true,
            multi_select: false,
            selected_multiple: Vec::new(),
            focus_handle: cx.focus_handle(),
        }
    }

    /// Set nodes
    pub fn set_nodes(&mut self, nodes: Vec<TreeNode>, cx: &mut Context<Self>) {
        self.nodes = nodes;
        cx.notify();
    }

    /// Set style
    pub fn set_style(&mut self, style: TreeStyle, cx: &mut Context<Self>) {
        self.style = style;
        cx.notify();
    }

    /// Set indent
    pub fn set_indent(&mut self, indent: f32, cx: &mut Context<Self>) {
        self.indent = indent;
        cx.notify();
    }

    /// Set show root
    pub fn set_show_root(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_root = show;
        cx.notify();
    }

    /// Set multi-select
    pub fn set_multi_select(&mut self, multi: bool, cx: &mut Context<Self>) {
        self.multi_select = multi;
        if !multi {
            self.selected_multiple.clear();
        }
        cx.notify();
    }

    /// Is node expanded
    pub fn is_expanded(&self, id: &str) -> bool {
        self.expanded.contains(&id.to_string())
    }

    /// Get selected node
    pub fn selected(&self) -> Option<&str> {
        self.selected.as_deref()
    }
}
