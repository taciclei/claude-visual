//! Session capabilities formatting

use crate::ui::chat::view::ChatView;

impl ChatView {
    /// Format session capabilities for notification
    pub(crate) fn format_session_capabilities(
        &self,
        info: &crate::claude::message::SessionInfo,
    ) -> String {
        let mut parts = Vec::new();

        // Model name (simplified)
        let model_name = if info.model.contains("opus") {
            "Opus"
        } else if info.model.contains("sonnet") {
            "Sonnet"
        } else if info.model.contains("haiku") {
            "Haiku"
        } else {
            &info.model
        };
        parts.push(format!("Connected to {}", model_name));

        // Count capabilities
        let mut capabilities = Vec::new();

        if !info.tools.is_empty() {
            capabilities.push(format!("{} tools", info.tools.len()));
        }

        if !info.slash_commands.is_empty() {
            capabilities.push(format!("{} commands", info.slash_commands.len()));
        }

        if !info.mcp_servers.is_empty() {
            capabilities.push(format!("{} MCP servers", info.mcp_servers.len()));
        }

        if !capabilities.is_empty() {
            parts.push(capabilities.join(", "));
        }

        // Highlight key features available
        let mut features = Vec::new();
        if info.slash_commands.iter().any(|c| c.contains("think")) {
            features.push("/think");
        }
        if info.slash_commands.iter().any(|c| c.contains("review")) {
            features.push("/review");
        }
        if info.slash_commands.iter().any(|c| c.contains("commit")) {
            features.push("/commit");
        }

        if !features.is_empty() && features.len() <= 3 {
            parts.push(format!("Try: {}", features.join(" ")));
        }

        parts.join(" · ")
    }
}
