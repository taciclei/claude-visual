//! Core FilePreviewPanel implementation

use std::path::PathBuf;
use std::sync::Arc;

use gpui::*;

use super::loader::load_file_preview;
use super::types::{FilePreviewEvent, PreviewState};
use crate::app::state::AppState;

impl EventEmitter<FilePreviewEvent> for FilePreviewPanel {}

/// File preview panel
pub struct FilePreviewPanel {
    pub(crate) app_state: Arc<AppState>,
    /// Current preview state
    pub(crate) state: PreviewState,
    /// Scroll offset for content
    pub(crate) scroll_offset: f32,
    /// Focus handle
    pub(crate) focus_handle: FocusHandle,
}

impl FilePreviewPanel {
    /// Create a new preview panel
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            state: PreviewState::Empty,
            scroll_offset: 0.0,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Load preview for a file
    pub fn preview_file(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        self.state = PreviewState::Loading(path.clone());
        self.scroll_offset = 0.0;
        cx.notify();

        // Load file content
        cx.spawn(async move |this, cx| {
            let result = std::thread::spawn(move || load_file_preview(&path)).join();

            let _ = this
                .update(cx, |this, cx| {
                    match result {
                        Ok(state) => this.state = state,
                        Err(_) => {
                            this.state = PreviewState::Error {
                                path: PathBuf::new(),
                                message: "Failed to load preview".to_string(),
                            };
                        }
                    }
                    cx.notify();
                })
                .ok();
        })
        .detach();
    }

    /// Clear the preview
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.state = PreviewState::Empty;
        cx.notify();
    }

    /// Check if showing a preview
    pub fn is_showing(&self) -> bool {
        !matches!(self.state, PreviewState::Empty)
    }

    /// Get the currently previewed path
    pub fn current_path(&self) -> Option<&PathBuf> {
        match &self.state {
            PreviewState::Empty => None,
            PreviewState::Loading(p) => Some(p),
            PreviewState::Loaded { path, .. } => Some(path),
            PreviewState::Binary { path, .. } => Some(path),
            PreviewState::TooLarge { path, .. } => Some(path),
            PreviewState::Error { path, .. } => Some(path),
        }
    }

    /// Format file size for display
    pub(crate) fn format_size(&self, bytes: u64) -> String {
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.1} KB", bytes as f64 / 1024.0)
        } else {
            format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
        }
    }
}

impl Focusable for FilePreviewPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preview_state_variants() {
        let empty = PreviewState::Empty;
        assert!(matches!(empty, PreviewState::Empty));

        let loading = PreviewState::Loading(PathBuf::from("/test"));
        assert!(matches!(loading, PreviewState::Loading(_)));
    }
}
