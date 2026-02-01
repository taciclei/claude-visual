//! HTML export format

use crate::claude::message::MessageRole;

use super::super::super::core::ChatView;

impl ChatView {
    /// Export conversation to HTML format
    pub(crate) fn export_to_html(&self) -> String {
        let title = self.display_title();
        let stats = self.calculate_stats();

        let mut html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        :root {{
            --bg: #1e1e2e;
            --surface: #313244;
            --text: #cdd6f4;
            --text-muted: #a6adc8;
            --accent: #89b4fa;
            --user-bg: #45475a;
            --assistant-bg: #313244;
            --tool-bg: #11111b;
            --error: #f38ba8;
            --success: #a6e3a1;
            --border: #45475a;
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
            background: var(--bg);
            color: var(--text);
            line-height: 1.6;
            padding: 2rem;
            max-width: 900px;
            margin: 0 auto;
        }}
        h1 {{ color: var(--accent); margin-bottom: 1rem; }}
        .meta {{
            background: var(--surface);
            padding: 1rem;
            border-radius: 8px;
            margin-bottom: 2rem;
            font-size: 0.9rem;
        }}
        .meta-row {{ display: flex; justify-content: space-between; padding: 0.25rem 0; }}
        .meta-label {{ color: var(--text-muted); }}
        .messages {{ display: flex; flex-direction: column; gap: 1rem; }}
        .message {{
            padding: 1rem;
            border-radius: 8px;
            border-left: 3px solid;
        }}
        .message.user {{ background: var(--user-bg); border-color: var(--accent); }}
        .message.assistant {{ background: var(--assistant-bg); border-color: var(--success); }}
        .message.tool {{ background: var(--tool-bg); border-color: var(--text-muted); font-size: 0.85rem; }}
        .message.error {{ background: var(--tool-bg); border-color: var(--error); }}
        .message.thinking {{ background: var(--tool-bg); border-color: #fab387; opacity: 0.8; }}
        .role {{
            font-weight: 600;
            margin-bottom: 0.5rem;
            text-transform: uppercase;
            font-size: 0.75rem;
            letter-spacing: 0.05em;
        }}
        .timestamp {{ color: var(--text-muted); font-size: 0.75rem; }}
        pre {{
            background: #11111b;
            padding: 1rem;
            border-radius: 4px;
            overflow-x: auto;
            margin: 0.5rem 0;
        }}
        code {{ font-family: 'Fira Code', 'Consolas', monospace; font-size: 0.9rem; }}
    </style>
</head>
<body>
    <h1>{}</h1>
"#, title, title);

        if self.export.include_metadata {
            html.push_str("    <div class=\"meta\">\n");
            html.push_str(&format!("        <div class=\"meta-row\"><span class=\"meta-label\">Messages</span><span>{}</span></div>\n", stats.message_count));
            html.push_str(&format!("        <div class=\"meta-row\"><span class=\"meta-label\">Duration</span><span>{}</span></div>\n", stats.format_duration()));
            html.push_str(&format!("        <div class=\"meta-row\"><span class=\"meta-label\">Words</span><span>{}</span></div>\n", stats.format_words()));
            if let Some(ref info) = self.session_info {
                if !info.model.is_empty() {
                    html.push_str(&format!("        <div class=\"meta-row\"><span class=\"meta-label\">Model</span><span>{}</span></div>\n", info.model));
                }
            }
            if self.stats.cost > 0.0 {
                html.push_str(&format!("        <div class=\"meta-row\"><span class=\"meta-label\">Cost</span><span>${:.4}</span></div>\n", self.stats.cost));
            }
            html.push_str("    </div>\n");
        }

        html.push_str("    <div class=\"messages\">\n");

        for message in &self.messages {
            // Filter based on export settings
            if !self.export.include_tools && matches!(message.role, MessageRole::ToolUse | MessageRole::ToolResult) {
                continue;
            }
            if !self.export.include_thinking && matches!(message.role, MessageRole::Thinking) {
                continue;
            }

            let class = match message.role {
                MessageRole::User => "user",
                MessageRole::Assistant => "assistant",
                MessageRole::ToolUse | MessageRole::ToolResult => "tool",
                MessageRole::Error => "error",
                MessageRole::Thinking => "thinking",
                MessageRole::System => "assistant",
            };

            let role_name = match message.role {
                MessageRole::User => "You",
                MessageRole::Assistant => "Claude",
                MessageRole::ToolUse => &format!("Tool: {}", message.tool_name.as_deref().unwrap_or("unknown")),
                MessageRole::ToolResult => "Tool Result",
                MessageRole::Error => "Error",
                MessageRole::Thinking => "Thinking",
                MessageRole::System => "System",
            };

            html.push_str(&format!("        <div class=\"message {}\">\n", class));
            html.push_str(&format!("            <div class=\"role\">{} <span class=\"timestamp\">{}</span></div>\n",
                role_name, message.timestamp.format("%H:%M:%S")));

            // Escape HTML and convert code blocks
            let content = html_escape(&message.content);
            let content = convert_code_blocks_to_html(&content);
            html.push_str(&format!("            <div class=\"content\">{}</div>\n", content));

            html.push_str("        </div>\n");
        }

        html.push_str("    </div>\n</body>\n</html>");
        html
    }
}

// ==================== Helper Functions ====================

/// Escape HTML special characters
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Convert markdown code blocks to HTML
fn convert_code_blocks_to_html(text: &str) -> String {
    // Simple conversion - wrap content in <pre> if it contains code blocks
    if text.contains("```") {
        text.replace("```", "<pre>")
            .replace("```", "</pre>")
    } else {
        text.to_string()
    }
}
