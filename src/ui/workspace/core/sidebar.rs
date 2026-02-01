//! Sidebar operations for Workspace

use super::super::types::SidebarTab;
use super::workspace::Workspace;
use gpui::*;

impl Workspace {
    /// Switch to a specific sidebar tab
    pub fn switch_sidebar_tab(&mut self, tab: SidebarTab, cx: &mut Context<Self>) {
        self.sidebar_tab = tab;
        // Refresh content when switching tabs
        match tab {
            SidebarTab::History => {
                self.history_sidebar.update(cx, |history, cx| {
                    history.refresh(cx);
                });
            }
            SidebarTab::Git => {
                self.worktree_panel.update(cx, |panel, cx| {
                    panel.refresh(cx);
                });
            }
            SidebarTab::Team => {
                // Team panel refresh would be triggered here
            }
            SidebarTab::Projects => {
                // Projects panel refresh would be triggered here
            }
            SidebarTab::Files => {
                self.file_tree.update(cx, |tree, cx| {
                    tree.refresh(cx);
                });
            }
        }
        cx.notify();
    }

    /// Toggle sidebar visibility
    pub fn toggle_sidebar(&mut self, cx: &mut Context<Self>) {
        self.show_sidebar = !self.show_sidebar;
        cx.notify();
    }
}
