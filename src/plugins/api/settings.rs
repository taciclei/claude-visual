//! Settings API for extensions to store configuration

use anyhow::{Context, Result};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

use super::types::ApiResult;

/// Settings API for extensions to store configuration
pub struct SettingsApi {
    /// Extension settings storage
    settings: Arc<RwLock<HashMap<String, HashMap<String, String>>>>,
}

impl Default for SettingsApi {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsApi {
    /// Create a new settings API instance
    pub fn new() -> Self {
        Self {
            settings: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get a setting value
    pub fn get(&self, extension_id: &str, key: &str) -> ApiResult {
        let settings = self.settings.read();
        if let Some(ext_settings) = settings.get(extension_id) {
            if let Some(value) = ext_settings.get(key) {
                return ApiResult::data(value.clone());
            }
        }
        ApiResult::error("Setting not found")
    }

    /// Set a setting value
    pub fn set(&self, extension_id: &str, key: &str, value: &str) -> ApiResult {
        let mut settings = self.settings.write();
        settings
            .entry(extension_id.to_string())
            .or_default()
            .insert(key.to_string(), value.to_string());
        ApiResult::ok()
    }

    /// Delete a setting
    pub fn delete(&self, extension_id: &str, key: &str) -> ApiResult {
        let mut settings = self.settings.write();
        if let Some(ext_settings) = settings.get_mut(extension_id) {
            ext_settings.remove(key);
        }
        ApiResult::ok()
    }

    /// Get all settings for an extension
    pub fn get_all(&self, extension_id: &str) -> ApiResult {
        let settings = self.settings.read();
        if let Some(ext_settings) = settings.get(extension_id) {
            let json = serde_json::to_string(ext_settings).unwrap_or_default();
            return ApiResult::data(json);
        }
        ApiResult::data("{}".to_string())
    }

    /// Load settings from disk
    pub fn load_from_disk(&self, extension_id: &str) -> Result<()> {
        let settings_dir = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find data directory"))?
            .join("claude-visual")
            .join("extension-settings");

        let settings_path = settings_dir.join(format!("{}.json", extension_id));

        if settings_path.exists() {
            let content =
                std::fs::read_to_string(&settings_path).context("Failed to read settings file")?;
            let ext_settings: HashMap<String, String> =
                serde_json::from_str(&content).context("Failed to parse settings file")?;

            self.settings
                .write()
                .insert(extension_id.to_string(), ext_settings);
        }

        Ok(())
    }

    /// Save settings to disk
    pub fn save_to_disk(&self, extension_id: &str) -> Result<()> {
        let settings_dir = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find data directory"))?
            .join("claude-visual")
            .join("extension-settings");

        std::fs::create_dir_all(&settings_dir)?;

        let settings_path = settings_dir.join(format!("{}.json", extension_id));
        let settings = self.settings.read();

        if let Some(ext_settings) = settings.get(extension_id) {
            let content = serde_json::to_string_pretty(ext_settings)?;
            std::fs::write(&settings_path, content)?;
        }

        Ok(())
    }
}
