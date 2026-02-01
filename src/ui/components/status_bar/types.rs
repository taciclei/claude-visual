//! Type definitions for status bar components

/// Status bar item types
#[derive(Debug, Clone)]
pub enum StatusItem {
    /// Text item
    Text(String),
    /// Separator
    Separator,
    /// Icon with text
    IconText { icon: &'static str, text: String },
    /// Clickable item
    Clickable { text: String, action: &'static str },
}

/// Status bar configuration
pub struct StatusBarConfig {
    /// Items on the left side
    pub left: Vec<StatusItem>,
    /// Items in the center
    pub center: Vec<StatusItem>,
    /// Items on the right side
    pub right: Vec<StatusItem>,
}

impl Default for StatusBarConfig {
    fn default() -> Self {
        Self {
            left: Vec::new(),
            center: Vec::new(),
            right: Vec::new(),
        }
    }
}

/// Events emitted by StatusBar
pub enum StatusBarEvent {
    /// An item was clicked
    ItemClicked(String),
    /// Request to open model switcher
    OpenModelSwitcher,
    /// Request to stop streaming
    StopStreaming,
    /// Request to toggle vim mode
    ToggleVimMode,
    /// Open MCP panel
    OpenMcpPanel,
    /// Open context panel
    OpenContextPanel,
    /// Open memory panel
    OpenMemoryPanel,
    /// Send a Claude Code skill command
    SendSkillCommand(String),
}
