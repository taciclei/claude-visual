//! Tool content parsing

use super::types::ToolDisplay;

/// Parse message content into a structured ToolDisplay
pub(super) fn parse_tool_content(content: &str, is_tool_result: bool) -> ToolDisplay {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(content) {
        // For tool use, show relevant fields nicely
        if let Some(file_path) = json.get("file_path").and_then(|v| v.as_str()) {
            // Check if it's an edit operation
            let old_string = json
                .get("old_string")
                .and_then(|v| v.as_str())
                .map(String::from);
            let new_string = json
                .get("new_string")
                .and_then(|v| v.as_str())
                .map(String::from);

            if old_string.is_some() || new_string.is_some() {
                ToolDisplay::Edit {
                    file_path: file_path.to_string(),
                    old_text: old_string,
                    new_text: new_string,
                }
            } else {
                ToolDisplay::FilePath {
                    path: file_path.to_string(),
                    display: format!("📄 {}", file_path),
                }
            }
        } else if let Some(command) = json.get("command").and_then(|v| v.as_str()) {
            let desc = json
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let display = format!("$ {}", command);
            ToolDisplay::Command {
                cmd: command.to_string(),
                desc,
                display,
            }
        } else if let Some(prompt) = json.get("prompt").and_then(|v| v.as_str()) {
            ToolDisplay::Prompt {
                display: format!(
                    "💭 {}",
                    if prompt.len() > 80 {
                        format!("{}...", &prompt[..80])
                    } else {
                        prompt.to_string()
                    }
                ),
            }
        } else if let Some(pattern) = json.get("pattern").and_then(|v| v.as_str()) {
            let path = json.get("path").and_then(|v| v.as_str()).map(String::from);
            ToolDisplay::Pattern {
                pattern: pattern.to_string(),
                path,
                display: format!("🔍 {}", pattern),
            }
        } else {
            // Fallback to pretty-printed JSON
            ToolDisplay::Json(serde_json::to_string_pretty(&json).unwrap_or(content.to_string()))
        }
    } else {
        // Not JSON, use as-is but truncate if too long
        let line_count = content.lines().count();
        if content.len() > 500 && !is_tool_result {
            ToolDisplay::Plain(format!("{}... ({} lines)", &content[..500], line_count))
        } else {
            ToolDisplay::Plain(content.to_string())
        }
    }
}
