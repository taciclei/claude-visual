//! Diff preview operations (show, hide, toggle)

use crate::ui::workspace::core::Workspace;
use gpui::*;

impl Workspace {
    /// Show diff preview for a file
    pub(in crate::ui::workspace) fn show_diff_preview(
        &mut self,
        path: String,
        cx: &mut Context<Self>,
    ) {
        if let Some(cwd) = self.app_state.current_directory() {
            if let Ok(repo) = crate::git::repository::Repository::open(&cwd) {
                match repo.file_diff(&path) {
                    Ok(diff) => {
                        if diff.is_empty() {
                            // No changes to show
                            tracing::info!("No diff available for {}", path);
                            return;
                        }
                        self.diff_preview = Some((path, diff));
                        cx.notify();
                    }
                    Err(e) => {
                        tracing::error!("Failed to get diff for {}: {}", path, e);
                    }
                }
            }
        }
    }

    /// Hide diff preview
    pub(in crate::ui::workspace) fn hide_diff_preview(&mut self, cx: &mut Context<Self>) {
        if self.diff_preview.is_some() {
            self.diff_preview = None;
            cx.notify();
        }
    }

    /// Toggle diff display mode (unified/side-by-side)
    pub(in crate::ui::workspace) fn toggle_diff_mode(&mut self, cx: &mut Context<Self>) {
        self.diff_side_by_side = !self.diff_side_by_side;
        cx.notify();
    }
}
