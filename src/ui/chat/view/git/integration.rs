//! Git integration methods

use gpui::*;

use crate::ui::chat::view::types::*;
use crate::ui::chat::view::core::ChatView;

impl ChatView {
    // ==================== Git Integration ====================

    /// Toggle git panel
    pub fn toggle_git_panel(&mut self, cx: &mut Context<Self>) {
        self.panels.git_panel = !self.panels.git_panel;
        cx.notify();
    }

    /// Update git info
    pub fn update_git_info(&mut self, info: GitInfo, cx: &mut Context<Self>) {
        self.git_info = Some(info);
        cx.notify();
    }

    /// Clear git info
    pub fn clear_git_info(&mut self, cx: &mut Context<Self>) {
        self.git_info = None;
        cx.notify();
    }

    /// Request a git status refresh from the workspace
    pub fn refresh_git_status(&mut self, cx: &mut Context<Self>) {
        cx.emit(ChatViewEvent::RefreshGitStatus);
    }
}
