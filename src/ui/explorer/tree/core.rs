//! Core file tree state and operations

use gpui::*;
use std::path::PathBuf;

use super::types::FileTreeEvent;
use crate::ui::explorer::file_item::{FileEntry, GitStatus};

/// File tree state
pub struct FileTree {
    /// Root entry
    pub(crate) root: Option<FileEntry>,
    /// Root path
    pub(crate) root_path: Option<PathBuf>,
    /// Selected path
    pub(crate) selected_path: Option<PathBuf>,
    /// Hovered path (for preview)
    pub(crate) hovered_path: Option<PathBuf>,
    /// Focus handle
    pub(crate) focus_handle: FocusHandle,
    /// Show hidden files
    pub(crate) show_hidden: bool,
    /// Filter pattern
    pub(crate) filter: String,
    /// Git status map
    pub(crate) git_statuses: std::collections::HashMap<PathBuf, GitStatus>,
    /// Is loading
    pub(crate) is_loading: bool,
    /// Error message
    pub(crate) error: Option<String>,
}

impl FileTree {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            root: None,
            root_path: None,
            selected_path: None,
            hovered_path: None,
            focus_handle: cx.focus_handle(),
            show_hidden: false,
            filter: String::new(),
            git_statuses: std::collections::HashMap::new(),
            is_loading: false,
            error: None,
        }
    }

    /// Set root directory
    pub fn set_root(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        self.root_path = Some(path.clone());
        self.is_loading = true;
        self.error = None;
        cx.notify();

        // Load in background
        cx.spawn(async move |this, cx| {
            let result = std::thread::spawn(move || {
                let mut root = FileEntry::directory(
                    path.clone(),
                    path.file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| path.display().to_string()),
                    0,
                );
                root.is_expanded = true;
                root.load_children().ok();
                root
            })
            .join();

            let _ = this
                .update(cx, |this, cx| {
                    match result {
                        Ok(root) => {
                            this.root = Some(root);
                        }
                        Err(_) => {
                            this.error = Some("Failed to load directory".to_string());
                        }
                    }
                    this.is_loading = false;
                    cx.notify();
                })
                .ok();
        })
        .detach();
    }

    /// Refresh the tree
    pub fn refresh(&mut self, cx: &mut Context<Self>) {
        if let Some(path) = self.root_path.clone() {
            self.set_root(path, cx);
        }
        cx.emit(FileTreeEvent::RefreshRequested);
    }

    /// Toggle hidden files
    pub fn toggle_hidden(&mut self, cx: &mut Context<Self>) {
        self.show_hidden = !self.show_hidden;
        self.refresh(cx);
    }

    /// Set filter
    pub fn set_filter(&mut self, filter: String, cx: &mut Context<Self>) {
        self.filter = filter;
        cx.notify();
    }

    /// Toggle expansion of a directory
    pub fn toggle_expand(&mut self, path: &PathBuf, cx: &mut Context<Self>) {
        if let Some(ref mut root) = self.root {
            if let Some(entry) = root.find_by_path_mut(path) {
                if entry.is_directory() {
                    entry.toggle_expanded();

                    // Load children if expanding and empty
                    if entry.is_expanded && entry.children.is_empty() {
                        entry.load_children().ok();
                    }

                    cx.notify();
                }
            }
        }
    }

    /// Select a file
    pub fn select(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        self.selected_path = Some(path.clone());
        cx.emit(FileTreeEvent::FileSelected(path));
        cx.notify();
    }

    /// Open a file (double click)
    pub fn open(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        cx.emit(FileTreeEvent::FileOpened(path));
    }

    /// Add file to context
    pub fn add_to_context(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        cx.emit(FileTreeEvent::FileAddedToContext(path));
    }

    /// Update git status
    pub fn set_git_status(&mut self, path: PathBuf, status: GitStatus, cx: &mut Context<Self>) {
        self.git_statuses.insert(path.clone(), status);

        // Update entry
        if let Some(ref mut root) = self.root {
            if let Some(entry) = root.find_by_path_mut(&path) {
                entry.git_status = Some(status);
            }
        }

        cx.notify();
    }

    /// Check if entry matches filter
    pub(crate) fn matches_filter(&self, entry: &FileEntry) -> bool {
        if self.filter.is_empty() {
            return true;
        }
        entry
            .name
            .to_lowercase()
            .contains(&self.filter.to_lowercase())
    }
}

impl EventEmitter<FileTreeEvent> for FileTree {}

impl Focusable for FileTree {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
