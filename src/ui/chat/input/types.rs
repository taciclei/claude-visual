//! Types for chat input

use crate::ai::mention::PartialMention;

/// Events emitted by ChatInput
pub enum ChatInputEvent {
    /// User submitted a message
    Submit(String),
    /// Files were attached via @file mentions
    FilesAttached(Vec<std::path::PathBuf>),
    /// User is typing a mention (for autocomplete)
    MentionPartial(PartialMention),
    /// Toggle think mode (extended reasoning)
    ToggleThinkMode,
    /// Clear the conversation (Ctrl+L)
    ClearConversation,
    /// Open history search (Ctrl+R)
    OpenHistorySearch,
}
