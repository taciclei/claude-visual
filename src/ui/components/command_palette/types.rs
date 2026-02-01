//! Types and constants for command palette

/// A command that can be executed from the palette
#[derive(Clone)]
pub struct Command {
    /// Unique identifier
    pub id: &'static str,
    /// Display label
    pub label: &'static str,
    /// Optional keyboard shortcut
    pub shortcut: Option<&'static str>,
    /// Category for grouping
    pub category: &'static str,
}

impl Command {
    pub const fn new(id: &'static str, label: &'static str, shortcut: Option<&'static str>, category: &'static str) -> Self {
        Self { id, label, shortcut, category }
    }
}

/// Built-in commands
pub const COMMANDS: &[Command] = &[
    // Chat commands
    Command::new("new_conversation", "New Conversation", Some("⌘N"), "Chat"),
    Command::new("export_markdown", "Export as Markdown", Some("⌘E"), "Chat"),
    Command::new("clear_conversation", "Clear Conversation", None, "Chat"),
    Command::new("search_messages", "Search Messages", Some("⌘F"), "Chat"),
    Command::new("copy_conversation", "Copy Conversation", Some("⇧⌘C"), "Chat"),
    Command::new("collapse_all", "Collapse All Messages", Some("⌘["), "Chat"),
    Command::new("expand_all", "Expand All Messages", Some("⌘]"), "Chat"),
    Command::new("toggle_compact", "Toggle Compact Mode", None, "Chat"),
    Command::new("toggle_timestamps", "Toggle Timestamps", None, "Chat"),
    Command::new("toggle_bookmarks", "Show Bookmarked Only", None, "Chat"),
    // Claude Code Skills - Implementation
    Command::new("skill_apex", "APEX: Full Implementation Workflow", None, "Skills"),
    Command::new("skill_ultrathink", "Ultrathink: Deep Thinking Mode", None, "Skills"),
    Command::new("skill_oneshot", "Oneshot: Ultra-fast Implementation", None, "Skills"),
    Command::new("skill_refactor", "Refactor: Multi-agent Refactoring", None, "Skills"),
    Command::new("skill_clean_code", "Clean Code: Apply Best Practices", None, "Skills"),
    // Claude Code Skills - Research & Exploration
    Command::new("skill_explore", "Explore: Deep Codebase Analysis", None, "Skills"),
    Command::new("skill_search", "Search: Quick Answer Lookup", None, "Skills"),
    Command::new("skill_explain", "Explain: Code Feature Deep Dive", None, "Skills"),
    Command::new("skill_brainstorm", "Brainstorm: Multi-perspective Research", None, "Skills"),
    Command::new("skill_docs", "Docs: Documentation Research", None, "Skills"),
    // Claude Code Skills - Debugging
    Command::new("skill_debug", "Debug: Systematic Error Analysis", None, "Skills"),
    Command::new("skill_ci_fixer", "CI Fixer: Auto-fix Pipeline Errors", None, "Skills"),
    // Claude Code Skills - Git Operations
    Command::new("skill_commit", "Commit: Smart Commit Message", None, "Git Skills"),
    Command::new("skill_create_pr", "Create PR: Generate Pull Request", None, "Git Skills"),
    Command::new("skill_review", "Review: Deep PR Analysis", None, "Git Skills"),
    Command::new("skill_merge", "Merge: Intelligent Branch Merge", None, "Git Skills"),
    Command::new("skill_fix_pr", "Fix PR: Address Review Comments", None, "Git Skills"),
    // Claude CLI commands
    Command::new("cli_resume", "Resume Previous Session", None, "Claude"),
    Command::new("cli_usage", "Show Token Usage", None, "Claude"),
    Command::new("cli_memory", "Manage Memory", None, "Claude"),
    Command::new("cli_compact", "Compact Context", None, "Claude"),
    Command::new("cli_model", "Switch Model", None, "Claude"),
    Command::new("cli_config", "Claude Configuration", None, "Claude"),
    Command::new("cli_permissions", "Manage Permissions", None, "Claude"),
    Command::new("cli_doctor", "Run Diagnostics", None, "Claude"),
    Command::new("cli_think", "Enable Extended Thinking", None, "Claude"),
    Command::new("cli_status", "Check Claude Status", None, "Claude"),
    // View commands
    Command::new("toggle_sidebar", "Toggle Sidebar", Some("⌘B"), "View"),
    Command::new("toggle_theme", "Toggle Dark/Light Theme", None, "View"),
    Command::new("toggle_focus", "Toggle Focus Mode", Some("⌘\\"), "View"),
    Command::new("toggle_stats", "Toggle Stats Bar", None, "View"),
    Command::new("toggle_word_wrap", "Toggle Word Wrap", None, "View"),
    Command::new("toggle_line_numbers", "Toggle Line Numbers", None, "View"),
    Command::new("increase_font", "Increase Font Size", Some("⌘+"), "View"),
    Command::new("decrease_font", "Decrease Font Size", Some("⌘-"), "View"),
    Command::new("reset_font", "Reset Font Size", Some("⌘0"), "View"),
    Command::new("show_shortcuts", "Show Keyboard Shortcuts", Some("⌘?"), "View"),
    // Panel commands
    Command::new("show_projects", "Show Projects Panel", None, "Panels"),
    Command::new("show_files", "Show Files Panel", None, "Panels"),
    Command::new("show_history", "Show History Panel", None, "Panels"),
    Command::new("show_git", "Show Git Panel", None, "Panels"),
    Command::new("show_team", "Show Team Panel", None, "Panels"),
    // Tab commands
    Command::new("new_tab", "New Tab", Some("⌘T"), "Tabs"),
    Command::new("close_tab", "Close Tab", Some("⌘W"), "Tabs"),
    Command::new("next_tab", "Next Tab", Some("⌃⇥"), "Tabs"),
    Command::new("prev_tab", "Previous Tab", Some("⌃⇧⇥"), "Tabs"),
    // Navigation commands
    Command::new("scroll_top", "Scroll to Top", Some("⌘↑"), "Navigate"),
    Command::new("scroll_bottom", "Scroll to Bottom", Some("⌘↓"), "Navigate"),
    Command::new("select_next_msg", "Select Next Message", Some("⌥↓"), "Navigate"),
    Command::new("select_prev_msg", "Select Previous Message", Some("⌥↑"), "Navigate"),
    // Project commands
    Command::new("open_project", "Open Project", Some("⌘O"), "Project"),
    Command::new("add_favorite", "Add to Favorites", None, "Project"),
    // Team commands
    Command::new("team_create", "Create New Team", None, "Team"),
    Command::new("team_invite", "Invite Team Member", None, "Team"),
    Command::new("team_activity", "View Team Activity", None, "Team"),
    Command::new("team_analytics", "View Team Analytics", None, "Team"),
    // App commands
    Command::new("settings", "Open Settings", Some("⌘,"), "App"),
    Command::new("check_updates", "Check for Updates", None, "App"),
    Command::new("quit", "Quit Application", Some("⌘Q"), "App"),
];

/// Events emitted by CommandPalette
pub enum CommandPaletteEvent {
    /// A command was selected
    CommandSelected(String),
    /// Palette was dismissed
    Dismissed,
}
