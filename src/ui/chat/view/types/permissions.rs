//! Permission request types

use crate::app::theme::Theme;

/// Permission request from Claude
#[derive(Debug, Clone)]
pub struct PermissionRequest {
    /// Unique ID for responding to this request
    pub request_id: String,
    /// Tool name requesting permission
    pub tool: String,
    /// Description of what's being requested
    pub description: String,
    /// The specific action/command
    pub action: String,
    /// Full command being executed (if available)
    pub command: Option<String>,
    /// Risk level
    pub risk_level: PermissionRisk,
    /// Request timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl PermissionRequest {
    /// Create a new permission request from CLI event
    pub fn from_event(
        request_id: impl Into<String>,
        tool: impl Into<String>,
        action: impl Into<String>,
        command: Option<String>,
    ) -> Self {
        let tool = tool.into();
        let action_str = action.into();
        let risk_level = PermissionRisk::from_tool_action(&tool, &action_str);

        Self {
            request_id: request_id.into(),
            tool,
            description: action_str.clone(),
            action: action_str,
            command,
            risk_level,
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Risk level for permission requests
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionRisk {
    Low,
    Medium,
    High,
}

impl PermissionRisk {
    pub fn color(&self, theme: &Theme) -> gpui::Hsla {
        match self {
            PermissionRisk::Low => theme.colors.success,
            PermissionRisk::Medium => theme.colors.warning,
            PermissionRisk::High => theme.colors.error,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            PermissionRisk::Low => "🟢",
            PermissionRisk::Medium => "🟡",
            PermissionRisk::High => "🔴",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            PermissionRisk::Low => "Low risk",
            PermissionRisk::Medium => "Medium risk",
            PermissionRisk::High => "High risk",
        }
    }

    /// Determine risk level from tool name and action
    pub fn from_tool_action(tool: &str, action: &str) -> Self {
        let tool_lower = tool.to_lowercase();
        let action_lower = action.to_lowercase();

        // High risk: destructive file operations, shell commands with dangerous patterns
        if tool_lower.contains("bash") || tool_lower.contains("shell") || tool_lower.contains("exec") {
            // Check for dangerous commands
            if action_lower.contains("rm -rf")
                || action_lower.contains("sudo")
                || action_lower.contains("> /dev")
                || action_lower.contains("dd if=")
                || action_lower.contains("mkfs")
                || action_lower.contains(":(){")
                || action_lower.contains("chmod 777")
            {
                return PermissionRisk::High;
            }
            // Shell commands are medium risk by default
            return PermissionRisk::Medium;
        }

        // High risk: write operations to important files
        if (tool_lower.contains("write") || tool_lower.contains("edit"))
            && (action_lower.contains("/etc/")
                || action_lower.contains("passwd")
                || action_lower.contains(".ssh")
                || action_lower.contains("id_rsa")
                || action_lower.contains(".env"))
        {
            return PermissionRisk::High;
        }

        // Medium risk: general write/edit operations
        if tool_lower.contains("write")
            || tool_lower.contains("edit")
            || tool_lower.contains("delete")
            || tool_lower.contains("notebook")
        {
            return PermissionRisk::Medium;
        }

        // Medium risk: git operations that modify state
        if tool_lower.contains("git")
            && (action_lower.contains("push")
                || action_lower.contains("force")
                || action_lower.contains("reset")
                || action_lower.contains("rebase"))
        {
            return PermissionRisk::Medium;
        }

        // Low risk: read operations, search, glob
        PermissionRisk::Low
    }
}
