//! User settings management

mod editor;
mod keybindings;
mod language;
mod ui;

pub use editor::EditorSettings;
pub use keybindings::Keybindings;
pub use language::LanguageSetting;
pub use ui::UISettings;

use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    /// Theme name (dark, light, or custom)
    #[serde(flatten)]
    pub ui: UISettings,
    /// Editor settings
    #[serde(flatten)]
    pub editor: EditorSettings,
    /// Language/locale setting
    #[serde(default)]
    pub language: LanguageSetting,
    /// Auto-save conversations
    pub auto_save_conversations: bool,
    /// Claude CLI path (if not in PATH)
    pub claude_cli_path: Option<PathBuf>,
    /// Default project directory
    pub default_project_dir: Option<PathBuf>,
    /// Custom keybindings
    #[serde(default)]
    pub keybindings: Keybindings,
    /// Auto-saved draft text (restored on restart)
    #[serde(default)]
    pub draft_text: String,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            ui: UISettings::default(),
            editor: EditorSettings::default(),
            language: LanguageSetting::Auto,
            auto_save_conversations: true,
            claude_cli_path: None,
            default_project_dir: None,
            keybindings: Keybindings::default(),
            draft_text: String::new(),
        }
    }
}

impl UserSettings {
    /// Get the settings file path
    pub fn config_path() -> Result<PathBuf> {
        let config_dir =
            dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        Ok(config_dir.join("claude-visual").join("settings.toml"))
    }

    /// Load settings from disk
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            let settings: UserSettings = toml::from_str(&content)?;
            Ok(settings)
        } else {
            Ok(Self::default())
        }
    }

    /// Save settings to disk
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    /// Export settings to JSON string
    pub fn export_json(&self) -> Result<String> {
        let json = serde_json::to_string_pretty(self)?;
        Ok(json)
    }

    /// Export settings to a file path
    pub fn export_to_file(&self, path: &std::path::Path) -> Result<()> {
        let json = self.export_json()?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Import settings from JSON string
    pub fn import_json(json: &str) -> Result<Self> {
        let settings: UserSettings = serde_json::from_str(json)?;
        Ok(settings)
    }

    /// Import settings from a file path
    pub fn import_from_file(path: &std::path::Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::import_json(&content)
    }

    /// Get the default export filename with timestamp
    pub fn default_export_filename() -> String {
        let now = chrono::Local::now();
        format!(
            "claude-visual-settings-{}.json",
            now.format("%Y%m%d-%H%M%S")
        )
    }

    /// Increase UI font size
    pub fn increase_ui_font_size(&mut self) {
        self.ui.increase_ui_font_size();
    }

    /// Decrease UI font size
    pub fn decrease_ui_font_size(&mut self) {
        self.ui.decrease_ui_font_size();
    }

    /// Increase code font size
    pub fn increase_code_font_size(&mut self) {
        self.editor.increase_code_font_size();
    }

    /// Decrease code font size
    pub fn decrease_code_font_size(&mut self) {
        self.editor.decrease_code_font_size();
    }

    /// Increase both UI and code font sizes
    pub fn increase_font_size(&mut self) {
        self.increase_ui_font_size();
        self.increase_code_font_size();
    }

    /// Decrease both UI and code font sizes
    pub fn decrease_font_size(&mut self) {
        self.decrease_ui_font_size();
        self.decrease_code_font_size();
    }

    /// Reset font sizes to default
    pub fn reset_font_size(&mut self) {
        self.ui.reset_ui_font_size();
        self.editor.reset_code_font_size();
    }

    // ============================================
    // Accessor methods for backward compatibility
    // ============================================

    // UI Settings accessors
    /// Get current theme name
    pub fn theme(&self) -> &str {
        &self.ui.theme
    }

    /// Set theme name
    pub fn set_theme(&mut self, theme: String) {
        self.ui.theme = theme;
    }

    /// Get UI font family
    pub fn ui_font_family(&self) -> &str {
        &self.ui.ui_font_family
    }

    /// Set UI font family
    pub fn set_ui_font_family(&mut self, family: String) {
        self.ui.ui_font_family = family;
    }

    /// Get UI font size
    pub fn ui_font_size(&self) -> f32 {
        self.ui.ui_font_size
    }

    /// Set UI font size
    pub fn set_ui_font_size(&mut self, size: f32) {
        self.ui.ui_font_size = size;
    }

    /// Get show sidebar setting
    pub fn show_sidebar(&self) -> bool {
        self.ui.show_sidebar
    }

    /// Set show sidebar setting
    pub fn set_show_sidebar(&mut self, show: bool) {
        self.ui.show_sidebar = show;
    }

    /// Get sidebar width
    pub fn sidebar_width(&self) -> f32 {
        self.ui.sidebar_width
    }

    /// Set sidebar width
    pub fn set_sidebar_width(&mut self, width: f32) {
        self.ui.sidebar_width = width;
    }

    // Editor Settings accessors
    /// Get code font family
    pub fn code_font_family(&self) -> &str {
        &self.editor.code_font_family
    }

    /// Set code font family
    pub fn set_code_font_family(&mut self, family: String) {
        self.editor.code_font_family = family;
    }

    /// Get code font size
    pub fn code_font_size(&self) -> f32 {
        self.editor.code_font_size
    }

    /// Set code font size
    pub fn set_code_font_size(&mut self, size: f32) {
        self.editor.code_font_size = size;
    }

    /// Get vim mode setting
    pub fn vim_mode(&self) -> bool {
        self.editor.vim_mode
    }

    /// Set vim mode setting
    pub fn set_vim_mode(&mut self, enabled: bool) {
        self.editor.vim_mode = enabled;
    }
}

// Backward compatibility alias
pub type Settings = UserSettings;
