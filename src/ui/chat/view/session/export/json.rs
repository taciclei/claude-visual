//! JSON export format

use crate::claude::message::MessageRole;

use super::super::super::core::ChatView;

impl ChatView {
    /// Export conversation to JSON format
    pub(crate) fn export_to_json(&self) -> String {
        let stats = self.calculate_stats();
        let mut export = serde_json::json!({
            "title": self.display_title(),
            "exportedAt": chrono::Utc::now().to_rfc3339(),
            "stats": {
                "messageCount": stats.message_count,
                "userMessages": stats.user_message_count,
                "assistantMessages": stats.assistant_message_count,
                "toolCalls": stats.tool_use_count,
                "wordCount": stats.word_count,
                "estimatedTokens": stats.estimated_tokens,
                "durationMinutes": stats.duration_minutes,
            },
            "messages": []
        });

        if self.export.include_metadata {
            if let Some(ref conv_id) = self.current_conversation_id {
                export["conversationId"] = serde_json::json!(conv_id);
            }
            if let Some(ref info) = self.session_info {
                export["session"] = serde_json::json!({
                    "sessionId": info.session_id,
                    "model": info.model,
                    "version": info.version,
                    "cwd": info.cwd,
                });
            }
            if self.stats.cost > 0.0 {
                export["cost"] = serde_json::json!({
                    "total": self.stats.cost,
                    "inputTokens": self.stats.input_tokens,
                    "outputTokens": self.stats.output_tokens,
                });
            }
        }

        let messages: Vec<serde_json::Value> = self.messages.iter()
            .filter(|m| {
                // Filter based on export settings
                if !self.export.include_tools && matches!(m.role, MessageRole::ToolUse | MessageRole::ToolResult) {
                    return false;
                }
                if !self.export.include_thinking && matches!(m.role, MessageRole::Thinking) {
                    return false;
                }
                true
            })
            .map(|m| {
                let mut msg = serde_json::json!({
                    "role": format!("{:?}", m.role),
                    "content": m.content,
                    "timestamp": m.timestamp.to_rfc3339(),
                });
                if let Some(ref tool) = m.tool_name {
                    msg["toolName"] = serde_json::json!(tool);
                }
                if m.is_error {
                    msg["isError"] = serde_json::json!(true);
                }
                msg
            })
            .collect();
        export["messages"] = serde_json::json!(messages);

        serde_json::to_string_pretty(&export).unwrap_or_else(|_| "{}".to_string())
    }
}
