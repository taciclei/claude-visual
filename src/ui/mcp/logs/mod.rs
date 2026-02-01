//! MCP Server Logs Viewer
//!
//! UI component for viewing and filtering MCP server logs.

mod types;
mod core;
mod render;

// Re-export public types
pub use types::{LogLevel, LogEntry, LogFilter, McpLogsPanelEvent};
pub use core::McpLogsPanel;

use gpui::{App, EventEmitter, Focusable, FocusHandle};

// Implement EventEmitter
impl EventEmitter<McpLogsPanelEvent> for McpLogsPanel {}

// Implement Focusable
impl Focusable for McpLogsPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_from_str() {
        assert_eq!(LogLevel::from_str("debug"), LogLevel::Debug);
        assert_eq!(LogLevel::from_str("INFO"), LogLevel::Info);
        assert_eq!(LogLevel::from_str("Warning"), LogLevel::Warning);
        assert_eq!(LogLevel::from_str("error"), LogLevel::Error);
        assert_eq!(LogLevel::from_str("unknown"), LogLevel::Info);
    }

    #[test]
    fn test_log_filter_matches() {
        let entry = LogEntry::new("test-server", LogLevel::Warning, "Test message");

        // Default filter matches everything
        let filter = LogFilter::default();
        assert!(filter.matches(&entry));

        // Level filter
        let filter = LogFilter {
            min_level: LogLevel::Error,
            ..Default::default()
        };
        assert!(!filter.matches(&entry));

        let filter = LogFilter {
            min_level: LogLevel::Warning,
            ..Default::default()
        };
        assert!(filter.matches(&entry));

        // Server filter
        let filter = LogFilter {
            server: "test".to_string(),
            ..Default::default()
        };
        assert!(filter.matches(&entry));

        let filter = LogFilter {
            server: "other".to_string(),
            ..Default::default()
        };
        assert!(!filter.matches(&entry));

        // Search filter
        let filter = LogFilter {
            search: "message".to_string(),
            ..Default::default()
        };
        assert!(filter.matches(&entry));

        let filter = LogFilter {
            search: "notfound".to_string(),
            ..Default::default()
        };
        assert!(!filter.matches(&entry));
    }

    #[test]
    fn test_log_entry_elapsed_str() {
        let entry = LogEntry::new("server", LogLevel::Info, "message");
        // Should be "0s ago" or close to it
        let elapsed = entry.elapsed_str();
        assert!(elapsed.contains("s ago") || elapsed.contains("m ago"));
    }
}
