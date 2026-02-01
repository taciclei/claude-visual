//! Message and tool conversion utilities

use crate::ai::claude::types::{ClaudeMessage, ClaudeTool};
use crate::ai::provider::{Message, MessageRole, ToolDefinition};

/// Convert internal message to Claude format
pub(crate) fn convert_message(message: &Message) -> ClaudeMessage {
    ClaudeMessage {
        role: match message.role {
            MessageRole::User => "user".to_string(),
            MessageRole::Assistant => "assistant".to_string(),
            _ => "user".to_string(), // System messages handled separately
        },
        content: message.content.clone(),
    }
}

/// Convert tool definition to Claude format
pub(crate) fn convert_tool(tool: &ToolDefinition) -> ClaudeTool {
    ClaudeTool {
        name: tool.name.clone(),
        description: tool.description.clone(),
        input_schema: tool.parameters.clone(),
    }
}
