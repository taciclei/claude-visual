//! Draft management for ChatView

use gpui::Context;

use super::ChatView;

impl ChatView {
    /// Save current input as draft (persists to settings)
    pub(crate) fn save_draft(&mut self, text: String, cx: &mut Context<Self>) {
        if !text.is_empty() {
            self.draft_text = Some(text.clone());
            self.draft_saved_at = Some(chrono::Utc::now());
            // Persist to settings and save to disk
            self.app_state.settings.update(cx, |settings, _| {
                settings.draft_text = text;
                let _ = settings.save(); // Best-effort save
            });
        } else {
            self.draft_text = None;
            self.draft_saved_at = None;
            // Clear from settings and save to disk
            self.app_state.settings.update(cx, |settings, _| {
                settings.draft_text.clear();
                let _ = settings.save(); // Best-effort save
            });
        }
        cx.notify();
    }

    /// Clear the saved draft (and persist)
    pub(crate) fn clear_draft(&mut self, cx: &mut Context<Self>) {
        self.draft_text = None;
        self.draft_saved_at = None;
        // Clear from settings and save to disk
        self.app_state.settings.update(cx, |settings, _| {
            settings.draft_text.clear();
            let _ = settings.save(); // Best-effort save
        });
        cx.notify();
    }

    /// Get draft text if available
    pub(crate) fn get_draft(&self) -> Option<&str> {
        self.draft_text.as_deref()
    }

    /// Check if there's a saved draft
    pub(crate) fn has_draft(&self) -> bool {
        self.draft_text.is_some()
    }

    /// Load draft from settings (call on startup)
    pub(crate) fn restore_draft_from_settings(&mut self, cx: &mut Context<Self>) {
        let draft = self.app_state.settings.read(cx).draft_text.clone();
        if !draft.is_empty() {
            self.draft_text = Some(draft.clone());
            self.draft_saved_at = None; // Don't know when it was saved
            // Set the text in the input
            self.input.update(cx, |input, cx| {
                input.set_text(draft, cx);
            });
            cx.notify();
        }
    }

    /// Sync current input text to draft (call before closing or periodically)
    pub(crate) fn sync_draft_to_settings(&mut self, cx: &mut Context<Self>) {
        let current_text = self.input.read(cx).text().to_string();
        if current_text != self.draft_text.as_deref().unwrap_or("") {
            self.save_draft(current_text, cx);
        }
    }
}
