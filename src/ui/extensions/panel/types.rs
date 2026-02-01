use crate::plugins::ExtensionManifest;

/// Tab selection for extensions panel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExtensionsTab {
    /// Installed extensions
    #[default]
    Installed,
    /// Available extensions (marketplace)
    Available,
    /// Extension updates
    Updates,
}

impl ExtensionsTab {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Installed => "Installed",
            Self::Available => "Available",
            Self::Updates => "Updates",
        }
    }

    pub fn all() -> &'static [Self] {
        &[Self::Installed, Self::Available, Self::Updates]
    }
}

/// Event emitted by ExtensionsPanel
#[derive(Debug, Clone)]
pub enum ExtensionsPanelEvent {
    /// User dismissed the panel
    Dismissed,
    /// Extension was enabled
    ExtensionEnabled(String),
    /// Extension was disabled
    ExtensionDisabled(String),
    /// Extension was uninstalled
    ExtensionUninstalled(String),
}

/// Extension display item
#[derive(Debug, Clone)]
pub struct ExtensionItem {
    /// Extension manifest
    pub manifest: ExtensionManifest,
    /// Whether the extension is enabled
    pub enabled: bool,
    /// Whether an update is available
    pub has_update: bool,
}
