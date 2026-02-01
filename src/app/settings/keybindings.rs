//! Keybindings configuration

use serde::{Deserialize, Serialize};

/// Custom keybindings configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Keybindings {
    // ==================== Basic Navigation ====================
    /// Toggle sidebar (default: cmd-b)
    pub toggle_sidebar: String,
    /// New conversation (default: cmd-n)
    pub new_conversation: String,
    /// Open settings (default: cmd-,)
    pub open_settings: String,
    /// Command palette (default: cmd-k)
    pub command_palette: String,
    /// Next tab (default: ctrl-tab)
    pub next_tab: String,
    /// Previous tab (default: ctrl-shift-tab)
    pub prev_tab: String,
    /// Close tab (default: cmd-w)
    pub close_tab: String,
    /// New tab (default: cmd-t)
    pub new_tab: String,
    /// Send message (default: cmd-enter)
    pub send_message: String,
    /// Focus input (default: cmd-l)
    pub focus_input: String,

    // ==================== Message Operations ====================
    /// Copy selected/last message (default: cmd-c)
    #[serde(default = "default_copy_message")]
    pub copy_message: String,
    /// Edit last user message (default: cmd-e)
    #[serde(default = "default_edit_message")]
    pub edit_message: String,
    /// Delete selected message (default: cmd-backspace)
    #[serde(default = "default_delete_message")]
    pub delete_message: String,
    /// Pin/unpin message (default: cmd-p)
    #[serde(default = "default_pin_message")]
    pub pin_message: String,
    /// Bookmark message (default: cmd-d)
    #[serde(default = "default_bookmark_message")]
    pub bookmark_message: String,
    /// Add reaction to message (default: cmd-r)
    #[serde(default = "default_add_reaction")]
    pub add_reaction: String,

    // ==================== Message Navigation ====================
    /// Jump to first message (default: cmd-home)
    #[serde(default = "default_jump_first")]
    pub jump_first: String,
    /// Jump to last message (default: cmd-end)
    #[serde(default = "default_jump_last")]
    pub jump_last: String,
    /// Select previous message (default: up)
    #[serde(default = "default_select_prev")]
    pub select_prev_message: String,
    /// Select next message (default: down)
    #[serde(default = "default_select_next")]
    pub select_next_message: String,
    /// Page up in messages (default: pageup)
    #[serde(default = "default_page_up")]
    pub page_up: String,
    /// Page down in messages (default: pagedown)
    #[serde(default = "default_page_down")]
    pub page_down: String,

    // ==================== View Controls ====================
    /// Collapse all messages (default: cmd-shift-c)
    #[serde(default = "default_collapse_all")]
    pub collapse_all: String,
    /// Expand all messages (default: cmd-shift-e)
    #[serde(default = "default_expand_all")]
    pub expand_all: String,
    /// Toggle timestamps (default: cmd-shift-t)
    #[serde(default = "default_toggle_timestamps")]
    pub toggle_timestamps: String,
    /// Toggle compact mode (default: cmd-shift-m)
    #[serde(default = "default_toggle_compact")]
    pub toggle_compact: String,
    /// Toggle thinking display (default: cmd-shift-k)
    #[serde(default = "default_toggle_thinking")]
    pub toggle_thinking: String,

    // ==================== Search ====================
    /// Find in conversation (default: cmd-f)
    #[serde(default = "default_find")]
    pub find: String,
    /// Find next (default: cmd-g)
    #[serde(default = "default_find_next")]
    pub find_next: String,
    /// Find previous (default: cmd-shift-g)
    #[serde(default = "default_find_prev")]
    pub find_prev: String,

    // ==================== Panel Toggles ====================
    /// Toggle context panel (default: cmd-1)
    #[serde(default = "default_toggle_context")]
    pub toggle_context_panel: String,
    /// Toggle commands panel (default: cmd-2)
    #[serde(default = "default_toggle_commands")]
    pub toggle_commands_panel: String,
    /// Toggle templates panel (default: cmd-3)
    #[serde(default = "default_toggle_templates")]
    pub toggle_templates_panel: String,
    /// Toggle MCP panel (default: cmd-4)
    #[serde(default = "default_toggle_mcp")]
    pub toggle_mcp_panel: String,
    /// Toggle tasks panel (default: cmd-5)
    #[serde(default = "default_toggle_tasks")]
    pub toggle_tasks_panel: String,
    /// Toggle git panel (default: cmd-6)
    #[serde(default = "default_toggle_git")]
    pub toggle_git_panel: String,
    /// Toggle export panel (default: cmd-7)
    #[serde(default = "default_toggle_export")]
    pub toggle_export_panel: String,
    /// Toggle shortcuts help (default: cmd-/)
    #[serde(default = "default_toggle_shortcuts")]
    pub toggle_shortcuts: String,

    // ==================== Accessibility Skip Links ====================
    /// Skip to main content (default: alt-1)
    #[serde(default = "default_skip_main")]
    pub skip_to_main: String,
    /// Skip to input (default: alt-2)
    #[serde(default = "default_skip_input")]
    pub skip_to_input: String,
    /// Skip to sidebar (default: alt-3)
    #[serde(default = "default_skip_sidebar")]
    pub skip_to_sidebar: String,
    /// Skip to toolbar (default: alt-4)
    #[serde(default = "default_skip_toolbar")]
    pub skip_to_toolbar: String,

    // ==================== Session Management ====================
    /// Save session (default: cmd-s)
    #[serde(default = "default_save_session")]
    pub save_session: String,
    /// Clear conversation (default: cmd-shift-backspace)
    #[serde(default = "default_clear_conversation")]
    pub clear_conversation: String,
    /// Toggle session history (default: cmd-h)
    #[serde(default = "default_toggle_history")]
    pub toggle_history: String,
}

// Default value functions for serde
fn default_copy_message() -> String {
    "cmd-c".to_string()
}
fn default_edit_message() -> String {
    "cmd-e".to_string()
}
fn default_delete_message() -> String {
    "cmd-backspace".to_string()
}
fn default_pin_message() -> String {
    "cmd-p".to_string()
}
fn default_bookmark_message() -> String {
    "cmd-d".to_string()
}
fn default_add_reaction() -> String {
    "cmd-r".to_string()
}
fn default_jump_first() -> String {
    "cmd-home".to_string()
}
fn default_jump_last() -> String {
    "cmd-end".to_string()
}
fn default_select_prev() -> String {
    "up".to_string()
}
fn default_select_next() -> String {
    "down".to_string()
}
fn default_page_up() -> String {
    "pageup".to_string()
}
fn default_page_down() -> String {
    "pagedown".to_string()
}
fn default_collapse_all() -> String {
    "cmd-shift-c".to_string()
}
fn default_expand_all() -> String {
    "cmd-shift-e".to_string()
}
fn default_toggle_timestamps() -> String {
    "cmd-shift-t".to_string()
}
fn default_toggle_compact() -> String {
    "cmd-shift-m".to_string()
}
fn default_toggle_thinking() -> String {
    "cmd-shift-k".to_string()
}
fn default_find() -> String {
    "cmd-f".to_string()
}
fn default_find_next() -> String {
    "cmd-g".to_string()
}
fn default_find_prev() -> String {
    "cmd-shift-g".to_string()
}
fn default_toggle_context() -> String {
    "cmd-1".to_string()
}
fn default_toggle_commands() -> String {
    "cmd-2".to_string()
}
fn default_toggle_templates() -> String {
    "cmd-3".to_string()
}
fn default_toggle_mcp() -> String {
    "cmd-4".to_string()
}
fn default_toggle_tasks() -> String {
    "cmd-5".to_string()
}
fn default_toggle_git() -> String {
    "cmd-6".to_string()
}
fn default_toggle_export() -> String {
    "cmd-7".to_string()
}
fn default_toggle_shortcuts() -> String {
    "cmd-/".to_string()
}
fn default_skip_main() -> String {
    "alt-1".to_string()
}
fn default_skip_input() -> String {
    "alt-2".to_string()
}
fn default_skip_sidebar() -> String {
    "alt-3".to_string()
}
fn default_skip_toolbar() -> String {
    "alt-4".to_string()
}
fn default_save_session() -> String {
    "cmd-s".to_string()
}
fn default_clear_conversation() -> String {
    "cmd-shift-backspace".to_string()
}
fn default_toggle_history() -> String {
    "cmd-h".to_string()
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            // Basic Navigation
            toggle_sidebar: "cmd-b".to_string(),
            new_conversation: "cmd-n".to_string(),
            open_settings: "cmd-,".to_string(),
            command_palette: "cmd-k".to_string(),
            next_tab: "ctrl-tab".to_string(),
            prev_tab: "ctrl-shift-tab".to_string(),
            close_tab: "cmd-w".to_string(),
            new_tab: "cmd-t".to_string(),
            send_message: "cmd-enter".to_string(),
            focus_input: "cmd-l".to_string(),
            // Message Operations
            copy_message: default_copy_message(),
            edit_message: default_edit_message(),
            delete_message: default_delete_message(),
            pin_message: default_pin_message(),
            bookmark_message: default_bookmark_message(),
            add_reaction: default_add_reaction(),
            // Message Navigation
            jump_first: default_jump_first(),
            jump_last: default_jump_last(),
            select_prev_message: default_select_prev(),
            select_next_message: default_select_next(),
            page_up: default_page_up(),
            page_down: default_page_down(),
            // View Controls
            collapse_all: default_collapse_all(),
            expand_all: default_expand_all(),
            toggle_timestamps: default_toggle_timestamps(),
            toggle_compact: default_toggle_compact(),
            toggle_thinking: default_toggle_thinking(),
            // Search
            find: default_find(),
            find_next: default_find_next(),
            find_prev: default_find_prev(),
            // Panel Toggles
            toggle_context_panel: default_toggle_context(),
            toggle_commands_panel: default_toggle_commands(),
            toggle_templates_panel: default_toggle_templates(),
            toggle_mcp_panel: default_toggle_mcp(),
            toggle_tasks_panel: default_toggle_tasks(),
            toggle_git_panel: default_toggle_git(),
            toggle_export_panel: default_toggle_export(),
            toggle_shortcuts: default_toggle_shortcuts(),
            // Accessibility
            skip_to_main: default_skip_main(),
            skip_to_input: default_skip_input(),
            skip_to_sidebar: default_skip_sidebar(),
            skip_to_toolbar: default_skip_toolbar(),
            // Session Management
            save_session: default_save_session(),
            clear_conversation: default_clear_conversation(),
            toggle_history: default_toggle_history(),
        }
    }
}

impl Keybindings {
    /// Get all keybindings as (action, keybinding) pairs grouped by category
    pub fn all_bindings(&self) -> Vec<(&'static str, &str)> {
        vec![
            // Basic Navigation
            ("Toggle Sidebar", &self.toggle_sidebar),
            ("New Conversation", &self.new_conversation),
            ("Open Settings", &self.open_settings),
            ("Command Palette", &self.command_palette),
            ("Next Tab", &self.next_tab),
            ("Previous Tab", &self.prev_tab),
            ("Close Tab", &self.close_tab),
            ("New Tab", &self.new_tab),
            ("Send Message", &self.send_message),
            ("Focus Input", &self.focus_input),
            // Message Operations
            ("Copy Message", &self.copy_message),
            ("Edit Message", &self.edit_message),
            ("Delete Message", &self.delete_message),
            ("Pin Message", &self.pin_message),
            ("Bookmark Message", &self.bookmark_message),
            ("Add Reaction", &self.add_reaction),
            // Message Navigation
            ("Jump to First", &self.jump_first),
            ("Jump to Last", &self.jump_last),
            ("Select Previous", &self.select_prev_message),
            ("Select Next", &self.select_next_message),
            ("Page Up", &self.page_up),
            ("Page Down", &self.page_down),
            // View Controls
            ("Collapse All", &self.collapse_all),
            ("Expand All", &self.expand_all),
            ("Toggle Timestamps", &self.toggle_timestamps),
            ("Toggle Compact", &self.toggle_compact),
            ("Toggle Thinking", &self.toggle_thinking),
            // Search
            ("Find", &self.find),
            ("Find Next", &self.find_next),
            ("Find Previous", &self.find_prev),
            // Panel Toggles
            ("Context Panel", &self.toggle_context_panel),
            ("Commands Panel", &self.toggle_commands_panel),
            ("Templates Panel", &self.toggle_templates_panel),
            ("MCP Panel", &self.toggle_mcp_panel),
            ("Tasks Panel", &self.toggle_tasks_panel),
            ("Git Panel", &self.toggle_git_panel),
            ("Export Panel", &self.toggle_export_panel),
            ("Shortcuts Help", &self.toggle_shortcuts),
            // Accessibility
            ("Skip to Main", &self.skip_to_main),
            ("Skip to Input", &self.skip_to_input),
            ("Skip to Sidebar", &self.skip_to_sidebar),
            ("Skip to Toolbar", &self.skip_to_toolbar),
            // Session Management
            ("Save Session", &self.save_session),
            ("Clear Conversation", &self.clear_conversation),
            ("Session History", &self.toggle_history),
        ]
    }

    /// Get bindings grouped by category for display
    pub fn bindings_by_category(&self) -> Vec<(&'static str, Vec<(&'static str, &str)>)> {
        vec![
            (
                "Navigation",
                vec![
                    ("Toggle Sidebar", &self.toggle_sidebar),
                    ("New Conversation", &self.new_conversation),
                    ("Command Palette", &self.command_palette),
                    ("Focus Input", &self.focus_input),
                    ("Next Tab", &self.next_tab),
                    ("Previous Tab", &self.prev_tab),
                ],
            ),
            (
                "Messages",
                vec![
                    ("Copy Message", &self.copy_message),
                    ("Edit Message", &self.edit_message),
                    ("Delete Message", &self.delete_message),
                    ("Pin Message", &self.pin_message),
                    ("Bookmark Message", &self.bookmark_message),
                    ("Add Reaction", &self.add_reaction),
                ],
            ),
            (
                "Message Navigation",
                vec![
                    ("Jump to First", &self.jump_first),
                    ("Jump to Last", &self.jump_last),
                    ("Select Previous", &self.select_prev_message),
                    ("Select Next", &self.select_next_message),
                    ("Page Up", &self.page_up),
                    ("Page Down", &self.page_down),
                ],
            ),
            (
                "View",
                vec![
                    ("Collapse All", &self.collapse_all),
                    ("Expand All", &self.expand_all),
                    ("Toggle Timestamps", &self.toggle_timestamps),
                    ("Toggle Compact", &self.toggle_compact),
                    ("Toggle Thinking", &self.toggle_thinking),
                ],
            ),
            (
                "Search",
                vec![
                    ("Find", &self.find),
                    ("Find Next", &self.find_next),
                    ("Find Previous", &self.find_prev),
                ],
            ),
            (
                "Panels",
                vec![
                    ("Context Panel", &self.toggle_context_panel),
                    ("Commands Panel", &self.toggle_commands_panel),
                    ("Templates Panel", &self.toggle_templates_panel),
                    ("MCP Panel", &self.toggle_mcp_panel),
                    ("Tasks Panel", &self.toggle_tasks_panel),
                    ("Git Panel", &self.toggle_git_panel),
                    ("Export Panel", &self.toggle_export_panel),
                    ("Shortcuts Help", &self.toggle_shortcuts),
                ],
            ),
            (
                "Accessibility",
                vec![
                    ("Skip to Main", &self.skip_to_main),
                    ("Skip to Input", &self.skip_to_input),
                    ("Skip to Sidebar", &self.skip_to_sidebar),
                    ("Skip to Toolbar", &self.skip_to_toolbar),
                ],
            ),
            (
                "Session",
                vec![
                    ("Save Session", &self.save_session),
                    ("Clear Conversation", &self.clear_conversation),
                    ("Session History", &self.toggle_history),
                ],
            ),
        ]
    }

    /// Update a keybinding by action name
    pub fn set_binding(&mut self, action: &str, keybinding: String) {
        match action {
            // Basic Navigation
            "Toggle Sidebar" => self.toggle_sidebar = keybinding,
            "New Conversation" => self.new_conversation = keybinding,
            "Open Settings" => self.open_settings = keybinding,
            "Command Palette" => self.command_palette = keybinding,
            "Next Tab" => self.next_tab = keybinding,
            "Previous Tab" => self.prev_tab = keybinding,
            "Close Tab" => self.close_tab = keybinding,
            "New Tab" => self.new_tab = keybinding,
            "Send Message" => self.send_message = keybinding,
            "Focus Input" => self.focus_input = keybinding,
            // Message Operations
            "Copy Message" => self.copy_message = keybinding,
            "Edit Message" => self.edit_message = keybinding,
            "Delete Message" => self.delete_message = keybinding,
            "Pin Message" => self.pin_message = keybinding,
            "Bookmark Message" => self.bookmark_message = keybinding,
            "Add Reaction" => self.add_reaction = keybinding,
            // Message Navigation
            "Jump to First" => self.jump_first = keybinding,
            "Jump to Last" => self.jump_last = keybinding,
            "Select Previous" => self.select_prev_message = keybinding,
            "Select Next" => self.select_next_message = keybinding,
            "Page Up" => self.page_up = keybinding,
            "Page Down" => self.page_down = keybinding,
            // View Controls
            "Collapse All" => self.collapse_all = keybinding,
            "Expand All" => self.expand_all = keybinding,
            "Toggle Timestamps" => self.toggle_timestamps = keybinding,
            "Toggle Compact" => self.toggle_compact = keybinding,
            "Toggle Thinking" => self.toggle_thinking = keybinding,
            // Search
            "Find" => self.find = keybinding,
            "Find Next" => self.find_next = keybinding,
            "Find Previous" => self.find_prev = keybinding,
            // Panel Toggles
            "Context Panel" => self.toggle_context_panel = keybinding,
            "Commands Panel" => self.toggle_commands_panel = keybinding,
            "Templates Panel" => self.toggle_templates_panel = keybinding,
            "MCP Panel" => self.toggle_mcp_panel = keybinding,
            "Tasks Panel" => self.toggle_tasks_panel = keybinding,
            "Git Panel" => self.toggle_git_panel = keybinding,
            "Export Panel" => self.toggle_export_panel = keybinding,
            "Shortcuts Help" => self.toggle_shortcuts = keybinding,
            // Accessibility
            "Skip to Main" => self.skip_to_main = keybinding,
            "Skip to Input" => self.skip_to_input = keybinding,
            "Skip to Sidebar" => self.skip_to_sidebar = keybinding,
            "Skip to Toolbar" => self.skip_to_toolbar = keybinding,
            // Session Management
            "Save Session" => self.save_session = keybinding,
            "Clear Conversation" => self.clear_conversation = keybinding,
            "Session History" => self.toggle_history = keybinding,
            _ => {}
        }
    }
}
