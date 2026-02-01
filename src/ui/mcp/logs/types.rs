//! Types and data structures for MCP logs

use std::time::{Duration, Instant};
use gpui::Hsla;

/// Log level for MCP server messages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl LogLevel {
    /// Get display label
    pub fn label(&self) -> &'static str {
        match self {
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warning => "WARN",
            Self::Error => "ERROR",
        }
    }

    /// Get color for the log level
    pub fn color(&self) -> Hsla {
        use gpui::hsla;
        match self {
            Self::Debug => hsla(0.0, 0.0, 0.5, 1.0),      // Gray
            Self::Info => hsla(210.0 / 360.0, 0.6, 0.5, 1.0), // Blue
            Self::Warning => hsla(45.0 / 360.0, 0.8, 0.5, 1.0), // Yellow
            Self::Error => hsla(0.0, 0.7, 0.5, 1.0),      // Red
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "debug" | "trace" => Self::Debug,
            "info" => Self::Info,
            "warn" | "warning" => Self::Warning,
            "error" | "err" | "fatal" => Self::Error,
            _ => Self::Info,
        }
    }
}

/// A single log entry
#[derive(Debug, Clone)]
pub struct LogEntry {
    /// Timestamp when log was received
    pub(crate) timestamp: Instant,
    /// Server name
    pub(crate) server: String,
    /// Log level
    pub(crate) level: LogLevel,
    /// Log message
    pub(crate) message: String,
    /// Additional context (JSON-RPC method, etc.)
    pub(crate) context: Option<String>,
}

impl LogEntry {
    /// Create a new log entry
    pub fn new(server: impl Into<String>, level: LogLevel, message: impl Into<String>) -> Self {
        Self {
            timestamp: Instant::now(),
            server: server.into(),
            level,
            message: message.into(),
            context: None,
        }
    }

    /// Create with context
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Get elapsed time since log entry was created
    pub fn elapsed(&self) -> Duration {
        self.timestamp.elapsed()
    }

    /// Format elapsed time as string
    pub fn elapsed_str(&self) -> String {
        let elapsed = self.elapsed();
        if elapsed.as_secs() < 60 {
            format!("{}s ago", elapsed.as_secs())
        } else if elapsed.as_secs() < 3600 {
            format!("{}m ago", elapsed.as_secs() / 60)
        } else {
            format!("{}h ago", elapsed.as_secs() / 3600)
        }
    }
}

/// Filter settings for logs
#[derive(Debug, Clone)]
pub struct LogFilter {
    /// Minimum log level to show
    pub(crate) min_level: LogLevel,
    /// Server name filter (empty = all)
    pub(crate) server: String,
    /// Search text in message
    pub(crate) search: String,
}

impl Default for LogFilter {
    fn default() -> Self {
        Self {
            min_level: LogLevel::Debug,
            server: String::new(),
            search: String::new(),
        }
    }
}

impl LogFilter {
    /// Check if a log entry matches the filter
    pub fn matches(&self, entry: &LogEntry) -> bool {
        // Check level
        let level_ok = match self.min_level {
            LogLevel::Debug => true,
            LogLevel::Info => !matches!(entry.level, LogLevel::Debug),
            LogLevel::Warning => matches!(entry.level, LogLevel::Warning | LogLevel::Error),
            LogLevel::Error => matches!(entry.level, LogLevel::Error),
        };
        if !level_ok {
            return false;
        }

        // Check server
        if !self.server.is_empty()
            && !entry
                .server
                .to_lowercase()
                .contains(&self.server.to_lowercase())
        {
            return false;
        }

        // Check search
        if !self.search.is_empty() {
            let search_lower = self.search.to_lowercase();
            if !entry.message.to_lowercase().contains(&search_lower)
                && !entry
                    .context
                    .as_ref()
                    .map(|c| c.to_lowercase().contains(&search_lower))
                    .unwrap_or(false)
            {
                return false;
            }
        }

        true
    }
}

/// Events emitted by the logs panel
pub enum McpLogsPanelEvent {
    /// Request to clear all logs
    ClearLogs,
    /// Request to clear logs for a specific server
    ClearServerLogs(String),
    /// Request to export logs
    ExportLogs,
    /// Log entry clicked
    LogClicked(usize),
}
