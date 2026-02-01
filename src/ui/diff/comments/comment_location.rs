//! Location key for comment threads

/// Location key for comment threads
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CommentLocation {
    pub hunk_id: usize,
    pub line_index: usize,
    pub side: String,
}

impl CommentLocation {
    pub fn new(hunk_id: usize, line_index: usize, side: &str) -> Self {
        Self {
            hunk_id,
            line_index,
            side: side.to_string(),
        }
    }
}
