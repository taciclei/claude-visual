//! Timeline types and enums

/// Timeline orientation
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TimelineOrientation {
    /// Vertical (default)
    #[default]
    Vertical,
    /// Horizontal
    Horizontal,
}

/// Timeline item status
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TimelineItemStatus {
    /// Pending/upcoming
    #[default]
    Pending,
    /// In progress
    InProgress,
    /// Completed
    Completed,
    /// Error/failed
    Error,
}

impl TimelineItemStatus {
    pub(crate) fn icon(&self) -> &str {
        match self {
            TimelineItemStatus::Pending => "○",
            TimelineItemStatus::InProgress => "◐",
            TimelineItemStatus::Completed => "●",
            TimelineItemStatus::Error => "✕",
        }
    }
}

/// Timeline item data
#[derive(Debug, Clone)]
pub struct TimelineItem {
    /// Item title
    pub title: String,
    /// Item description
    pub description: Option<String>,
    /// Timestamp or date
    pub timestamp: Option<String>,
    /// Status
    pub status: TimelineItemStatus,
    /// Custom icon
    pub icon: Option<String>,
}

impl TimelineItem {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            timestamp: None,
            status: TimelineItemStatus::default(),
            icon: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }

    pub fn status(mut self, status: TimelineItemStatus) -> Self {
        self.status = status;
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn completed(mut self) -> Self {
        self.status = TimelineItemStatus::Completed;
        self
    }

    pub fn in_progress(mut self) -> Self {
        self.status = TimelineItemStatus::InProgress;
        self
    }

    pub fn error(mut self) -> Self {
        self.status = TimelineItemStatus::Error;
        self
    }
}

/// Events emitted by Timeline
#[derive(Debug, Clone)]
pub enum TimelineEvent {
    /// Item clicked
    ItemClicked(usize),
}
