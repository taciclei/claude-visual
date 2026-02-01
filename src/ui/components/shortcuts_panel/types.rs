//! Types for keyboard shortcuts

/// A keyboard shortcut group
#[derive(Clone)]
pub struct ShortcutGroup {
    /// Group name
    pub name: &'static str,
    /// Shortcuts in this group
    pub shortcuts: &'static [Shortcut],
}

/// A single keyboard shortcut
#[derive(Clone)]
pub struct Shortcut {
    /// Key combination (e.g., "Cmd+N")
    pub keys: &'static str,
    /// Description of the action
    pub description: &'static str,
}

/// All keyboard shortcuts organized by category
pub const SHORTCUT_GROUPS: &[ShortcutGroup] = &[
    ShortcutGroup {
        name: "General",
        shortcuts: &[
            Shortcut {
                keys: "⌘Q",
                description: "Quit application",
            },
            Shortcut {
                keys: "⌘,",
                description: "Open settings",
            },
            Shortcut {
                keys: "⌘K",
                description: "Open command palette",
            },
            Shortcut {
                keys: "⌘B",
                description: "Toggle sidebar",
            },
            Shortcut {
                keys: "⌘⇧T",
                description: "Toggle dark/light theme",
            },
            Shortcut {
                keys: "⌘⇧H",
                description: "Toggle high contrast",
            },
            Shortcut {
                keys: "⌘⇧F",
                description: "Toggle focus mode",
            },
            Shortcut {
                keys: "⌘?",
                description: "Show keyboard shortcuts",
            },
            Shortcut {
                keys: "⎋",
                description: "Dismiss overlays/Stop streaming",
            },
        ],
    },
    ShortcutGroup {
        name: "Claude CLI",
        shortcuts: &[
            Shortcut {
                keys: "/resume",
                description: "Resume previous session",
            },
            Shortcut {
                keys: "/usage",
                description: "Show token usage & costs",
            },
            Shortcut {
                keys: "/clear",
                description: "Clear conversation",
            },
            Shortcut {
                keys: "/compact",
                description: "Compact context",
            },
            Shortcut {
                keys: "/memory",
                description: "Manage persistent memory",
            },
            Shortcut {
                keys: "/model",
                description: "Switch AI model",
            },
            Shortcut {
                keys: "/help",
                description: "Show available commands",
            },
            Shortcut {
                keys: "/think",
                description: "Enable extended thinking",
            },
            Shortcut {
                keys: "/review",
                description: "Request code review",
            },
            Shortcut {
                keys: "/pr",
                description: "Create pull request",
            },
            Shortcut {
                keys: "/config",
                description: "View configuration",
            },
            Shortcut {
                keys: "/doctor",
                description: "Run diagnostics",
            },
            Shortcut {
                keys: "@file:path",
                description: "Attach file to context",
            },
        ],
    },
    ShortcutGroup {
        name: "Conversations",
        shortcuts: &[
            Shortcut {
                keys: "⌘N",
                description: "New conversation",
            },
            Shortcut {
                keys: "⌘E",
                description: "Export conversation",
            },
            Shortcut {
                keys: "⇧⌘C",
                description: "Copy conversation",
            },
            Shortcut {
                keys: "⇧⌘⌫",
                description: "Clear conversation",
            },
        ],
    },
    ShortcutGroup {
        name: "Tabs",
        shortcuts: &[
            Shortcut {
                keys: "⌘T",
                description: "New tab",
            },
            Shortcut {
                keys: "⌘W",
                description: "Close tab",
            },
            Shortcut {
                keys: "⌃⇥",
                description: "Next tab",
            },
            Shortcut {
                keys: "⌃⇧⇥",
                description: "Previous tab",
            },
            Shortcut {
                keys: "⌘1-9",
                description: "Switch to tab 1-9",
            },
        ],
    },
    ShortcutGroup {
        name: "Split View",
        shortcuts: &[
            Shortcut {
                keys: "⌘\\",
                description: "Split horizontal",
            },
            Shortcut {
                keys: "⇧⌘\\",
                description: "Split vertical",
            },
            Shortcut {
                keys: "⌥⌘→",
                description: "Focus next pane",
            },
            Shortcut {
                keys: "⌥⌘←",
                description: "Focus previous pane",
            },
            Shortcut {
                keys: "⇧⌘W",
                description: "Close pane",
            },
        ],
    },
    ShortcutGroup {
        name: "Chat View",
        shortcuts: &[
            Shortcut {
                keys: "⌘F",
                description: "Search in conversation",
            },
            Shortcut {
                keys: "⌘G",
                description: "Next search result",
            },
            Shortcut {
                keys: "⇧⌘G",
                description: "Previous search result",
            },
            Shortcut {
                keys: "⌘I",
                description: "Toggle stats panel",
            },
            Shortcut {
                keys: "⌘[",
                description: "Collapse all messages",
            },
            Shortcut {
                keys: "⌘]",
                description: "Expand all messages",
            },
            Shortcut {
                keys: "⌥⇧B",
                description: "Show bookmarked only",
            },
            Shortcut {
                keys: "⌥F",
                description: "Cycle message filter",
            },
        ],
    },
    ShortcutGroup {
        name: "Search Filters",
        shortcuts: &[
            Shortcut {
                keys: "Aa",
                description: "Toggle case-sensitive search",
            },
            Shortcut {
                keys: ".*",
                description: "Toggle regex search mode",
            },
            Shortcut {
                keys: "Click role",
                description: "Cycle search role filter",
            },
            Shortcut {
                keys: "All/You/Claude/Tools",
                description: "Filter by message type",
            },
        ],
    },
    ShortcutGroup {
        name: "Session & Response",
        shortcuts: &[
            Shortcut {
                keys: "⌥S",
                description: "Show session details",
            },
            Shortcut {
                keys: "⌥T",
                description: "Toggle thinking display",
            },
            Shortcut {
                keys: "⌘⇧H",
                description: "Session history",
            },
            Shortcut {
                keys: "⌘M",
                description: "Switch model",
            },
            Shortcut {
                keys: "⌘⇧R",
                description: "Regenerate response",
            },
            Shortcut {
                keys: "⌥⇧C",
                description: "Copy last response",
            },
            Shortcut {
                keys: "⌘.",
                description: "Stop streaming",
            },
            Shortcut {
                keys: "⌥C",
                description: "Continue response",
            },
        ],
    },
    ShortcutGroup {
        name: "Message Navigation",
        shortcuts: &[
            Shortcut {
                keys: "⌥↓",
                description: "Select next message",
            },
            Shortcut {
                keys: "⌥↑",
                description: "Select previous message",
            },
            Shortcut {
                keys: "⌥Home",
                description: "Select first message",
            },
            Shortcut {
                keys: "⌥End",
                description: "Select last message",
            },
            Shortcut {
                keys: "⌥C",
                description: "Copy selected message",
            },
            Shortcut {
                keys: "⌥B",
                description: "Bookmark selected message",
            },
            Shortcut {
                keys: "⌘↑",
                description: "Scroll to top",
            },
            Shortcut {
                keys: "⌘↓",
                description: "Scroll to bottom",
            },
            Shortcut {
                keys: "⌘[",
                description: "Navigate back in history",
            },
            Shortcut {
                keys: "⌘]",
                description: "Navigate forward in history",
            },
            Shortcut {
                keys: "Enter",
                description: "Jump to search result",
            },
        ],
    },
    ShortcutGroup {
        name: "Editor",
        shortcuts: &[
            Shortcut {
                keys: "⌃⇧V",
                description: "Toggle vim mode",
            },
            Shortcut {
                keys: "⌥W",
                description: "Toggle word wrap",
            },
            Shortcut {
                keys: "⌥L",
                description: "Toggle line numbers",
            },
            Shortcut {
                keys: "⌥C",
                description: "Toggle compact mode",
            },
            Shortcut {
                keys: "⌥T",
                description: "Toggle timestamps",
            },
            Shortcut {
                keys: "⌘+",
                description: "Increase font size",
            },
            Shortcut {
                keys: "⌘-",
                description: "Decrease font size",
            },
            Shortcut {
                keys: "⌘0",
                description: "Reset font size",
            },
        ],
    },
    ShortcutGroup {
        name: "Input",
        shortcuts: &[
            Shortcut {
                keys: "⏎",
                description: "Send message",
            },
            Shortcut {
                keys: "⇧⏎",
                description: "New line",
            },
            Shortcut {
                keys: "/",
                description: "Open command menu",
            },
            Shortcut {
                keys: "@",
                description: "Mention/attach file",
            },
            Shortcut {
                keys: "⌘T",
                description: "Prompt templates",
            },
            Shortcut {
                keys: "⌘/",
                description: "Commands & skills",
            },
            Shortcut {
                keys: "⌥⏎",
                description: "Edit last message",
            },
            Shortcut {
                keys: "↑↓",
                description: "Navigate input history",
            },
            Shortcut {
                keys: "⌃U",
                description: "Clear input line",
            },
            Shortcut {
                keys: "⌃K",
                description: "Kill to end of line",
            },
            Shortcut {
                keys: "⌃W",
                description: "Delete word before",
            },
            Shortcut {
                keys: "⌃A",
                description: "Go to beginning",
            },
            Shortcut {
                keys: "⌃E",
                description: "Go to end",
            },
        ],
    },
    ShortcutGroup {
        name: "Message Actions",
        shortcuts: &[
            Shortcut {
                keys: "⌥C",
                description: "Copy selected message",
            },
            Shortcut {
                keys: "⌥B",
                description: "Bookmark selected",
            },
            Shortcut {
                keys: "⌥P",
                description: "Pin/unpin selected",
            },
            Shortcut {
                keys: "⌥⇧C",
                description: "Copy last response",
            },
            Shortcut {
                keys: "⌥Q",
                description: "Quote selected in input",
            },
            Shortcut {
                keys: "⌥⇧B",
                description: "Branch from selected",
            },
            Shortcut {
                keys: "⌥⇧R",
                description: "Retry from selected",
            },
            Shortcut {
                keys: "⌥⌫",
                description: "Delete selected",
            },
            Shortcut {
                keys: "⌘E",
                description: "Export options",
            },
        ],
    },
    ShortcutGroup {
        name: "Notes & Organization",
        shortcuts: &[
            Shortcut {
                keys: "⌥N",
                description: "Session notes",
            },
            Shortcut {
                keys: "⌥F",
                description: "Favorite prompts",
            },
            Shortcut {
                keys: "⌥⇧P",
                description: "View pinned messages",
            },
            Shortcut {
                keys: "⌥⇧F",
                description: "Toggle focus mode",
            },
            Shortcut {
                keys: "⌘I",
                description: "View statistics",
            },
            Shortcut {
                keys: "⌘,",
                description: "Quick settings",
            },
        ],
    },
    ShortcutGroup {
        name: "Summary & Title",
        shortcuts: &[
            Shortcut {
                keys: "⌥⇧S",
                description: "Request AI summary",
            },
            Shortcut {
                keys: "Palette",
                description: "Auto-generate title",
            },
            Shortcut {
                keys: "Palette",
                description: "Request AI title",
            },
            Shortcut {
                keys: "Palette",
                description: "Export shareable summary",
            },
        ],
    },
    ShortcutGroup {
        name: "Input History",
        shortcuts: &[
            Shortcut {
                keys: "↑",
                description: "Previous input",
            },
            Shortcut {
                keys: "↓",
                description: "Next input",
            },
        ],
    },
    ShortcutGroup {
        name: "Quick Actions (FAB)",
        shortcuts: &[
            Shortcut {
                keys: "⚡ Button",
                description: "Quick actions menu",
            },
            Shortcut {
                keys: "📝 Notes",
                description: "Open session notes",
            },
            Shortcut {
                keys: "📌 Pinned",
                description: "View pinned messages",
            },
            Shortcut {
                keys: "📊 Stats",
                description: "View conversation statistics",
            },
            Shortcut {
                keys: "⚙️ Settings",
                description: "Quick settings panel",
            },
            Shortcut {
                keys: "📦 Compact",
                description: "Free up context space",
            },
            Shortcut {
                keys: "💾 Export",
                description: "Export conversation",
            },
            Shortcut {
                keys: "📋 Summary",
                description: "Summarize conversation",
            },
            Shortcut {
                keys: "🔄 Retry",
                description: "Retry last request",
            },
            Shortcut {
                keys: "▶️ Continue",
                description: "Continue response",
            },
        ],
    },
    ShortcutGroup {
        name: "Session Health",
        shortcuts: &[
            Shortcut {
                keys: "💚/💛/❤️",
                description: "Session health indicator",
            },
            Shortcut {
                keys: "⚡/⏱/🐌",
                description: "Response latency indicator",
            },
            Shortcut {
                keys: "In/Out tokens",
                description: "Token usage breakdown",
            },
            Shortcut {
                keys: "Progress bar",
                description: "Context usage (blue=input, green=output)",
            },
        ],
    },
    ShortcutGroup {
        name: "Quick Replies",
        shortcuts: &[
            Shortcut {
                keys: "💡 Explain",
                description: "Request more detail",
            },
            Shortcut {
                keys: "🔀 Alternative",
                description: "Ask for alternative approach",
            },
            Shortcut {
                keys: "📋 Summary",
                description: "Summarize discussion",
            },
            Shortcut {
                keys: "🧪 Tests",
                description: "Request unit tests",
            },
        ],
    },
    ShortcutGroup {
        name: "Files",
        shortcuts: &[
            Shortcut {
                keys: "⌥R",
                description: "Recent files panel",
            },
            Shortcut {
                keys: "@file:path",
                description: "Mention file in input",
            },
            Shortcut {
                keys: "📂 Button",
                description: "Quick file access",
            },
        ],
    },
    ShortcutGroup {
        name: "Claude Code Skills",
        shortcuts: &[
            Shortcut {
                keys: "/apex",
                description: "APEX workflow: Analyze-Plan-Execute-eXamine",
            },
            Shortcut {
                keys: "/brainstorm",
                description: "Deep research with skeptical analysis",
            },
            Shortcut {
                keys: "/explore",
                description: "Deep codebase exploration",
            },
            Shortcut {
                keys: "/ultrathink",
                description: "Deep thinking - craft elegant solutions",
            },
            Shortcut {
                keys: "/debug",
                description: "Systematic bug resolution",
            },
            Shortcut {
                keys: "/review",
                description: "Expert code review (security, SOLID)",
            },
            Shortcut {
                keys: "/oneshot",
                description: "Ultra-fast: Explore → Code → Test",
            },
            Shortcut {
                keys: "/explain",
                description: "Deep code explanation with diagrams",
            },
            Shortcut {
                keys: "/refactor",
                description: "Parallel refactoring agents",
            },
            Shortcut {
                keys: "/docs",
                description: "Documentation research",
            },
            Shortcut {
                keys: "/search",
                description: "Lightning-fast codebase search",
            },
        ],
    },
    ShortcutGroup {
        name: "Git Skills",
        shortcuts: &[
            Shortcut {
                keys: "/commit",
                description: "Quick commit with clean messages",
            },
            Shortcut {
                keys: "/create-pr",
                description: "Create PR with auto description",
            },
            Shortcut {
                keys: "/fix-pr-comments",
                description: "Implement PR review changes",
            },
            Shortcut {
                keys: "/merge",
                description: "Smart branch merging",
            },
        ],
    },
    ShortcutGroup {
        name: "Context Management",
        shortcuts: &[
            Shortcut {
                keys: "/compact",
                description: "Free up context space (URGENT when >80%)",
            },
            Shortcut {
                keys: "/memory",
                description: "Manage persistent project memory",
            },
            Shortcut {
                keys: "/add-dir",
                description: "Add directory to context",
            },
            Shortcut {
                keys: "/status",
                description: "View current context status",
            },
        ],
    },
    ShortcutGroup {
        name: "Workflow Templates",
        shortcuts: &[
            Shortcut {
                keys: "Bug Fix",
                description: "/debug → fix → /commit",
            },
            Shortcut {
                keys: "Feature",
                description: "/apex → implement → /review → /commit",
            },
            Shortcut {
                keys: "Refactor",
                description: "/explore → /refactor → /review",
            },
            Shortcut {
                keys: "Research",
                description: "/brainstorm → /docs → summarize",
            },
            Shortcut {
                keys: "Quick Fix",
                description: "/oneshot for simple changes",
            },
        ],
    },
];

/// Events emitted by ShortcutsPanel
pub enum ShortcutsPanelEvent {
    /// Panel was dismissed
    Dismissed,
}
