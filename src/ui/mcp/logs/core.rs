//! Core logic for MCP logs panel

use std::collections::VecDeque;
use std::sync::Arc;
use gpui::{Context, FocusHandle};

use crate::app::state::AppState;
use super::types::{LogEntry, LogFilter, LogLevel};

/// MCP Server Logs Panel
pub struct McpLogsPanel {
    pub(crate) app_state: Arc<AppState>,
    /// All log entries
    pub(crate) logs: VecDeque<LogEntry>,
    /// Maximum number of logs to keep
    pub(crate) max_logs: usize,
    /// Current filter
    pub(crate) filter: LogFilter,
    /// Whether auto-scroll is enabled
    pub(crate) auto_scroll: bool,
    /// Selected log index
    pub(crate) selected: Option<usize>,
    /// Whether the panel is expanded
    pub(crate) expanded: bool,
    /// Focus handle
    pub(crate) focus_handle: FocusHandle,
}

impl McpLogsPanel {
    /// Create a new logs panel
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            logs: VecDeque::new(),
            max_logs: 1000,
            filter: LogFilter::default(),
            auto_scroll: true,
            selected: None,
            expanded: true,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Add a log entry
    pub fn add_log(&mut self, entry: LogEntry, cx: &mut Context<Self>) {
        self.logs.push_back(entry);

        // Trim old logs
        while self.logs.len() > self.max_logs {
            self.logs.pop_front();
        }

        cx.notify();
    }

    /// Add a simple log message
    pub fn log(
        &mut self,
        server: impl Into<String>,
        level: LogLevel,
        message: impl Into<String>,
        cx: &mut Context<Self>,
    ) {
        self.add_log(LogEntry::new(server, level, message), cx);
    }

    /// Clear all logs
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.logs.clear();
        self.selected = None;
        cx.notify();
    }

    /// Clear logs for a specific server
    pub fn clear_server(&mut self, server: &str, cx: &mut Context<Self>) {
        self.logs.retain(|e| e.server != server);
        self.selected = None;
        cx.notify();
    }

    /// Set filter
    pub fn set_filter(&mut self, filter: LogFilter, cx: &mut Context<Self>) {
        self.filter = filter;
        cx.notify();
    }

    /// Set minimum log level
    pub fn set_min_level(&mut self, level: LogLevel, cx: &mut Context<Self>) {
        self.filter.min_level = level;
        cx.notify();
    }

    /// Set server filter
    pub fn set_server_filter(&mut self, server: String, cx: &mut Context<Self>) {
        self.filter.server = server;
        cx.notify();
    }

    /// Set search filter
    pub fn set_search(&mut self, search: String, cx: &mut Context<Self>) {
        self.filter.search = search;
        cx.notify();
    }

    /// Toggle auto-scroll
    pub fn toggle_auto_scroll(&mut self, cx: &mut Context<Self>) {
        self.auto_scroll = !self.auto_scroll;
        cx.notify();
    }

    /// Toggle expanded state
    pub fn toggle_expanded(&mut self, cx: &mut Context<Self>) {
        self.expanded = !self.expanded;
        cx.notify();
    }

    /// Get filtered logs
    pub fn filtered_logs(&self) -> Vec<(usize, &LogEntry)> {
        self.logs
            .iter()
            .enumerate()
            .filter(|(_, entry)| self.filter.matches(entry))
            .collect()
    }

    /// Get unique server names
    pub fn servers(&self) -> Vec<String> {
        let mut servers: Vec<String> = self
            .logs
            .iter()
            .map(|e| e.server.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        servers.sort();
        servers
    }

    /// Get log count by level
    pub fn count_by_level(&self) -> (usize, usize, usize, usize) {
        let mut debug = 0;
        let mut info = 0;
        let mut warning = 0;
        let mut error = 0;

        for entry in &self.logs {
            match entry.level {
                LogLevel::Debug => debug += 1,
                LogLevel::Info => info += 1,
                LogLevel::Warning => warning += 1,
                LogLevel::Error => error += 1,
            }
        }

        (debug, info, warning, error)
    }
}
