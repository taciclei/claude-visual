//! Command palette command definitions
//!
//! This module contains the static command definitions used by the command palette.

use super::super::types::PaletteCommand;
use super::super::core::ChatView;

impl ChatView {
    /// Get all available commands for the command palette
    pub(crate) fn get_palette_commands() -> Vec<PaletteCommand> {
        vec![
            // Navigation
            PaletteCommand { id: "scroll_to_top", label: "Go to First Message", description: "Navigate to the first message", shortcut: Some("⌥Home"), category: "Navigation", icon: "⬆" },
            PaletteCommand { id: "scroll_to_bottom", label: "Go to Last Message", description: "Navigate to the last message", shortcut: Some("⌥End"), category: "Navigation", icon: "⬇" },
            PaletteCommand { id: "prev_message", label: "Previous Message", description: "Select the previous message", shortcut: Some("⌥↑"), category: "Navigation", icon: "↑" },
            PaletteCommand { id: "next_message", label: "Next Message", description: "Select the next message", shortcut: Some("⌥↓"), category: "Navigation", icon: "↓" },
            PaletteCommand { id: "navigate_back", label: "Navigate Back", description: "Go to previously viewed message", shortcut: Some("⌘["), category: "Navigation", icon: "◀" },
            PaletteCommand { id: "navigate_forward", label: "Navigate Forward", description: "Go to next viewed message", shortcut: Some("⌘]"), category: "Navigation", icon: "▶" },
            PaletteCommand { id: "jump_to_search_result", label: "Jump to Search Result", description: "Scroll to and highlight current search result", shortcut: Some("Enter"), category: "Navigation", icon: "↵" },

            // View
            PaletteCommand { id: "toggle_search", label: "Search in Conversation", description: "Find text in messages", shortcut: Some("⌘F"), category: "View", icon: "🔍" },
            PaletteCommand { id: "toggle_stats", label: "Toggle Statistics Bar", description: "Show/hide token usage stats", shortcut: Some("⌘I"), category: "View", icon: "📊" },
            PaletteCommand { id: "toggle_timestamps", label: "Toggle Timestamps", description: "Show/hide message timestamps", shortcut: None, category: "View", icon: "🕐" },
            PaletteCommand { id: "toggle_time_separators", label: "Toggle Time Separators", description: "Show/hide date separators between messages", shortcut: None, category: "View", icon: "📅" },
            PaletteCommand { id: "toggle_compact", label: "Toggle Compact Mode", description: "Switch to compact view", shortcut: None, category: "View", icon: "📐" },
            PaletteCommand { id: "toggle_word_wrap", label: "Toggle Word Wrap", description: "Wrap long lines", shortcut: Some("⌥W"), category: "View", icon: "↩" },
            PaletteCommand { id: "toggle_line_numbers", label: "Toggle Line Numbers", description: "Show/hide line numbers in code", shortcut: Some("⌥L"), category: "View", icon: "#" },
            PaletteCommand { id: "toggle_theme", label: "Toggle Theme", description: "Switch between light/dark mode", shortcut: Some("⇧⌘T"), category: "View", icon: "🌓" },

            // Actions
            PaletteCommand { id: "copy_conversation", label: "Copy Conversation", description: "Copy all messages to clipboard", shortcut: Some("⇧⌘C"), category: "Actions", icon: "📋" },
            PaletteCommand { id: "copy_last_response", label: "Copy Last Response", description: "Copy Claude's last response", shortcut: Some("⌥⇧C"), category: "Actions", icon: "📋" },
            PaletteCommand { id: "export_conversation", label: "Export Conversation", description: "Export with format options", shortcut: Some("⌘E"), category: "Actions", icon: "💾" },
            PaletteCommand { id: "clear_conversation", label: "Clear Conversation", description: "Start a new conversation", shortcut: None, category: "Actions", icon: "🗑" },
            PaletteCommand { id: "copy_selected", label: "Copy Selected Message", description: "Copy the selected message", shortcut: Some("⌥C"), category: "Actions", icon: "📄" },
            PaletteCommand { id: "bookmark_selected", label: "Bookmark Message", description: "Toggle bookmark on selected", shortcut: Some("⌥B"), category: "Actions", icon: "⭐" },
            PaletteCommand { id: "edit_last_message", label: "Edit Last Message", description: "Edit your last message", shortcut: Some("⌥⏎"), category: "Actions", icon: "✏️" },
            PaletteCommand { id: "pin_selected", label: "Pin Message", description: "Pin/unpin selected message", shortcut: Some("⌥P"), category: "Actions", icon: "📌" },

            // Messages
            PaletteCommand { id: "expand_all", label: "Expand All Messages", description: "Expand all collapsed messages", shortcut: None, category: "Messages", icon: "📂" },
            PaletteCommand { id: "collapse_all", label: "Collapse All Messages", description: "Collapse all messages", shortcut: None, category: "Messages", icon: "📁" },
            PaletteCommand { id: "collapse_tools", label: "Collapse Tool Messages", description: "Collapse all tool use/result messages", shortcut: Some("⌥⇧T"), category: "Messages", icon: "🔧" },
            PaletteCommand { id: "expand_tools", label: "Expand Tool Messages", description: "Expand all tool use/result messages", shortcut: None, category: "Messages", icon: "🔧" },
            PaletteCommand { id: "toggle_tools", label: "Toggle Tool Messages", description: "Toggle collapse state of tool messages", shortcut: None, category: "Messages", icon: "🔧" },
            PaletteCommand { id: "collapse_assistant", label: "Collapse Claude Messages", description: "Collapse all assistant messages", shortcut: None, category: "Messages", icon: "🤖" },
            PaletteCommand { id: "expand_assistant", label: "Expand Claude Messages", description: "Expand all assistant messages", shortcut: None, category: "Messages", icon: "🤖" },
            PaletteCommand { id: "filter_all", label: "Show All Messages", description: "Remove message filters", shortcut: None, category: "Messages", icon: "👁" },
            PaletteCommand { id: "filter_user", label: "Show User Messages Only", description: "Filter to your messages", shortcut: None, category: "Messages", icon: "👤" },
            PaletteCommand { id: "filter_assistant", label: "Show Claude Messages Only", description: "Filter to Claude's responses", shortcut: None, category: "Messages", icon: "🤖" },
            PaletteCommand { id: "filter_tools", label: "Show Tool Messages Only", description: "Filter to tool use/results", shortcut: None, category: "Messages", icon: "🔧" },

            // Claude CLI Commands
            PaletteCommand { id: "cmd_resume", label: "/resume", description: "Resume a previous conversation", shortcut: None, category: "Commands", icon: "↩" },
            PaletteCommand { id: "cmd_usage", label: "/usage", description: "Show token usage and costs", shortcut: None, category: "Commands", icon: "📊" },
            PaletteCommand { id: "cmd_help", label: "/help", description: "Show available commands", shortcut: None, category: "Commands", icon: "❓" },
            PaletteCommand { id: "cmd_config", label: "/config", description: "Open configuration", shortcut: None, category: "Commands", icon: "⚙" },
            PaletteCommand { id: "cmd_memory", label: "/memory", description: "Manage persistent memory", shortcut: None, category: "Commands", icon: "🧠" },
            PaletteCommand { id: "cmd_model", label: "/model", description: "Switch AI model", shortcut: None, category: "Commands", icon: "🔄" },
            PaletteCommand { id: "cmd_compact", label: "/compact", description: "Compact conversation context", shortcut: None, category: "Commands", icon: "📦" },
            PaletteCommand { id: "cmd_vim", label: "/vim", description: "Toggle vim mode", shortcut: None, category: "Commands", icon: "⌨" },
            PaletteCommand { id: "cmd_doctor", label: "/doctor", description: "Run diagnostics check", shortcut: None, category: "Commands", icon: "🩺" },
            PaletteCommand { id: "cmd_permissions", label: "/permissions", description: "Manage tool permissions", shortcut: None, category: "Commands", icon: "🔐" },
            PaletteCommand { id: "cmd_init", label: "/init", description: "Initialize project setup", shortcut: None, category: "Commands", icon: "🚀" },
            PaletteCommand { id: "cmd_add_dir", label: "/add-dir", description: "Add directory to context", shortcut: None, category: "Commands", icon: "📁" },
            PaletteCommand { id: "cmd_clear", label: "/clear", description: "Clear conversation context", shortcut: None, category: "Commands", icon: "🗑" },
            PaletteCommand { id: "cmd_cost", label: "/cost", description: "Show cost breakdown", shortcut: None, category: "Commands", icon: "💰" },
            PaletteCommand { id: "cmd_status", label: "/status", description: "Show session status", shortcut: None, category: "Commands", icon: "📊" },
            PaletteCommand { id: "cmd_think", label: "/think", description: "Enable extended thinking mode", shortcut: None, category: "Commands", icon: "🧠" },
            PaletteCommand { id: "cmd_think_off", label: "/think off", description: "Disable extended thinking mode", shortcut: None, category: "Commands", icon: "💭" },
            PaletteCommand { id: "toggle_think", label: "Toggle Think Mode", description: "Toggle extended thinking on/off", shortcut: Some("⌥T"), category: "Workflow", icon: "🧠" },
            PaletteCommand { id: "cmd_review", label: "/review", description: "Request code review", shortcut: None, category: "Commands", icon: "👀" },
            PaletteCommand { id: "cmd_pr", label: "/pr", description: "Create a pull request", shortcut: None, category: "Commands", icon: "🔀" },
            PaletteCommand { id: "cmd_pr_comments", label: "/pr-comments", description: "Show PR review comments", shortcut: None, category: "Commands", icon: "💬" },
            PaletteCommand { id: "cmd_login", label: "/login", description: "Authenticate with Anthropic", shortcut: None, category: "Commands", icon: "🔑" },
            PaletteCommand { id: "cmd_logout", label: "/logout", description: "Log out from Anthropic", shortcut: None, category: "Commands", icon: "🚪" },
            PaletteCommand { id: "cmd_bug", label: "/bug", description: "Report a bug", shortcut: None, category: "Commands", icon: "🐛" },
            PaletteCommand { id: "cmd_mcp", label: "/mcp", description: "Manage MCP servers", shortcut: None, category: "Commands", icon: "🔌" },

            // Response Actions
            PaletteCommand { id: "continue_response", label: "Continue Response", description: "Ask Claude to continue from where it stopped", shortcut: Some("⌥C"), category: "Actions", icon: "▶" },
            PaletteCommand { id: "regenerate_response", label: "Regenerate Response", description: "Regenerate Claude's last response", shortcut: Some("⌘⇧R"), category: "Actions", icon: "🔄" },

            // Model switching
            PaletteCommand { id: "switch_model", label: "Switch Model", description: "Change the AI model", shortcut: Some("⌘M"), category: "Model", icon: "🤖" },

            // Templates
            PaletteCommand { id: "show_templates", label: "Prompt Templates", description: "Browse and use prompt templates", shortcut: Some("⌘T"), category: "Input", icon: "📝" },
            PaletteCommand { id: "show_commands", label: "Commands & Skills", description: "Browse slash commands and skills", shortcut: Some("⌘/"), category: "Input", icon: "/" },
            PaletteCommand { id: "show_context", label: "Session Context", description: "View files, tools, and context usage", shortcut: Some("⌘I"), category: "Session", icon: "📚" },

            // Session & History
            PaletteCommand { id: "session_history", label: "Session History", description: "Browse and resume recent sessions", shortcut: Some("⇧⌘H"), category: "Session", icon: "📋" },
            PaletteCommand { id: "resume_last", label: "Resume Last Session", description: "Quickly resume the most recent session", shortcut: Some("⇧⌘R"), category: "Session", icon: "↩" },
            PaletteCommand { id: "toggle_suggestions", label: "Toggle Suggestions", description: "Show/hide contextual suggestions", shortcut: None, category: "Session", icon: "💡" },

            // Claude Code Quick Commands
            PaletteCommand { id: "cmd_resume", label: "/resume", description: "Resume the last Claude Code session", shortcut: None, category: "Claude Code", icon: "↩" },
            PaletteCommand { id: "cmd_compact", label: "/compact", description: "Compact context to free up tokens", shortcut: None, category: "Claude Code", icon: "📦" },
            PaletteCommand { id: "cmd_usage", label: "/usage", description: "Show token usage and costs", shortcut: None, category: "Claude Code", icon: "📊" },
            PaletteCommand { id: "cmd_memory", label: "/memory", description: "View and edit persistent memory", shortcut: None, category: "Claude Code", icon: "🧠" },
            PaletteCommand { id: "cmd_doctor", label: "/doctor", description: "Run diagnostics on Claude Code setup", shortcut: None, category: "Claude Code", icon: "🩺" },
            PaletteCommand { id: "cmd_permissions", label: "/permissions", description: "Manage tool permissions", shortcut: None, category: "Claude Code", icon: "🔐" },
            PaletteCommand { id: "cmd_config", label: "/config", description: "View Claude Code configuration", shortcut: None, category: "Claude Code", icon: "⚙️" },

            // Claude Code Skills (AI-powered workflows)
            PaletteCommand { id: "skill_apex", label: "/apex", description: "APEX workflow: Analyze-Plan-Execute-eXamine", shortcut: None, category: "Skills", icon: "⚡" },
            PaletteCommand { id: "skill_brainstorm", label: "/brainstorm", description: "Deep iterative research with skeptical analysis", shortcut: None, category: "Skills", icon: "💡" },
            PaletteCommand { id: "skill_explore", label: "/explore", description: "Deep codebase exploration and analysis", shortcut: None, category: "Skills", icon: "🔍" },
            PaletteCommand { id: "skill_debug", label: "/debug", description: "Systematic error debugging and resolution", shortcut: None, category: "Skills", icon: "🐛" },
            PaletteCommand { id: "skill_review", label: "/review", description: "Deep PR review with parallel subagents", shortcut: None, category: "Skills", icon: "👀" },
            PaletteCommand { id: "skill_oneshot", label: "/oneshot", description: "Ultra-fast feature implementation", shortcut: None, category: "Skills", icon: "🚀" },
            PaletteCommand { id: "skill_explain", label: "/explain", description: "Deep code explanation with visual diagrams", shortcut: None, category: "Skills", icon: "📖" },
            PaletteCommand { id: "skill_refactor", label: "/refactor", description: "Refactor code with parallel agents", shortcut: None, category: "Skills", icon: "♻️" },
            PaletteCommand { id: "skill_docs", label: "/docs", description: "Deep documentation research", shortcut: None, category: "Skills", icon: "📚" },
            PaletteCommand { id: "skill_ultrathink", label: "/ultrathink", description: "Deep thinking mode for elegant solutions", shortcut: None, category: "Skills", icon: "🧠" },

            // Git Skills
            PaletteCommand { id: "skill_commit", label: "/commit", description: "Quick commit with clean messages", shortcut: None, category: "Git", icon: "📦" },
            PaletteCommand { id: "skill_create_pr", label: "/create-pr", description: "Create and push PR with auto-generated description", shortcut: None, category: "Git", icon: "🔀" },
            PaletteCommand { id: "skill_fix_pr", label: "/fix-pr-comments", description: "Fetch and implement PR review comments", shortcut: None, category: "Git", icon: "💬" },
            PaletteCommand { id: "skill_merge", label: "/merge", description: "Intelligent merge with conflict resolution", shortcut: None, category: "Git", icon: "🔗" },

            // Permissions
            PaletteCommand { id: "permissions_panel", label: "Permissions Panel", description: "View and manage pending permissions", shortcut: None, category: "Security", icon: "🔐" },

            // Panels
            PaletteCommand { id: "mcp_panel", label: "MCP Servers", description: "View connected MCP servers and tools", shortcut: None, category: "Panels", icon: "🔌" },
            PaletteCommand { id: "tasks_panel", label: "Active Tasks", description: "View running tasks and subagents", shortcut: None, category: "Panels", icon: "⚡" },
            PaletteCommand { id: "git_panel", label: "Git Status", description: "View git repository status", shortcut: Some("⌘G"), category: "Panels", icon: "🔀" },

            // Files
            PaletteCommand { id: "file_picker", label: "Insert File Mention", description: "Browse and insert @file mentions", shortcut: Some("⌘P"), category: "Files", icon: "📎" },

            // Bookmarks
            PaletteCommand { id: "toggle_bookmark", label: "Toggle Bookmark", description: "Bookmark/unbookmark selected message", shortcut: Some("⌘D"), category: "Bookmarks", icon: "⭐" },
            PaletteCommand { id: "show_bookmarks", label: "Show Bookmarks Only", description: "Filter to show only bookmarked messages", shortcut: None, category: "Bookmarks", icon: "📌" },
            PaletteCommand { id: "next_bookmark", label: "Next Bookmark", description: "Jump to next bookmarked message", shortcut: Some("⌥]"), category: "Bookmarks", icon: "→" },
            PaletteCommand { id: "prev_bookmark", label: "Previous Bookmark", description: "Jump to previous bookmarked message", shortcut: Some("⌥["), category: "Bookmarks", icon: "←" },

            // Input mode
            PaletteCommand { id: "toggle_multiline", label: "Toggle Multiline Input", description: "Switch between single and multi-line input mode", shortcut: Some("⌥M"), category: "Input", icon: "↕" },
            PaletteCommand { id: "increase_input_height", label: "Increase Input Height", description: "Make the input area taller (multiline)", shortcut: None, category: "Input", icon: "⬆" },
            PaletteCommand { id: "decrease_input_height", label: "Decrease Input Height", description: "Make the input area shorter (multiline)", shortcut: None, category: "Input", icon: "⬇" },

            // Session & Metrics
            PaletteCommand { id: "session_details", label: "Session Details", description: "Show detailed session information and metrics", shortcut: Some("⌥S"), category: "Session", icon: "📊" },
            PaletteCommand { id: "toggle_thinking", label: "Toggle Thinking Display", description: "Show/hide Claude's reasoning process", shortcut: Some("⌥T"), category: "Session", icon: "🧠" },
            PaletteCommand { id: "copy_session_id", label: "Copy Session ID", description: "Copy current session ID to clipboard", shortcut: None, category: "Session", icon: "📋" },
            PaletteCommand { id: "cmd_clear", label: "/clear", description: "Clear the conversation context", shortcut: None, category: "Commands", icon: "🗑" },

            // Message Actions (on selected message)
            PaletteCommand { id: "branch_selected", label: "Branch from Selected", description: "Edit and resend from selected message", shortcut: Some("⌥⇧B"), category: "Messages", icon: "🔀" },
            PaletteCommand { id: "retry_selected", label: "Retry from Selected", description: "Re-send from selected message", shortcut: Some("⌥⇧R"), category: "Messages", icon: "🔄" },
            PaletteCommand { id: "quote_selected", label: "Quote Selected", description: "Quote selected message in input", shortcut: Some("⌥Q"), category: "Messages", icon: "💬" },
            PaletteCommand { id: "delete_selected", label: "Delete Selected", description: "Delete the selected message", shortcut: Some("⌥⌫"), category: "Messages", icon: "🗑" },

            // Notes & Organization
            PaletteCommand { id: "toggle_notes", label: "Session Notes", description: "Add notes to this session", shortcut: Some("⌥N"), category: "Notes", icon: "📝" },
            PaletteCommand { id: "toggle_tags", label: "Conversation Tags", description: "Add tags to organize conversation", shortcut: None, category: "Notes", icon: "🏷️" },
            PaletteCommand { id: "toggle_favorites", label: "Favorite Prompts", description: "View and use saved prompts", shortcut: Some("⌥F"), category: "Notes", icon: "⭐" },
            PaletteCommand { id: "save_as_favorite", label: "Save to Favorites", description: "Save current input as favorite", shortcut: None, category: "Notes", icon: "💾" },
            PaletteCommand { id: "toggle_pinned", label: "Pinned Messages", description: "View all pinned messages", shortcut: Some("⌥⇧P"), category: "Notes", icon: "📌" },
            PaletteCommand { id: "toggle_recent_files", label: "Recent Files", description: "View and mention recent files", shortcut: Some("⌥R"), category: "Files", icon: "📂" },
            PaletteCommand { id: "toggle_stats", label: "Statistics", description: "View conversation statistics", shortcut: Some("⌘I"), category: "Notes", icon: "📊" },

            // Focus & Workflow
            PaletteCommand { id: "toggle_focus_mode", label: "Focus Mode", description: "Distraction-free input mode", shortcut: Some("⌥⇧F"), category: "Workflow", icon: "🎯" },
            PaletteCommand { id: "quick_resume", label: "Quick Resume", description: "Resume a recent session", shortcut: Some("⌘R"), category: "Workflow", icon: "↩️" },

            // Input
            PaletteCommand { id: "clear_input_history", label: "Clear Input History", description: "Clear all saved input history", shortcut: None, category: "Input", icon: "🗑️" },
            PaletteCommand { id: "toggle_input_hints", label: "Toggle Input Hints", description: "Show/hide input suggestions", shortcut: None, category: "Input", icon: "💡" },

            // Settings
            PaletteCommand { id: "quick_settings", label: "Quick Settings", description: "Toggle common UI settings", shortcut: Some("⌘,"), category: "Settings", icon: "⚙️" },

            // Help
            PaletteCommand { id: "show_shortcuts", label: "Keyboard Shortcuts", description: "Show all keyboard shortcuts", shortcut: Some("⌘?"), category: "Help", icon: "⌨" },

            // Summary & Title
            PaletteCommand { id: "auto_title", label: "Auto-generate Title", description: "Generate title from first message", shortcut: None, category: "Conversation", icon: "✨" },
            PaletteCommand { id: "ai_title", label: "Request AI Title", description: "Ask Claude to suggest a title", shortcut: None, category: "Conversation", icon: "🤖" },
            PaletteCommand { id: "request_summary", label: "Request Summary", description: "Ask Claude to summarize conversation", shortcut: Some("⌥⇧S"), category: "Conversation", icon: "📝" },
            PaletteCommand { id: "quick_summary", label: "Show Quick Stats", description: "Display conversation statistics", shortcut: None, category: "Conversation", icon: "📊" },
            PaletteCommand { id: "export_summary", label: "Export Shareable Summary", description: "Create a shareable conversation summary", shortcut: None, category: "Export", icon: "🔗" },

            // Quick File Mentions
            PaletteCommand { id: "mention_readme", label: "@README.md", description: "Mention README file", shortcut: None, category: "Files", icon: "📖" },
            PaletteCommand { id: "mention_package", label: "@package.json", description: "Mention package.json", shortcut: None, category: "Files", icon: "📦" },
            PaletteCommand { id: "mention_cargo", label: "@Cargo.toml", description: "Mention Cargo.toml", shortcut: None, category: "Files", icon: "🦀" },

            // Session Health & Performance
            PaletteCommand { id: "retry_last_request", label: "Retry Last Request", description: "Retry the last failed request", shortcut: None, category: "Actions", icon: "🔄" },
            PaletteCommand { id: "check_session_health", label: "Check Session Health", description: "View session health status", shortcut: None, category: "Session", icon: "💚" },
            PaletteCommand { id: "clear_quick_replies", label: "Clear Quick Replies", description: "Clear suggested quick replies", shortcut: None, category: "UI", icon: "🗑" },
            PaletteCommand { id: "dismiss_tips", label: "Dismiss All Tips", description: "Hide all onboarding tips", shortcut: None, category: "UI", icon: "👋" },
        ]
    }
}
