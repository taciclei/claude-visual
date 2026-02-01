//! Core state management methods

use gpui::*;

use super::super::types::CodeDisplayMode;
use super::CodeBlockView;

impl CodeBlockView {
    /// Toggle collapsed state
    pub fn toggle_collapsed(&mut self, cx: &mut Context<Self>) {
        self.collapsed = !self.collapsed;
        cx.notify();
    }

    /// Copy code to clipboard
    pub fn copy_to_clipboard(&mut self, cx: &mut Context<Self>) {
        cx.write_to_clipboard(ClipboardItem::new_string(self.code.clone()));
        tracing::info!("Code copied to clipboard");

        // Show "Copied!" feedback
        self.show_copied_feedback = true;
        cx.notify();

        // Auto-hide after 2 seconds
        cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(std::time::Duration::from_secs(2))
                .await;
            let _ = this.update(cx, |view, cx| {
                view.show_copied_feedback = false;
                cx.notify();
            });
        })
        .detach();
    }

    /// Toggle between normal and diff display modes
    pub fn toggle_display_mode(&mut self, cx: &mut Context<Self>) {
        if self.old_code.is_some() {
            self.display_mode = match self.display_mode {
                CodeDisplayMode::Normal => CodeDisplayMode::Diff,
                CodeDisplayMode::Diff => CodeDisplayMode::Normal,
            };
            cx.notify();
        }
    }

    /// Set the old code for diff comparison
    pub fn set_old_code(&mut self, old_code: String, cx: &mut Context<Self>) {
        self.diff_lines = Self::compute_diff(&old_code, &self.code);
        self.old_code = Some(old_code);
        cx.notify();
    }

    /// Check if diff mode is available
    pub fn has_diff(&self) -> bool {
        self.old_code.is_some()
    }
}
