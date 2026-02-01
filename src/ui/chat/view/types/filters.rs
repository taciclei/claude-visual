//! Message filtering types

use crate::claude::message::MessageRole;

/// Message filter for conversation view
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MessageFilter {
    /// Show all messages
    #[default]
    All,
    /// Show only user messages
    UserOnly,
    /// Show only assistant messages
    AssistantOnly,
    /// Show only tool messages (tool use and results)
    ToolsOnly,
}

impl MessageFilter {
    /// Get the display label for this filter
    pub fn label(&self) -> &'static str {
        match self {
            MessageFilter::All => "All",
            MessageFilter::UserOnly => "You",
            MessageFilter::AssistantOnly => "Claude",
            MessageFilter::ToolsOnly => "Tools",
        }
    }

    /// Get the icon for this filter
    pub fn icon(&self) -> &'static str {
        match self {
            MessageFilter::All => "💬",
            MessageFilter::UserOnly => "👤",
            MessageFilter::AssistantOnly => "🤖",
            MessageFilter::ToolsOnly => "🔧",
        }
    }

    /// Check if a message role should be shown with this filter
    pub fn includes_role(&self, role: MessageRole) -> bool {
        match self {
            MessageFilter::All => true,
            MessageFilter::UserOnly => matches!(role, MessageRole::User),
            MessageFilter::AssistantOnly => matches!(role, MessageRole::Assistant),
            MessageFilter::ToolsOnly => matches!(role, MessageRole::ToolUse | MessageRole::ToolResult),
        }
    }

    /// Get all filter options
    pub fn all_options() -> &'static [MessageFilter] {
        &[
            MessageFilter::All,
            MessageFilter::UserOnly,
            MessageFilter::AssistantOnly,
            MessageFilter::ToolsOnly,
        ]
    }
}

/// A search result in the conversation
#[derive(Debug, Clone)]
pub struct ConversationSearchResult {
    /// Message index in the messages list
    pub message_index: usize,
    /// Line number in the message content (0-based)
    pub line_number: usize,
    /// Start position in the line
    pub start: usize,
    /// End position in the line
    pub end: usize,
    /// Snippet of the matching line
    pub snippet: String,
    /// Role of the message
    pub role: MessageRole,
}
