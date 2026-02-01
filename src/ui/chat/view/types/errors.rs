//! Error handling types

/// Error category for smart suggestions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    /// Network/connection issues
    Network,
    /// Rate limiting or quota exceeded
    RateLimit,
    /// Context too large
    ContextOverflow,
    /// Authentication/permission issues
    Auth,
    /// Tool execution failed
    ToolError,
    /// General/unknown error
    General,
}

impl ErrorCategory {
    /// Detect error category from message
    pub fn from_message(msg: &str) -> Self {
        let msg_lower = msg.to_lowercase();

        if msg_lower.contains("network") || msg_lower.contains("connection")
            || msg_lower.contains("timeout") || msg_lower.contains("refused")
            || msg_lower.contains("dns") || msg_lower.contains("offline") {
            ErrorCategory::Network
        } else if msg_lower.contains("rate limit") || msg_lower.contains("quota")
            || msg_lower.contains("too many requests") || msg_lower.contains("429") {
            ErrorCategory::RateLimit
        } else if msg_lower.contains("context") || msg_lower.contains("token limit")
            || msg_lower.contains("too long") || msg_lower.contains("exceeds")
            || msg_lower.contains("maximum") {
            ErrorCategory::ContextOverflow
        } else if msg_lower.contains("auth") || msg_lower.contains("permission")
            || msg_lower.contains("denied") || msg_lower.contains("unauthorized")
            || msg_lower.contains("api key") || msg_lower.contains("401") || msg_lower.contains("403") {
            ErrorCategory::Auth
        } else if msg_lower.contains("tool") || msg_lower.contains("bash")
            || msg_lower.contains("command") || msg_lower.contains("execution") {
            ErrorCategory::ToolError
        } else {
            ErrorCategory::General
        }
    }

    /// Get icon for this error category
    pub fn icon(&self) -> &'static str {
        match self {
            ErrorCategory::Network => "🌐",
            ErrorCategory::RateLimit => "⏱️",
            ErrorCategory::ContextOverflow => "📦",
            ErrorCategory::Auth => "🔐",
            ErrorCategory::ToolError => "🔧",
            ErrorCategory::General => "⚠️",
        }
    }

    /// Get suggested actions for this error type
    pub fn suggestions(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        match self {
            ErrorCategory::Network => vec![
                ("🔄", "Retry", "retry"),
                ("🩺", "Doctor", "/doctor"),
                ("📡", "Status", "/status"),
            ],
            ErrorCategory::RateLimit => vec![
                ("⏳", "Wait & Retry", "wait_retry"),
                ("💰", "Usage", "/usage"),
                ("🔄", "Switch Model", "/model haiku"),
            ],
            ErrorCategory::ContextOverflow => vec![
                ("🗜️", "Compact", "/compact"),
                ("📝", "Summarize", "/summarize"),
                ("🆕", "New Chat", "new_conversation"),
            ],
            ErrorCategory::Auth => vec![
                ("🔑", "Login", "/login"),
                ("🩺", "Doctor", "/doctor"),
                ("⚙️", "Config", "/config"),
            ],
            ErrorCategory::ToolError => vec![
                ("🐛", "Debug", "/debug"),
                ("🔄", "Retry", "retry"),
                ("🩺", "Doctor", "/doctor"),
            ],
            ErrorCategory::General => vec![
                ("🐛", "Debug", "/debug"),
                ("🔄", "Retry", "retry"),
                ("🩺", "Doctor", "/doctor"),
            ],
        }
    }

    /// Get extended skill suggestions for recovery
    pub fn skill_suggestions(&self) -> Vec<(&'static str, &'static str, &'static str, &'static str)> {
        // (icon, label, command, description)
        match self {
            ErrorCategory::Network => vec![
                ("🩺", "Doctor", "/doctor", "Check Claude Code health"),
                ("🔍", "Explore", "/explore", "Explore codebase offline"),
                ("📖", "Explain", "/explain", "Explain code without network"),
            ],
            ErrorCategory::RateLimit => vec![
                ("💰", "Usage", "/usage", "Check token usage"),
                ("🗜️", "Compact", "/compact", "Reduce context size"),
                ("🚀", "Oneshot", "/oneshot", "Quick single-task mode"),
            ],
            ErrorCategory::ContextOverflow => vec![
                ("🗜️", "Compact", "/compact", "Compress conversation"),
                ("🆕", "New Chat", "new_conversation", "Start fresh session"),
                ("📝", "Memory", "/memory", "Save to CLAUDE.md"),
                ("🔍", "Search", "/search", "Quick search instead"),
            ],
            ErrorCategory::Auth => vec![
                ("🔑", "Login", "/login", "Authenticate with Claude"),
                ("🩺", "Doctor", "/doctor", "Diagnose auth issues"),
                ("⚙️", "Config", "/config", "Check configuration"),
            ],
            ErrorCategory::ToolError => vec![
                ("🐛", "Debug", "/debug", "Analyze the error"),
                ("🔧", "CI Fixer", "/ci-fixer", "Auto-fix CI failures"),
                ("👀", "Review", "/review-code", "Review for issues"),
                ("📖", "Explain", "/explain", "Understand the error"),
            ],
            ErrorCategory::General => vec![
                ("🐛", "Debug", "/debug", "Deep error analysis"),
                ("💡", "Brainstorm", "/brainstorm", "Research solutions"),
                ("🔍", "Explore", "/explore", "Explore related code"),
                ("🩺", "Doctor", "/doctor", "Check system health"),
            ],
        }
    }

    /// Get a helpful tip for this error category
    pub fn tip(&self) -> &'static str {
        match self {
            ErrorCategory::Network => "Tip: /explore and /explain work offline for local code",
            ErrorCategory::RateLimit => "Tip: Use /compact to reduce context and avoid rate limits",
            ErrorCategory::ContextOverflow => "Tip: /memory saves key info to CLAUDE.md for persistence",
            ErrorCategory::Auth => "Tip: Run 'claude doctor' in terminal to diagnose auth issues",
            ErrorCategory::ToolError => "Tip: /debug provides step-by-step error analysis",
            ErrorCategory::General => "Tip: /brainstorm helps research complex problems",
        }
    }
}

pub struct ErrorInfo {
    /// Error message
    pub message: String,
    /// The prompt that caused the error
    pub original_prompt: Option<String>,
    /// When the error occurred
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Whether retry is possible
    pub can_retry: bool,
    /// Error category for smart suggestions
    pub category: ErrorCategory,
}
