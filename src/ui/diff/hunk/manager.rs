//! Diff hunk manager

use std::path::PathBuf;

use super::managed::ManagedHunk;
use super::types::{HunkAction, HunkStatus};

/// Manager for diff hunks in a file
#[derive(Debug, Clone)]
pub struct DiffHunkManager {
    /// File path being diffed
    pub(crate) file_path: PathBuf,
    /// All hunks in this diff
    pub(crate) hunks: Vec<ManagedHunk>,
    /// Currently focused hunk
    pub(crate) focused_hunk: Option<usize>,
}

impl DiffHunkManager {
    /// Create a new hunk manager
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            file_path,
            hunks: Vec::new(),
            focused_hunk: None,
        }
    }

    /// Parse unified diff text into managed hunks
    pub fn parse_diff(&mut self, diff_text: &str) {
        self.hunks.clear();
        let mut current_hunk: Option<ManagedHunk> = None;
        let mut hunk_id = 0;
        let mut old_line = 0;
        let mut new_line = 0;

        for line in diff_text.lines() {
            if line.starts_with("@@") {
                // Save previous hunk
                if let Some(hunk) = current_hunk.take() {
                    self.hunks.push(hunk);
                }

                // Parse hunk header
                let (old_start, old_count, new_start, new_count) = Self::parse_hunk_header(line);
                old_line = old_start;
                new_line = new_start;

                current_hunk = Some(ManagedHunk::new(
                    hunk_id, line, old_start, old_count, new_start, new_count,
                ));
                hunk_id += 1;
            } else if let Some(ref mut hunk) = current_hunk {
                if line.starts_with('+') && !line.starts_with("+++") {
                    let content = if line.len() > 1 { &line[1..] } else { "" };
                    hunk.add_line(content, '+', None, Some(new_line));
                    new_line += 1;
                } else if line.starts_with('-') && !line.starts_with("---") {
                    let content = if line.len() > 1 { &line[1..] } else { "" };
                    hunk.add_line(content, '-', Some(old_line), None);
                    old_line += 1;
                } else if line.starts_with(' ') || line.is_empty() {
                    let content = if line.is_empty() { "" } else { &line[1..] };
                    hunk.add_line(content, ' ', Some(old_line), Some(new_line));
                    old_line += 1;
                    new_line += 1;
                }
            }
        }

        // Don't forget the last hunk
        if let Some(hunk) = current_hunk {
            self.hunks.push(hunk);
        }
    }

    /// Parse hunk header to extract line numbers
    fn parse_hunk_header(header: &str) -> (usize, usize, usize, usize) {
        let parts: Vec<&str> = header.split_whitespace().collect();

        let mut old_start = 1;
        let mut old_count = 0;
        let mut new_start = 1;
        let mut new_count = 0;

        if parts.len() >= 3 {
            if let Some(old_range) = parts.get(1) {
                let old_range = old_range.trim_start_matches('-');
                let old_parts: Vec<&str> = old_range.split(',').collect();
                old_start = old_parts.first().and_then(|s| s.parse().ok()).unwrap_or(1);
                old_count = old_parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            }
            if let Some(new_range) = parts.get(2) {
                let new_range = new_range.trim_start_matches('+');
                let new_parts: Vec<&str> = new_range.split(',').collect();
                new_start = new_parts.first().and_then(|s| s.parse().ok()).unwrap_or(1);
                new_count = new_parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            }
        }

        (old_start, old_count, new_start, new_count)
    }

    /// Apply action to a hunk
    pub fn apply_action(&mut self, hunk_id: usize, action: HunkAction) {
        if let Some(hunk) = self.hunks.iter_mut().find(|h| h.id == hunk_id) {
            match action {
                HunkAction::Apply => hunk.status = HunkStatus::Applied,
                HunkAction::Reject => hunk.status = HunkStatus::Rejected,
                HunkAction::Reset => hunk.status = HunkStatus::Pending,
                HunkAction::Edit => hunk.status = HunkStatus::Modified,
                HunkAction::Split | HunkAction::Combine => {
                    // Complex operations - not implemented yet
                }
            }
        }
    }

    /// Apply all pending hunks
    pub fn apply_all(&mut self) {
        for hunk in &mut self.hunks {
            if hunk.status == HunkStatus::Pending {
                hunk.status = HunkStatus::Applied;
            }
        }
    }

    /// Reject all pending hunks
    pub fn reject_all(&mut self) {
        for hunk in &mut self.hunks {
            if hunk.status == HunkStatus::Pending {
                hunk.status = HunkStatus::Rejected;
            }
        }
    }

    /// Reset all hunks to pending
    pub fn reset_all(&mut self) {
        for hunk in &mut self.hunks {
            hunk.status = HunkStatus::Pending;
        }
    }

    /// Get count of hunks by status
    pub fn count_by_status(&self, status: HunkStatus) -> usize {
        self.hunks.iter().filter(|h| h.status == status).count()
    }

    /// Get total additions
    pub fn total_additions(&self) -> usize {
        self.hunks.iter().map(|h| h.additions()).sum()
    }

    /// Get total deletions
    pub fn total_deletions(&self) -> usize {
        self.hunks.iter().map(|h| h.deletions()).sum()
    }

    /// Get applied additions
    pub fn applied_additions(&self) -> usize {
        self.hunks
            .iter()
            .filter(|h| h.status == HunkStatus::Applied)
            .map(|h| h.selected_additions())
            .sum()
    }

    /// Get applied deletions
    pub fn applied_deletions(&self) -> usize {
        self.hunks
            .iter()
            .filter(|h| h.status == HunkStatus::Applied)
            .map(|h| h.selected_deletions())
            .sum()
    }

    /// Navigate to next hunk
    pub fn next_hunk(&mut self) {
        if self.hunks.is_empty() {
            return;
        }

        self.focused_hunk = match self.focused_hunk {
            Some(idx) => Some((idx + 1) % self.hunks.len()),
            None => Some(0),
        };
    }

    /// Navigate to previous hunk
    pub fn prev_hunk(&mut self) {
        if self.hunks.is_empty() {
            return;
        }

        self.focused_hunk = match self.focused_hunk {
            Some(0) => Some(self.hunks.len() - 1),
            Some(idx) => Some(idx - 1),
            None => Some(self.hunks.len() - 1),
        };
    }

    /// Navigate to next pending hunk
    pub fn next_pending(&mut self) {
        let current = self.focused_hunk.unwrap_or(0);

        for i in 1..=self.hunks.len() {
            let idx = (current + i) % self.hunks.len();
            if self.hunks[idx].status == HunkStatus::Pending {
                self.focused_hunk = Some(idx);
                return;
            }
        }
    }

    /// Generate full patch for applied hunks
    pub fn generate_patch(&self) -> String {
        let mut patch = String::new();

        // Add file headers
        patch.push_str(&format!("--- a/{}\n", self.file_path.display()));
        patch.push_str(&format!("+++ b/{}\n", self.file_path.display()));

        for hunk in &self.hunks {
            if hunk.status == HunkStatus::Applied {
                patch.push_str(&hunk.generate_patch());
            }
        }

        patch
    }
}
