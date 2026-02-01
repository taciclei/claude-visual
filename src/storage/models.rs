//! Database models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A conversation (chat session)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    /// Unique identifier
    pub id: String,
    /// Associated project ID (optional)
    pub project_id: Option<String>,
    /// Conversation title
    pub title: String,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

impl Conversation {
    /// Create a new conversation
    pub fn new(title: impl Into<String>, project_id: Option<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            project_id,
            title: title.into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

/// A message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique identifier
    pub id: String,
    /// Parent conversation ID
    pub conversation_id: String,
    /// Role (user, assistant, tool_use, tool_result, error)
    pub role: String,
    /// Message content
    pub content: String,
    /// Tool name (for tool use/result)
    pub tool_name: Option<String>,
    /// Whether this is an error
    pub is_error: bool,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl Message {
    /// Create a new message
    pub fn new(
        conversation_id: impl Into<String>,
        role: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            conversation_id: conversation_id.into(),
            role: role.into(),
            content: content.into(),
            tool_name: None,
            is_error: false,
            timestamp: Utc::now(),
        }
    }

    /// Create a user message
    pub fn user(conversation_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self::new(conversation_id, "user", content)
    }

    /// Create an assistant message
    pub fn assistant(conversation_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self::new(conversation_id, "assistant", content)
    }

    /// Create a tool use message
    pub fn tool_use(
        conversation_id: impl Into<String>,
        tool_name: impl Into<String>,
        input: impl Into<String>,
    ) -> Self {
        let mut msg = Self::new(conversation_id, "tool_use", input);
        msg.tool_name = Some(tool_name.into());
        msg
    }

    /// Create a tool result message
    pub fn tool_result(
        conversation_id: impl Into<String>,
        output: impl Into<String>,
        is_error: bool,
    ) -> Self {
        let mut msg = Self::new(conversation_id, "tool_result", output);
        msg.is_error = is_error;
        msg
    }

    /// Create an error message
    pub fn error(conversation_id: impl Into<String>, message: impl Into<String>) -> Self {
        let mut msg = Self::new(conversation_id, "error", message);
        msg.is_error = true;
        msg
    }
}

/// Date range filter for search
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum DateRangeFilter {
    /// All time (no filter)
    #[default]
    AllTime,
    /// Today only
    Today,
    /// Last 7 days
    LastWeek,
    /// Last 30 days
    LastMonth,
    /// Last 3 months
    LastQuarter,
    /// Last year
    LastYear,
}

impl DateRangeFilter {
    /// Get display name for the filter
    pub fn display_name(&self) -> &'static str {
        match self {
            DateRangeFilter::AllTime => "All Time",
            DateRangeFilter::Today => "Today",
            DateRangeFilter::LastWeek => "Last 7 Days",
            DateRangeFilter::LastMonth => "Last 30 Days",
            DateRangeFilter::LastQuarter => "Last 3 Months",
            DateRangeFilter::LastYear => "Last Year",
        }
    }

    /// Get the start date for this filter (returns None for AllTime)
    pub fn start_date(&self) -> Option<DateTime<Utc>> {
        let now = Utc::now();
        match self {
            DateRangeFilter::AllTime => None,
            DateRangeFilter::Today => Some(now - chrono::Duration::days(1)),
            DateRangeFilter::LastWeek => Some(now - chrono::Duration::days(7)),
            DateRangeFilter::LastMonth => Some(now - chrono::Duration::days(30)),
            DateRangeFilter::LastQuarter => Some(now - chrono::Duration::days(90)),
            DateRangeFilter::LastYear => Some(now - chrono::Duration::days(365)),
        }
    }

    /// Get all filter options
    pub fn all() -> Vec<DateRangeFilter> {
        vec![
            DateRangeFilter::AllTime,
            DateRangeFilter::Today,
            DateRangeFilter::LastWeek,
            DateRangeFilter::LastMonth,
            DateRangeFilter::LastQuarter,
            DateRangeFilter::LastYear,
        ]
    }
}

/// Search filter options
#[derive(Debug, Clone, Default)]
pub struct SearchFilter {
    /// Date range filter
    pub date_range: DateRangeFilter,
    /// Filter by project ID (None = all projects)
    pub project_id: Option<String>,
}

impl SearchFilter {
    /// Check if any filters are active
    pub fn is_active(&self) -> bool {
        self.date_range != DateRangeFilter::AllTime || self.project_id.is_some()
    }

    /// Clear all filters
    pub fn clear(&mut self) {
        self.date_range = DateRangeFilter::AllTime;
        self.project_id = None;
    }
}

/// Search result from full-text search
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// Message that matched
    pub message: Message,
    /// Conversation title
    pub conversation_title: String,
    /// Highlighted content with matches
    pub highlighted: String,
    /// Search rank (lower is better)
    pub rank: f64,
}

/// Saved session for export/import
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedSession {
    /// Session version for compatibility
    pub version: u32,
    /// Conversation metadata
    pub conversation: Conversation,
    /// All messages
    pub messages: Vec<Message>,
    /// Export timestamp
    pub exported_at: DateTime<Utc>,
}

impl SavedSession {
    /// Current version
    pub const VERSION: u32 = 1;

    /// Create a new saved session
    pub fn new(conversation: Conversation, messages: Vec<Message>) -> Self {
        Self {
            version: Self::VERSION,
            conversation,
            messages,
            exported_at: Utc::now(),
        }
    }

    /// Export to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Import from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
