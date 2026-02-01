//! Type definitions for history sidebar

/// Events emitted by HistorySidebar
pub enum HistorySidebarEvent {
    /// A conversation was selected
    ConversationSelected(String),
    /// User requested to delete a conversation
    DeleteConversation(String),
    /// Send a Claude Code skill command
    SendSkillCommand(String),
    /// Resume a specific session
    ResumeSession(String),
}

/// Display mode for the history sidebar
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum DisplayMode {
    /// Showing recent conversations
    Recent,
    /// Showing search results
    Search,
}
