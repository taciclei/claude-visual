use gpui::*;

/// Simple colors struct for default theme
pub(crate) struct SimpleColors {
    pub text: Hsla,
    pub text_muted: Hsla,
    pub background: Hsla,
    pub surface: Hsla,
    pub surface_hover: Hsla,
    pub border: Hsla,
    pub accent: Hsla,
    pub success: Hsla,
    pub warning: Hsla,
    pub error: Hsla,
}

pub(crate) fn default_colors() -> SimpleColors {
    SimpleColors {
        text: hsla(0.0, 0.0, 0.93, 1.0),
        text_muted: hsla(0.0, 0.0, 0.6, 1.0),
        background: hsla(0.0, 0.0, 0.07, 1.0),
        surface: hsla(0.0, 0.0, 0.12, 1.0),
        surface_hover: hsla(0.0, 0.0, 0.16, 1.0),
        border: hsla(0.0, 0.0, 0.2, 1.0),
        accent: hsla(0.6, 0.7, 0.5, 1.0),
        success: hsla(0.35, 0.7, 0.45, 1.0),
        warning: hsla(0.1, 0.8, 0.55, 1.0),
        error: hsla(0.0, 0.7, 0.5, 1.0),
    }
}

/// Settings tab selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsTab {
    Appearance,
    Editor,
    Keybindings,
    Git,
    Claude,
    Mcp,
}

impl SettingsTab {
    pub(crate) fn label(&self) -> &'static str {
        match self {
            SettingsTab::Appearance => "Appearance",
            SettingsTab::Editor => "Editor",
            SettingsTab::Keybindings => "Keybindings",
            SettingsTab::Git => "Git",
            SettingsTab::Claude => "Claude",
            SettingsTab::Mcp => "MCP Servers",
        }
    }

    pub(crate) fn all() -> &'static [SettingsTab] {
        &[
            SettingsTab::Appearance,
            SettingsTab::Editor,
            SettingsTab::Keybindings,
            SettingsTab::Git,
            SettingsTab::Claude,
            SettingsTab::Mcp,
        ]
    }
}

/// Event emitted by SettingsModal
#[derive(Debug, Clone)]
pub enum SettingsModalEvent {
    /// User dismissed the modal
    Dismissed,
    /// Settings were saved
    Saved,
}
