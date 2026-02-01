//! Core implementation for tool result blocks

use std::sync::Arc;
use std::time::Duration;

use gpui::*;
use serde_json::Value;

use super::types::{ToolExecutionStatus, ToolResult, ToolResultBlock, ToolResultBlockEvent};
use crate::app::state::AppState;

impl ToolResult {
    /// Create a successful result
    pub fn success(
        tool_name: String,
        server_name: String,
        arguments: Option<Value>,
        content: Value,
        duration: Duration,
    ) -> Self {
        Self {
            tool_name,
            server_name,
            arguments,
            content: Some(content),
            error: None,
            status: ToolExecutionStatus::Success,
            duration: Some(duration),
        }
    }

    /// Create an error result
    pub fn error(
        tool_name: String,
        server_name: String,
        arguments: Option<Value>,
        error: String,
        duration: Duration,
    ) -> Self {
        Self {
            tool_name,
            server_name,
            arguments,
            content: None,
            error: Some(error),
            status: ToolExecutionStatus::Error,
            duration: Some(duration),
        }
    }

    /// Create a pending result
    pub fn pending(tool_name: String, server_name: String, arguments: Option<Value>) -> Self {
        Self {
            tool_name,
            server_name,
            arguments,
            content: None,
            error: None,
            status: ToolExecutionStatus::Pending,
            duration: None,
        }
    }
}

impl EventEmitter<ToolResultBlockEvent> for ToolResultBlock {}

impl ToolResultBlock {
    /// Create a new tool result block
    pub fn new(result: ToolResult, app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            result,
            collapsed: false,
            args_expanded: false,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Toggle collapsed state
    pub fn toggle_collapsed(&mut self, cx: &mut Context<Self>) {
        self.collapsed = !self.collapsed;
        cx.notify();
    }

    /// Toggle arguments expanded state
    pub fn toggle_args_expanded(&mut self, cx: &mut Context<Self>) {
        self.args_expanded = !self.args_expanded;
        cx.notify();
    }

    /// Update the result
    pub fn set_result(&mut self, result: ToolResult, cx: &mut Context<Self>) {
        self.result = result;
        cx.notify();
    }

    /// Format JSON value for display
    pub(crate) fn format_json(&self, value: &Value) -> String {
        serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
    }

    /// Format duration for display
    pub(crate) fn format_duration(&self, duration: Duration) -> String {
        let millis = duration.as_millis();
        if millis < 1000 {
            format!("{}ms", millis)
        } else {
            format!("{:.2}s", duration.as_secs_f64())
        }
    }
}

impl Focusable for ToolResultBlock {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
