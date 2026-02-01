//! Operations on FileEntry (loading, traversal, etc.)

use std::path::PathBuf;

use super::entry::FileEntry;

impl FileEntry {
    /// Toggle expanded state
    pub fn toggle_expanded(&mut self) {
        if self.is_directory() {
            self.is_expanded = !self.is_expanded;
        }
    }

    /// Load children for a directory
    pub fn load_children(&mut self) -> std::io::Result<()> {
        if !self.is_directory() {
            return Ok(());
        }

        self.children.clear();

        let mut entries: Vec<_> = std::fs::read_dir(&self.path)?
            .filter_map(|e| e.ok())
            .collect();

        // Sort: directories first, then alphabetically
        entries.sort_by(|a, b| {
            let a_is_dir = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
            let b_is_dir = b.file_type().map(|t| t.is_dir()).unwrap_or(false);

            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.file_name().cmp(&b.file_name()),
            }
        });

        for entry in entries {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            // Skip hidden files by default
            if name.starts_with('.') && name != ".gitignore" {
                continue;
            }

            let file_type = entry.file_type()?;
            let mut child = if file_type.is_dir() {
                FileEntry::directory(path, name, self.depth + 1)
            } else {
                FileEntry::file(path, name, self.depth + 1)
            };

            // Inherit git status (could be overridden by git integration)
            child.git_status = self.git_status;

            self.children.push(child);
        }

        Ok(())
    }

    /// Get visible entries (flattened tree respecting expansion)
    pub fn visible_entries(&self) -> Vec<&FileEntry> {
        let mut entries = vec![self];

        if self.is_expanded {
            for child in &self.children {
                entries.extend(child.visible_entries());
            }
        }

        entries
    }

    /// Get mutable visible entries
    pub fn visible_entries_mut(&mut self) -> Vec<&mut FileEntry> {
        let mut entries = Vec::new();

        // Use raw pointer to work around borrow checker
        let self_ptr = self as *mut FileEntry;
        unsafe {
            entries.push(&mut *self_ptr);
        }

        if self.is_expanded {
            for child in &mut self.children {
                entries.extend(child.visible_entries_mut());
            }
        }

        entries
    }

    /// Find entry by path
    pub fn find_by_path(&self, target: &PathBuf) -> Option<&FileEntry> {
        if &self.path == target {
            return Some(self);
        }

        for child in &self.children {
            if let Some(found) = child.find_by_path(target) {
                return Some(found);
            }
        }

        None
    }

    /// Find mutable entry by path
    pub fn find_by_path_mut(&mut self, target: &PathBuf) -> Option<&mut FileEntry> {
        if &self.path == target {
            return Some(self);
        }

        for child in &mut self.children {
            if let Some(found) = child.find_by_path_mut(target) {
                return Some(found);
            }
        }

        None
    }
}
