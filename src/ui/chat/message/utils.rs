//! Utility functions for message rendering

/// Format a timestamp as relative time ("just now", "2 min ago") for recent times,
/// or absolute time for older ones
pub fn format_relative_time(timestamp: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let duration = now.signed_duration_since(timestamp);

    // Future timestamps
    if duration.num_seconds() < 0 {
        return timestamp.format("%H:%M").to_string();
    }

    // Less than 1 minute
    if duration.num_seconds() < 60 {
        return "just now".to_string();
    }

    // Less than 1 hour - show minutes
    if duration.num_minutes() < 60 {
        let mins = duration.num_minutes();
        if mins == 1 {
            return "1 min ago".to_string();
        }
        return format!("{} min ago", mins);
    }

    // Less than 24 hours - show hours
    if duration.num_hours() < 24 {
        let hours = duration.num_hours();
        if hours == 1 {
            return "1 hour ago".to_string();
        }
        return format!("{} hours ago", hours);
    }

    // Less than 7 days - show days
    if duration.num_days() < 7 {
        let days = duration.num_days();
        if days == 1 {
            return "yesterday".to_string();
        }
        return format!("{} days ago", days);
    }

    // Older - show absolute date/time
    let local = timestamp.with_timezone(&chrono::Local);
    if duration.num_days() < 365 {
        // Same year - show month and day
        return local.format("%b %d, %H:%M").to_string();
    }

    // Different year - show full date
    local.format("%b %d %Y, %H:%M").to_string()
}

/// Get icon for a tool based on its name
pub fn tool_icon(tool_name: &str) -> &'static str {
    match tool_name.to_lowercase().as_str() {
        // File operations
        "read" => "📄",
        "write" => "✏️",
        "edit" => "📝",
        "notebookedit" => "📓",
        // Shell/command
        "bash" | "command" | "shell" => "💻",
        // Search tools
        "grep" | "search" => "🔍",
        "glob" | "find" => "📁",
        // Web tools
        "webfetch" | "web" | "fetch" => "🌐",
        "websearch" => "🔎",
        // Agent/task tools
        "task" | "agent" => "🚀",
        "taskoutput" => "📤",
        "taskstop" | "taskcancel" => "⏹️",
        // Git operations
        "git" => "📚",
        // File system
        "mkdir" | "createdir" => "📂",
        "rm" | "delete" | "remove" => "🗑️",
        "mv" | "move" | "rename" => "📦",
        "cp" | "copy" => "📋",
        // Code analysis
        "lint" | "check" => "🔬",
        "test" | "unittest" => "🧪",
        "build" | "compile" => "🔨",
        // MCP tools
        "mcp" => "🔌",
        // Planning/thinking
        "think" | "plan" | "enterplanmode" => "🧠",
        "exitplanmode" => "✅",
        // Questions
        "askuserquestion" => "❓",
        // Skills
        "skill" => "⚡",
        // Todo/tasks
        "taskcreate" | "todowrite" => "📋",
        "taskupdate" | "taskget" | "tasklist" => "📊",
        _ => "🔧",
    }
}

/// Get a short description for a tool
pub fn tool_description(tool_name: &str) -> &'static str {
    match tool_name.to_lowercase().as_str() {
        // File operations
        "read" => "Reading file",
        "write" => "Writing file",
        "edit" => "Editing file",
        "notebookedit" => "Editing notebook",
        // Shell/command
        "bash" | "command" | "shell" => "Running command",
        // Search tools
        "grep" | "search" => "Searching content",
        "glob" | "find" => "Finding files",
        // Web tools
        "webfetch" | "web" | "fetch" => "Fetching web page",
        "websearch" => "Searching web",
        // Agent/task tools
        "task" | "agent" => "Spawning agent",
        "taskoutput" => "Getting task output",
        "taskstop" | "taskcancel" => "Stopping task",
        // Git operations
        "git" => "Git operation",
        // File system
        "mkdir" | "createdir" => "Creating directory",
        "rm" | "delete" | "remove" => "Deleting file",
        "mv" | "move" | "rename" => "Moving file",
        "cp" | "copy" => "Copying file",
        // Code analysis
        "lint" | "check" => "Checking code",
        "test" | "unittest" => "Running tests",
        "build" | "compile" => "Building project",
        // MCP tools
        "mcp" => "MCP server call",
        // Planning/thinking
        "think" | "plan" | "enterplanmode" => "Planning",
        "exitplanmode" => "Exiting plan mode",
        // Questions
        "askuserquestion" => "Asking user",
        // Skills
        "skill" => "Running skill",
        // Todo/tasks
        "taskcreate" | "todowrite" => "Creating task",
        "taskupdate" | "taskget" | "tasklist" => "Managing tasks",
        _ => "Using tool",
    }
}

/// Categorize an error message and return (icon, title, is_retryable)
pub fn categorize_error(error: &str) -> (&'static str, &'static str, bool) {
    let error_lower = error.to_lowercase();

    // Network/API errors (usually retryable)
    if error_lower.contains("network") || error_lower.contains("connection")
        || error_lower.contains("timeout") || error_lower.contains("api")
        || error_lower.contains("rate limit") || error_lower.contains("overloaded")
    {
        return ("🌐", "Network Error", true);
    }

    // Authentication errors
    if error_lower.contains("auth") || error_lower.contains("token")
        || error_lower.contains("permission") || error_lower.contains("unauthorized")
        || error_lower.contains("forbidden") || error_lower.contains("api key")
    {
        return ("🔐", "Authentication Error", false);
    }

    // File/path errors
    if error_lower.contains("file not found") || error_lower.contains("no such file")
        || error_lower.contains("cannot read") || error_lower.contains("cannot write")
        || error_lower.contains("path")
    {
        return ("📁", "File Error", false);
    }

    // Command/execution errors (might be retryable)
    if error_lower.contains("command") || error_lower.contains("exit code")
        || error_lower.contains("failed to execute") || error_lower.contains("process")
    {
        return ("💻", "Command Error", true);
    }

    // Context/token errors
    if error_lower.contains("context") || error_lower.contains("too long")
        || error_lower.contains("token") || error_lower.contains("limit")
    {
        return ("📊", "Context Limit", false);
    }

    // Validation errors
    if error_lower.contains("invalid") || error_lower.contains("malformed")
        || error_lower.contains("parse") || error_lower.contains("syntax")
    {
        return ("⚠️", "Validation Error", false);
    }

    // Tool errors (usually retryable)
    if error_lower.contains("tool") {
        return ("🔧", "Tool Error", true);
    }

    // Default - generic error, usually retryable
    ("❌", "Error", true)
}
