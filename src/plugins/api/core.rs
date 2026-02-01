//! Core extension API that combines all sub-APIs

use super::ui::UiApi;
use super::fs::FileSystemApi;
use super::settings::SettingsApi;
use super::events::EventApi;

/// Version of the extension API
pub const API_VERSION: &str = "0.1.0";

/// Main extension API that combines all sub-APIs
pub struct ExtensionApi {
    /// UI API
    pub ui: UiApi,
    /// File system API
    pub fs: FileSystemApi,
    /// Settings API
    pub settings: SettingsApi,
    /// Event API
    pub events: EventApi,
}

impl Default for ExtensionApi {
    fn default() -> Self {
        Self::new()
    }
}

impl ExtensionApi {
    /// Create a new extension API instance
    pub fn new() -> Self {
        Self {
            ui: UiApi::new(),
            fs: FileSystemApi::new(),
            settings: SettingsApi::new(),
            events: EventApi::new(),
        }
    }

    /// Clean up resources for an extension
    pub fn cleanup_extension(&self, extension_id: &str) {
        // Remove UI elements
        self.ui.status_items.write().retain(|_, item| item.extension_id != extension_id);
        self.ui.notifications.write().retain(|n| n.extension_id != extension_id);

        // Remove event subscriptions
        self.events.remove_extension_subscriptions(extension_id);
    }
}
