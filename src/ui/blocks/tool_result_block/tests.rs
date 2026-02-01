//! Tests for tool result blocks

use super::types::{ToolExecutionStatus, ToolResult};
use std::time::Duration;

#[test]
fn test_tool_result_success() {
    let result = ToolResult::success(
        "read_file".to_string(),
        "filesystem".to_string(),
        Some(serde_json::json!({"path": "/tmp/test.txt"})),
        serde_json::json!({"content": "Hello, World!"}),
        Duration::from_millis(150),
    );

    assert_eq!(result.status, ToolExecutionStatus::Success);
    assert!(result.content.is_some());
    assert!(result.error.is_none());
}

#[test]
fn test_tool_result_error() {
    let result = ToolResult::error(
        "read_file".to_string(),
        "filesystem".to_string(),
        Some(serde_json::json!({"path": "/nonexistent"})),
        "File not found".to_string(),
        Duration::from_millis(50),
    );

    assert_eq!(result.status, ToolExecutionStatus::Error);
    assert!(result.content.is_none());
    assert!(result.error.is_some());
}

#[test]
fn test_tool_execution_status_display() {
    assert_eq!(ToolExecutionStatus::Success.as_str(), "Success");
    assert_eq!(ToolExecutionStatus::Error.as_str(), "Error");
    assert_eq!(ToolExecutionStatus::Pending.as_str(), "Pending");
    assert_eq!(ToolExecutionStatus::Cancelled.as_str(), "Cancelled");
}

#[test]
fn test_tool_execution_status_icon() {
    assert_eq!(ToolExecutionStatus::Success.icon(), "✓");
    assert_eq!(ToolExecutionStatus::Error.icon(), "✗");
    assert_eq!(ToolExecutionStatus::Pending.icon(), "⋯");
    assert_eq!(ToolExecutionStatus::Cancelled.icon(), "○");
}
