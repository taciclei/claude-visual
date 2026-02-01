//! Type definitions for side-by-side diff view

use super::super::hunk::HunkAction;

/// Events emitted by SideBySideDiffView
#[derive(Debug, Clone)]
pub enum SideBySideDiffEvent {
    /// Hunk action performed
    HunkActionPerformed { hunk_id: usize, action: HunkAction },
    /// Comment added
    CommentAdded { hunk_id: usize, line_index: usize, side: String },
    /// File clicked
    FileClicked(String),
    /// Apply changes requested
    ApplyRequested,
    /// Close requested
    CloseRequested,
}

/// Display mode for the diff
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffDisplayMode {
    /// Side by side (split view)
    SideBySide,
    /// Unified (inline)
    Unified,
}
