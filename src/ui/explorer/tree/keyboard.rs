//! Keyboard navigation for file tree

use gpui::*;
use std::path::PathBuf;

use super::core::FileTree;

impl FileTree {
    /// Handle key events
    pub(crate) fn handle_key(&mut self, event: &KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        match event.keystroke.key.as_str() {
            "up" => self.select_previous(cx),
            "down" => self.select_next(cx),
            "left" => {
                if let Some(path) = &self.selected_path.clone() {
                    if let Some(ref root) = self.root {
                        if let Some(entry) = root.find_by_path(path) {
                            if entry.is_expanded && entry.is_directory() {
                                self.toggle_expand(path, cx);
                            }
                        }
                    }
                }
            }
            "right" | "enter" => {
                if let Some(path) = &self.selected_path.clone() {
                    if let Some(ref root) = self.root {
                        if let Some(entry) = root.find_by_path(path) {
                            if entry.is_directory() {
                                self.toggle_expand(path, cx);
                            } else {
                                self.open(path.clone(), cx);
                            }
                        }
                    }
                }
            }
            "space" => {
                if let Some(path) = &self.selected_path.clone() {
                    self.add_to_context(path.clone(), cx);
                }
            }
            "r" if event.keystroke.modifiers.platform => {
                self.refresh(cx);
            }
            "h" if event.keystroke.modifiers.platform => {
                self.toggle_hidden(cx);
            }
            _ => {}
        }
    }

    /// Select previous item
    pub(crate) fn select_previous(&mut self, cx: &mut Context<Self>) {
        if let Some(ref root) = self.root {
            let entries: Vec<PathBuf> = root.visible_entries()
                .iter()
                .map(|e| e.path.clone())
                .collect();

            if let Some(current) = &self.selected_path {
                if let Some(idx) = entries.iter().position(|p| p == current) {
                    if idx > 0 {
                        self.select(entries[idx - 1].clone(), cx);
                    }
                }
            } else if !entries.is_empty() {
                self.select(entries[0].clone(), cx);
            }
        }
    }

    /// Select next item
    pub(crate) fn select_next(&mut self, cx: &mut Context<Self>) {
        if let Some(ref root) = self.root {
            let entries: Vec<PathBuf> = root.visible_entries()
                .iter()
                .map(|e| e.path.clone())
                .collect();

            if let Some(current) = &self.selected_path {
                if let Some(idx) = entries.iter().position(|p| p == current) {
                    if idx + 1 < entries.len() {
                        self.select(entries[idx + 1].clone(), cx);
                    }
                }
            } else if !entries.is_empty() {
                self.select(entries[0].clone(), cx);
            }
        }
    }
}
