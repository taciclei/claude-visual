//! Context and input management for ChatView
//!
//! This module contains methods for:
//! - Context files management (add, remove, clear, get)
//! - File mentions handling (insert mentions into input)
//! - Recent files tracking (track, add to context, clear)
//! - File picker (toggle, update query, select files)
//! - Input processing (text operations)
//! - Slash commands (insert into input)

use super::core::ChatView;
use super::types::{ContextFile, FilePickerItem, RecentFile};
use gpui::*;
use std::path::Path;

impl ChatView {
    // ==================== Context Panel ====================

    /// Toggle context panel
    pub fn toggle_context_panel(&mut self, cx: &mut Context<Self>) {
        self.panels.context_panel = !self.panels.context_panel;
        cx.notify();
    }

    /// Add a file to context
    pub fn add_context_file(&mut self, path: impl Into<String>, cx: &mut Context<Self>) {
        let path = path.into();
        // Avoid duplicates
        if !self.context_files.iter().any(|f| f.path == path) {
            self.context_files.push(ContextFile::from_path(path));
            cx.notify();
        }
    }

    /// Remove a file from context
    pub fn remove_context_file(&mut self, path: &str, cx: &mut Context<Self>) {
        self.context_files.retain(|f| f.path != path);
        cx.notify();
    }

    /// Clear all context files
    pub fn clear_context_files(&mut self, cx: &mut Context<Self>) {
        self.context_files.clear();
        cx.notify();
    }

    /// Get total tokens in context files
    pub fn context_files_tokens(&self) -> u64 {
        self.context_files.iter().map(|f| f.tokens).sum()
    }

    /// Get context file count
    pub fn context_file_count(&self) -> usize {
        self.context_files.len()
    }

    // ==================== Recent Files ====================

    /// Toggle recent files panel
    pub fn toggle_recent_files_panel(&mut self, cx: &mut Context<Self>) {
        self.panels.recent_files_panel = !self.panels.recent_files_panel;
        cx.notify();
    }

    /// Track a file as recently accessed
    pub fn track_recent_file(&mut self, path: impl Into<String>, cx: &mut Context<Self>) {
        let path = path.into();

        // Check if file already exists in recent files
        if let Some(existing) = self.recent_files.iter_mut().find(|f| f.path == path) {
            existing.touch();
        } else {
            // Add new recent file
            self.recent_files.push(RecentFile::from_path(path));
        }

        // Sort by access time (most recent first)
        self.recent_files
            .sort_by(|a, b| b.accessed_at.cmp(&a.accessed_at));

        // Limit to max recent files
        self.recent_files.truncate(self.max_recent_files);

        cx.notify();
    }

    /// Get recent files count
    pub fn recent_files_count(&self) -> usize {
        self.recent_files.len()
    }

    /// Clear recent files
    pub fn clear_recent_files(&mut self, cx: &mut Context<Self>) {
        self.recent_files.clear();
        cx.notify();
    }

    /// Add recent file to context
    pub fn add_recent_file_to_context(&mut self, path: &str, cx: &mut Context<Self>) {
        self.add_context_file(path, cx);
        // Also track as recently accessed
        self.track_recent_file(path, cx);
    }

    /// Insert file mention into input
    pub fn insert_file_mention(&mut self, path: &str, cx: &mut Context<Self>) {
        let mention = format!("@file:{} ", path);
        self.input.update(cx, |input, cx| {
            let current_text = input.text().to_string();
            input.set_text(format!("{}{}", current_text, mention), cx);
        });
        // Track as recently accessed
        self.track_recent_file(path, cx);
    }

    // ==================== File Picker ====================

    /// Toggle file picker
    pub fn toggle_file_picker(&mut self, cx: &mut Context<Self>) {
        self.file_picker.visible = !self.file_picker.visible;
        if self.file_picker.visible {
            self.file_picker.query.clear();
            // Load initial file list from project
            self.load_file_picker_files();
        }
        cx.notify();
    }

    /// Load files into the file picker
    fn load_file_picker_files(&mut self) {
        if let Some(dir) = self.app_state.current_directory() {
            self.file_picker.results = collect_project_files(&dir, 100);
        } else {
            self.file_picker.results.clear();
        }
    }

    /// Update file picker query and filter results
    pub fn update_file_picker_query(&mut self, query: String, cx: &mut Context<Self>) {
        self.file_picker.query = query.clone();

        // Filter file list based on query
        if let Some(dir) = self.app_state.current_directory() {
            let all_files = collect_project_files(&dir, 500);

            if query.is_empty() {
                // Show first 100 files when no query
                self.file_picker.results = all_files.into_iter().take(100).collect();
            } else {
                // Filter by query (case-insensitive)
                let query_lower = query.to_lowercase();
                self.file_picker.results = all_files
                    .into_iter()
                    .filter(|f| {
                        f.name.to_lowercase().contains(&query_lower)
                            || f.path.to_lowercase().contains(&query_lower)
                    })
                    .take(50)
                    .collect();
            }
        }
        cx.notify();
    }

    /// Select file from picker
    pub fn select_file(&mut self, path: &str, cx: &mut Context<Self>) {
        // Insert file mention into input
        self.input.update(cx, |input, cx| {
            input.insert_text(&format!("@{} ", path), cx);
        });
        self.file_picker.visible = false;
        cx.notify();
    }

    // ==================== Slash Commands ====================

    /// Insert a slash command into the input
    pub fn insert_slash_command(&mut self, command: &str, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| {
            input.clear(cx);
            input.insert_text(&format!("/{} ", command), cx);
        });
        // Close any open panels to focus on the input
        self.panels.session_details = false;
        cx.notify();
    }
}

// ==================== File Collection Helpers ====================

/// Ignore patterns for file collection
const IGNORE_PATTERNS: &[&str] = &[
    "node_modules",
    ".git",
    "target",
    "dist",
    "build",
    ".next",
    ".nuxt",
    ".cache",
    "__pycache__",
    ".venv",
    "venv",
    ".env",
    "coverage",
    ".nyc_output",
];

/// Collect project files for the file picker
fn collect_project_files(dir: &Path, max_files: usize) -> Vec<FilePickerItem> {
    let mut files = Vec::new();
    collect_files_recursive(dir, dir, &mut files, max_files);
    // Sort by name for better UX
    files.sort_by(|a, b| a.name.cmp(&b.name));
    files
}

fn collect_files_recursive(
    base: &Path,
    dir: &Path,
    files: &mut Vec<FilePickerItem>,
    max_files: usize,
) {
    if files.len() >= max_files {
        return;
    }

    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        if files.len() >= max_files {
            return;
        }

        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        // Skip hidden files and ignored directories
        if name.starts_with('.') || IGNORE_PATTERNS.contains(&name.as_str()) {
            continue;
        }

        if path.is_file() {
            // Get relative path from base
            if let Ok(relative) = path.strip_prefix(base) {
                let extension = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|s| s.to_lowercase());

                let size = std::fs::metadata(&path).ok().map(|m| m.len());

                files.push(FilePickerItem {
                    path: relative.display().to_string(),
                    name: name.clone(),
                    is_dir: false,
                    extension,
                    size,
                    modified: None,
                });
            }
        } else if path.is_dir() {
            collect_files_recursive(base, &path, files, max_files);
        }
    }
}
