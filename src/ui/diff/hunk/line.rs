//! Diff line representation

/// A diff line for hunk operations
#[derive(Debug, Clone)]
pub struct HunkLine {
    /// Original content
    pub(crate) content: String,
    /// Line type: '+', '-', or ' ' (context)
    pub(crate) line_type: char,
    /// Line number in old file (for - and context)
    pub(crate) old_line: Option<usize>,
    /// Line number in new file (for + and context)
    pub(crate) new_line: Option<usize>,
    /// Is this line selected for partial apply
    pub(crate) selected: bool,
}

impl HunkLine {
    pub fn is_addition(&self) -> bool {
        self.line_type == '+'
    }

    pub fn is_deletion(&self) -> bool {
        self.line_type == '-'
    }

    pub fn is_context(&self) -> bool {
        self.line_type == ' '
    }
}
