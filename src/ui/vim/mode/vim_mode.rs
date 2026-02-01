//! Vim mode enumeration

/// Vim editing modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VimMode {
    /// Normal mode - navigation and commands
    #[default]
    Normal,
    /// Insert mode - text entry
    Insert,
    /// Visual mode - character selection
    Visual,
    /// Visual line mode - line selection
    VisualLine,
    /// Visual block mode - block selection
    VisualBlock,
    /// Command mode - ex commands (:w, :q, etc.)
    Command,
    /// Search mode - pattern search (/)
    Search,
}

impl VimMode {
    /// Check if this mode allows text input
    pub fn allows_input(&self) -> bool {
        matches!(self, VimMode::Insert | VimMode::Command | VimMode::Search)
    }

    /// Check if this is a visual mode
    pub fn is_visual(&self) -> bool {
        matches!(self, VimMode::Visual | VimMode::VisualLine | VimMode::VisualBlock)
    }
}
