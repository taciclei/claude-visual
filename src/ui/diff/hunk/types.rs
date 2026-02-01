//! Hunk types and enums

/// Status of a hunk in the review process
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HunkStatus {
    /// Not yet reviewed
    Pending,
    /// Hunk accepted/applied
    Applied,
    /// Hunk rejected
    Rejected,
    /// Hunk modified by user
    Modified,
    /// Conflict detected
    Conflicted,
}

impl HunkStatus {
    /// Get color for status (as RGB)
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            HunkStatus::Pending => (128, 128, 128),    // Gray
            HunkStatus::Applied => (98, 181, 67),      // Green
            HunkStatus::Rejected => (224, 108, 117),   // Red
            HunkStatus::Modified => (209, 154, 102),   // Yellow
            HunkStatus::Conflicted => (198, 120, 221), // Purple
        }
    }

    /// Get icon for status
    pub fn icon(&self) -> &'static str {
        match self {
            HunkStatus::Pending => "○",
            HunkStatus::Applied => "✓",
            HunkStatus::Rejected => "✗",
            HunkStatus::Modified => "~",
            HunkStatus::Conflicted => "!",
        }
    }
}

/// Action that can be performed on a hunk
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HunkAction {
    /// Apply this hunk
    Apply,
    /// Reject this hunk
    Reject,
    /// Revert to pending state
    Reset,
    /// Edit hunk content
    Edit,
    /// Split into smaller hunks
    Split,
    /// Combine with adjacent hunk
    Combine,
}
