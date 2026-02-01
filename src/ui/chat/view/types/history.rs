//! Edit history and reactions

/// Entry in the edit history for undo/redo
#[derive(Debug, Clone)]
pub struct EditHistoryEntry {
    /// Action type
    pub action: EditAction,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Message index affected (if applicable)
    pub message_index: Option<usize>,
    /// Previous content (for undo)
    pub previous_content: Option<String>,
    /// New content (for redo)
    pub new_content: Option<String>,
}

/// Types of edit actions
#[derive(Debug, Clone, PartialEq)]
pub enum EditAction {
    /// Message was edited
    EditMessage,
    /// Message was deleted
    DeleteMessage,
    /// Message was added
    AddMessage,
    /// Bookmark toggled
    ToggleBookmark,
    /// Pin toggled
    TogglePin,
}

impl EditAction {
    /// Display name for the action
    pub fn display_name(&self) -> &'static str {
        match self {
            EditAction::EditMessage => "Edit Message",
            EditAction::DeleteMessage => "Delete Message",
            EditAction::AddMessage => "Add Message",
            EditAction::ToggleBookmark => "Toggle Bookmark",
            EditAction::TogglePin => "Toggle Pin",
        }
    }
}

/// A reaction to a message
#[derive(Debug, Clone)]
pub struct MessageReaction {
    /// The emoji reaction
    pub emoji: &'static str,
    /// When it was added
    pub added_at: chrono::DateTime<chrono::Utc>,
}

impl MessageReaction {
    pub fn new(emoji: &'static str) -> Self {
        Self {
            emoji,
            added_at: chrono::Utc::now(),
        }
    }
}

/// Available quick reactions
pub const QUICK_REACTIONS: &[&str] = &["👍", "👎", "❤️", "🎉", "🤔", "👀", "🔥", "💡"];
