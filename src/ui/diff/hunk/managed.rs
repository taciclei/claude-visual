//! Managed hunk with status and selection

use super::line::HunkLine;
use super::types::HunkStatus;

/// A diff hunk with management capabilities
#[derive(Debug, Clone)]
pub struct ManagedHunk {
    /// Unique ID for this hunk
    pub(crate) id: usize,
    /// Hunk header (@@...)
    pub(crate) header: String,
    /// Lines in this hunk
    pub(crate) lines: Vec<HunkLine>,
    /// Current status
    pub(crate) status: HunkStatus,
    /// Is expanded in UI
    pub(crate) expanded: bool,
    /// Original start line in old file
    pub(crate) old_start: usize,
    /// Line count in old file
    pub(crate) old_count: usize,
    /// Original start line in new file
    pub(crate) new_start: usize,
    /// Line count in new file
    pub(crate) new_count: usize,
}

impl ManagedHunk {
    /// Create a new managed hunk from diff text
    pub fn new(
        id: usize,
        header: &str,
        old_start: usize,
        old_count: usize,
        new_start: usize,
        new_count: usize,
    ) -> Self {
        Self {
            id,
            header: header.to_string(),
            lines: Vec::new(),
            status: HunkStatus::Pending,
            expanded: true,
            old_start,
            old_count,
            new_start,
            new_count,
        }
    }

    /// Add a line to this hunk
    pub fn add_line(
        &mut self,
        content: &str,
        line_type: char,
        old_line: Option<usize>,
        new_line: Option<usize>,
    ) {
        self.lines.push(HunkLine {
            content: content.to_string(),
            line_type,
            old_line,
            new_line,
            selected: true,
        });
    }

    /// Get addition count
    pub fn additions(&self) -> usize {
        self.lines.iter().filter(|l| l.is_addition()).count()
    }

    /// Get deletion count
    pub fn deletions(&self) -> usize {
        self.lines.iter().filter(|l| l.is_deletion()).count()
    }

    /// Get selected addition count
    pub fn selected_additions(&self) -> usize {
        self.lines
            .iter()
            .filter(|l| l.is_addition() && l.selected)
            .count()
    }

    /// Get selected deletion count
    pub fn selected_deletions(&self) -> usize {
        self.lines
            .iter()
            .filter(|l| l.is_deletion() && l.selected)
            .count()
    }

    /// Toggle line selection
    pub fn toggle_line(&mut self, index: usize) {
        if let Some(line) = self.lines.get_mut(index) {
            if !line.is_context() {
                line.selected = !line.selected;
            }
        }
    }

    /// Select all lines
    pub fn select_all(&mut self) {
        for line in &mut self.lines {
            line.selected = true;
        }
    }

    /// Deselect all lines
    pub fn deselect_all(&mut self) {
        for line in &mut self.lines {
            if !line.is_context() {
                line.selected = false;
            }
        }
    }

    /// Generate patch text for selected lines
    pub fn generate_patch(&self) -> String {
        let mut patch = format!("{}\n", self.header);

        for line in &self.lines {
            if line.is_context() || line.selected {
                patch.push(line.line_type);
                patch.push_str(&line.content);
                patch.push('\n');
            }
        }

        patch
    }
}
