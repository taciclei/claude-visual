//! Stream JSON parsing for Claude CLI output

use crate::claude::message::{ClaudeEvent, SessionInfo};

/// Parse a stream-json line from Claude CLI
pub(crate) fn parse_stream_json(json: &serde_json::Value) -> Option<ClaudeEvent> {
    let event_type = json.get("type")?.as_str()?;

    match event_type {
        "system" => {
            // Check for init subtype
            let subtype = json.get("subtype").and_then(|s| s.as_str());
            if subtype == Some("init") {
                // Parse session info
                let info = SessionInfo {
                    session_id: json.get("session_id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    model: json.get("model").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    tools: json.get("tools")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                        .unwrap_or_default(),
                    slash_commands: json.get("slash_commands")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                        .unwrap_or_default(),
                    agents: json.get("agents")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                        .unwrap_or_default(),
                    skills: json.get("skills")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                        .unwrap_or_default(),
                    cwd: json.get("cwd").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    version: json.get("claude_code_version").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    mcp_servers: Vec::new(), // MCP servers parsed separately if available
                };
                tracing::info!("Claude session initialized: model={}, {} tools, {} commands",
                    info.model, info.tools.len(), info.slash_commands.len());
                Some(ClaudeEvent::SystemInit { info })
            } else {
                tracing::debug!("Claude system event: {:?}", subtype);
                None
            }
        }
        "assistant" => {
            // Check for content in the message
            if let Some(message) = json.get("message") {
                if let Some(content) = message.get("content") {
                    if let Some(arr) = content.as_array() {
                        // Collect all content blocks
                        let mut text_parts = Vec::new();
                        let mut tool_uses = Vec::new();
                        let mut thinking_parts = Vec::new();

                        for item in arr {
                            let item_type = item.get("type").and_then(|t| t.as_str());
                            match item_type {
                                Some("text") => {
                                    if let Some(text) = item.get("text").and_then(|t| t.as_str()) {
                                        text_parts.push(text.to_string());
                                    }
                                }
                                Some("tool_use") => {
                                    let name = item.get("name").and_then(|n| n.as_str()).unwrap_or("unknown").to_string();
                                    let input = item.get("input").cloned().unwrap_or(serde_json::json!({}));
                                    tool_uses.push((name, input));
                                }
                                Some("thinking") => {
                                    if let Some(text) = item.get("thinking").and_then(|t| t.as_str()) {
                                        thinking_parts.push(text.to_string());
                                    }
                                }
                                _ => {
                                    tracing::debug!("Unknown content type in assistant message: {:?}", item_type);
                                }
                            }
                        }

                        // Return text content if any
                        if !text_parts.is_empty() {
                            return Some(ClaudeEvent::ContentBlockDelta {
                                delta: text_parts.join("\n"),
                            });
                        }
                        // Return first tool use if any
                        if let Some((name, input)) = tool_uses.into_iter().next() {
                            return Some(ClaudeEvent::ToolUse { name, input });
                        }
                        // Return thinking if any
                        if !thinking_parts.is_empty() {
                            return Some(ClaudeEvent::Thinking {
                                content: thinking_parts.join("\n"),
                            });
                        }
                    }
                }
            }
            None
        }
        "content_block_delta" => {
            let delta = json
                .get("delta")?
                .get("text")?
                .as_str()?
                .to_string();
            Some(ClaudeEvent::ContentBlockDelta { delta })
        }
        "content_block_start" => {
            // Check if this is a tool use
            if let Some(content_block) = json.get("content_block") {
                if content_block.get("type")?.as_str()? == "tool_use" {
                    let name = content_block.get("name")?.as_str()?.to_string();
                    return Some(ClaudeEvent::ToolUse {
                        name,
                        input: serde_json::json!({}),
                    });
                }
            }
            None
        }
        "tool_use" => {
            // Tool use from Claude
            let tool_name = json
                .get("tool")
                .or_else(|| json.get("name"))
                .and_then(|t| t.as_str())
                .unwrap_or("unknown")
                .to_string();

            let input = json
                .get("input")
                .cloned()
                .unwrap_or(serde_json::json!({}));

            Some(ClaudeEvent::ToolUse {
                name: tool_name,
                input,
            })
        }
        "tool_result" => {
            let output = json
                .get("content")
                .or_else(|| json.get("output"))
                .and_then(|c| c.as_str())
                .unwrap_or("")
                .to_string();
            let is_error = json.get("is_error").and_then(|e| e.as_bool()).unwrap_or(false);

            Some(ClaudeEvent::ToolResult { output, is_error })
        }
        "result" => {
            // Final result - check if success or error
            let subtype = json.get("subtype").and_then(|s| s.as_str());
            let is_error = json.get("is_error").and_then(|e| e.as_bool()).unwrap_or(false);

            // Extract usage info
            if let Some(usage) = json.get("usage") {
                let input_tokens = usage.get("input_tokens").and_then(|v| v.as_u64()).unwrap_or(0)
                    + usage.get("cache_read_input_tokens").and_then(|v| v.as_u64()).unwrap_or(0);
                let output_tokens = usage.get("output_tokens").and_then(|v| v.as_u64()).unwrap_or(0);
                let cost_usd = json.get("total_cost_usd").and_then(|v| v.as_f64());

                if input_tokens > 0 || output_tokens > 0 {
                    tracing::info!("Session usage: {} input, {} output tokens, cost: ${:.4}",
                        input_tokens, output_tokens, cost_usd.unwrap_or(0.0));
                }
            }

            if is_error || subtype == Some("error") {
                let message = json.get("result")
                    .and_then(|r| r.as_str())
                    .unwrap_or("Unknown error")
                    .to_string();
                Some(ClaudeEvent::Error { message })
            } else {
                // Success - signal end
                Some(ClaudeEvent::AssistantEnd)
            }
        }
        "error" => {
            let message = json
                .get("error")
                .and_then(|e| e.get("message"))
                .or_else(|| json.get("message"))
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown error")
                .to_string();
            Some(ClaudeEvent::Error { message })
        }
        "message_stop" | "message_end" => {
            Some(ClaudeEvent::AssistantEnd)
        }
        "usage" => {
            // Token usage information
            let input_tokens = json.get("input_tokens")
                .or_else(|| json.get("usage").and_then(|u| u.get("input_tokens")))
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            let output_tokens = json.get("output_tokens")
                .or_else(|| json.get("usage").and_then(|u| u.get("output_tokens")))
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            let cost_usd = json.get("cost_usd")
                .or_else(|| json.get("cost"))
                .and_then(|v| v.as_f64());

            if input_tokens > 0 || output_tokens > 0 {
                Some(ClaudeEvent::Usage {
                    input_tokens,
                    output_tokens,
                    cost_usd,
                })
            } else {
                None
            }
        }
        _ => {
            // Log all unhandled events for debugging
            tracing::debug!("Unhandled event type '{}': {}", event_type, serde_json::to_string_pretty(json).unwrap_or_default());
            None
        }
    }
}
