//! Command palette types

/// A command in the command palette
#[derive(Debug, Clone)]
pub struct PaletteCommand {
    /// Command ID
    pub id: &'static str,
    /// Display label
    pub label: &'static str,
    /// Description
    pub description: &'static str,
    /// Keyboard shortcut (for display)
    pub shortcut: Option<&'static str>,
    /// Category for grouping
    pub category: &'static str,
    /// Icon/emoji
    pub icon: &'static str,
}

/// Category filter for commands panel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CommandCategory {
    #[default]
    All,
    SlashCommands,
    Skills,
    Actions,
}

impl CommandCategory {
    pub fn label(&self) -> &'static str {
        match self {
            CommandCategory::All => "All",
            CommandCategory::SlashCommands => "Commands",
            CommandCategory::Skills => "Skills",
            CommandCategory::Actions => "Actions",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            CommandCategory::All => "📋",
            CommandCategory::SlashCommands => "/",
            CommandCategory::Skills => "⚡",
            CommandCategory::Actions => "🎯",
        }
    }
}
