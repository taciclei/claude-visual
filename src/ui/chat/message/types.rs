//! Message types and enums

use crate::claude::message::MessageRole;

/// Reaction type for message feedback
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageReaction {
    /// Positive feedback (thumbs up)
    ThumbsUp,
    /// Negative feedback (thumbs down)
    ThumbsDown,
}

impl MessageReaction {
    /// Get the emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            MessageReaction::ThumbsUp => "👍",
            MessageReaction::ThumbsDown => "👎",
        }
    }

    /// Get the label for accessibility
    pub fn label(&self) -> &'static str {
        match self {
            MessageReaction::ThumbsUp => "Good response",
            MessageReaction::ThumbsDown => "Poor response",
        }
    }
}

/// Events emitted by MessageView
pub enum MessageViewEvent {
    /// Request to delete this message
    Delete,
    /// Request to copy message content
    Copy,
    /// Request to copy as Markdown
    CopyAsMarkdown,
    /// Request to copy a specific code block
    CopyCode(String),
    /// Request to regenerate this response (assistant messages only)
    RegenerateResponse,
    /// Request to edit this message (user messages only)
    Edit(String),
    /// Quote message content in a new message
    Quote(String),
    /// Request to retry from this point
    RetryFromHere,
    /// User reacted to the message
    Reacted(MessageReaction),
    /// Bookmark toggled
    BookmarkToggled(bool),
    /// Request to rerun a command (tool use)
    RerunCommand(String),
    /// Request to open file in editor
    OpenFile(String),
    /// Execute a skill/command (for error recovery suggestions)
    ExecuteSkill(String),
}

/// Context menu action for messages
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageAction {
    Copy,
    CopyAsMarkdown,
    Regenerate,
    Edit,
    Quote,
    RetryFromHere,
    Bookmark,
    Delete,
}

impl MessageAction {
    /// Get display label for action
    pub fn label(&self) -> &'static str {
        match self {
            MessageAction::Copy => "Copy",
            MessageAction::CopyAsMarkdown => "Copy as Markdown",
            MessageAction::Regenerate => "Regenerate Response",
            MessageAction::Edit => "Edit Message",
            MessageAction::Quote => "Quote in Reply",
            MessageAction::RetryFromHere => "Retry from Here",
            MessageAction::Bookmark => "Toggle Bookmark",
            MessageAction::Delete => "Delete",
        }
    }

    /// Get icon for action
    pub fn icon(&self) -> &'static str {
        match self {
            MessageAction::Copy => "📋",
            MessageAction::CopyAsMarkdown => "📝",
            MessageAction::Regenerate => "🔄",
            MessageAction::Edit => "✏️",
            MessageAction::Quote => "💬",
            MessageAction::RetryFromHere => "↩️",
            MessageAction::Bookmark => "🔖",
            MessageAction::Delete => "🗑️",
        }
    }

    /// Get keyboard shortcut hint
    pub fn shortcut(&self) -> Option<&'static str> {
        match self {
            MessageAction::Copy => Some("⌘C"),
            MessageAction::Bookmark => Some("⌘B"),
            MessageAction::Delete => Some("⌫"),
            MessageAction::Regenerate => Some("⌘R"),
            MessageAction::Edit => Some("⌘E"),
            _ => None,
        }
    }

    /// Check if action is available for a role
    pub fn available_for_role(&self, role: MessageRole) -> bool {
        match self {
            MessageAction::Copy | MessageAction::CopyAsMarkdown | MessageAction::Quote | MessageAction::Bookmark | MessageAction::Delete => true,
            MessageAction::Regenerate | MessageAction::RetryFromHere => matches!(role, MessageRole::Assistant),
            MessageAction::Edit => matches!(role, MessageRole::User),
        }
    }

    /// Check if action is destructive
    pub fn is_destructive(&self) -> bool {
        matches!(self, MessageAction::Delete)
    }
}
