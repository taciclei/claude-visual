//! File mention autocomplete

use super::utils::{fuzzy_match_files, FileMatch};
use super::ChatInput;
use crate::ai::mention::PartialMentionKind;
use gpui::*;
use std::path::Path;

/// Maximum number of files to index for autocomplete
const MAX_FILES: usize = 1000;

/// Default ignore patterns
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

impl ChatInput {
    /// Update file autocomplete based on partial mention
    pub(super) fn update_file_autocomplete(&mut self, cx: &mut Context<Self>) {
        if let Some(ref partial) = self.partial_mention {
            // Only show autocomplete for file mentions or unknown (could be file)
            match partial.kind {
                PartialMentionKind::File | PartialMentionKind::Unknown => {
                    // Extract the file query (after @file: or just after @)
                    let query = if partial.prefix.starts_with("file:") {
                        &partial.prefix[5..] // Remove "file:" prefix
                    } else {
                        &partial.prefix
                    };

                    // Get project files and filter
                    let project_files = self.get_project_files();
                    self.filtered_files = fuzzy_match_files(&project_files, query);
                    self.show_file_autocomplete = !self.filtered_files.is_empty();
                    self.selected_file_index = 0;
                }
                _ => {
                    self.show_file_autocomplete = false;
                    self.filtered_files.clear();
                }
            }
        } else {
            self.show_file_autocomplete = false;
            self.filtered_files.clear();
        }
        cx.notify();
    }

    /// Get list of project files for autocomplete
    fn get_project_files(&self) -> Vec<String> {
        if let Some(dir) = self.app_state.current_directory() {
            collect_files(&dir, MAX_FILES)
        } else {
            Vec::new()
        }
    }

    /// Select previous file in autocomplete
    pub(super) fn select_previous_file(&mut self, cx: &mut Context<Self>) {
        if !self.filtered_files.is_empty() {
            if self.selected_file_index > 0 {
                self.selected_file_index -= 1;
            } else {
                self.selected_file_index = self.filtered_files.len() - 1;
            }
            cx.notify();
        }
    }

    /// Select next file in autocomplete
    pub(super) fn select_next_file(&mut self, cx: &mut Context<Self>) {
        if !self.filtered_files.is_empty() {
            self.selected_file_index = (self.selected_file_index + 1) % self.filtered_files.len();
            cx.notify();
        }
    }

    /// Insert selected file from autocomplete
    pub(super) fn insert_selected_file(&mut self, cx: &mut Context<Self>) {
        if let Some(file_match) = self.filtered_files.get(self.selected_file_index).cloned() {
            if let Some(ref partial) = self.partial_mention {
                // Replace the partial mention with the full file path
                let before = &self.text[..partial.start];
                let after_cursor = if self.cursor_position < self.text.len() {
                    &self.text[self.cursor_position..]
                } else {
                    ""
                };

                // Format: @path (simple format)
                let mention = format!("@{} ", file_match.path);
                self.text = format!("{}{}{}", before, mention, after_cursor);
                self.cursor_position = partial.start + mention.len();
            }

            self.show_file_autocomplete = false;
            self.filtered_files.clear();
            self.partial_mention = None;
            self.update_mentions();
            cx.notify();
        }
    }

    /// Close file autocomplete
    pub(super) fn close_file_autocomplete(&mut self, cx: &mut Context<Self>) {
        self.show_file_autocomplete = false;
        self.filtered_files.clear();
        cx.notify();
    }

    /// Get match data for a file at index
    pub fn get_file_match(&self, index: usize) -> Option<&FileMatch> {
        self.filtered_files.get(index)
    }
}

/// Recursively collect files from a directory
fn collect_files(dir: &Path, max_files: usize) -> Vec<String> {
    let mut files = Vec::new();
    collect_files_recursive(dir, dir, &mut files, max_files);
    files
}

fn collect_files_recursive(base: &Path, dir: &Path, files: &mut Vec<String>, max_files: usize) {
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
                files.push(relative.display().to_string());
            }
        } else if path.is_dir() {
            collect_files_recursive(base, &path, files, max_files);
        }
    }
}
