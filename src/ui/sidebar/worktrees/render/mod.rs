//! Rendering implementation for worktree panel

mod header;
mod file_status;
mod no_changes;
mod worktrees_list;
mod create_button;

use gpui::*;
use gpui::prelude::*;

use super::types::WorktreePanel;
use header::render_header;
use file_status::render_file_status;
use no_changes::{render_no_changes, render_no_git_repo};
use worktrees_list::render_worktrees_list;
use create_button::render_create_button;

impl Render for WorktreePanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();
        let is_git_repo = self.is_git_repo;
        let current_branch = self.current_branch.clone();

        // Pre-compute worktree items for rendering
        let worktree_items: Vec<_> = self.worktrees
            .iter()
            .enumerate()
            .map(|(idx, wt)| {
                let is_selected = self.selected_worktree == Some(idx);
                (
                    idx,
                    wt.branch.clone().unwrap_or_else(|| "detached".to_string()),
                    wt.path.display().to_string(),
                    wt.is_main,
                    wt.is_locked,
                    is_selected,
                )
            })
            .collect();

        // Pre-compute file status items
        let staged_files: Vec<_> = self.file_statuses
            .iter()
            .filter(|f| f.is_staged())
            .map(|f| (f.path.clone(), f.status_char(), f.index_status))
            .collect();

        let unstaged_files: Vec<_> = self.file_statuses
            .iter()
            .filter(|f| f.is_unstaged() && !f.is_staged())
            .map(|f| (f.path.clone(), f.status_char(), f.workdir_status))
            .collect();

        let has_changes = !staged_files.is_empty() || !unstaged_files.is_empty();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.colors.surface)
            // Branch header (when git repo) or not-git-repo message
            .child(render_header(&theme, is_git_repo, current_branch, has_changes))
            // Main content area
            .child(
                div()
                    .flex_1()
                    .id("scroll-worktrees")
                    .overflow_y_scroll()
                    // Not a git repo message
                    .when(!is_git_repo, |d| {
                        d.child(render_no_git_repo(&theme))
                    })
                    // File status section
                    .when(is_git_repo && has_changes, |d| {
                        d.child(render_file_status(self, &theme, staged_files, unstaged_files, cx))
                    })
                    // No changes message
                    .when(is_git_repo && !has_changes, |d| {
                        d.child(render_no_changes(&theme))
                    })
                    // Worktrees section (collapsed by default, shown if multiple)
                    .when(is_git_repo && worktree_items.len() > 1, |d| {
                        d.child(render_worktrees_list(self, &theme, worktree_items, cx))
                    }),
            )
            // Create worktree button (only show if git repo)
            .when(is_git_repo, |d| {
                d.child(render_create_button(&theme, cx))
            })
    }
}
