//! Input completion manager implementation

use std::collections::HashMap;
use std::path::PathBuf;

use super::types::*;

/// Chat input completion manager
pub struct InputCompletionManager {
    /// Configuration
    pub(crate) config: CompletionConfig,
    /// Current state
    pub(crate) state: CompletionState,
    /// Current trigger
    pub(crate) trigger: Option<CompletionTrigger>,
    /// Trigger position in text
    pub(crate) trigger_position: Option<usize>,
    /// Current query
    pub(crate) query: String,
    /// Completion items
    pub(crate) items: Vec<ChatCompletionItem>,
    /// Selected index
    pub(crate) selected_index: usize,
    /// Available commands
    pub(crate) commands: HashMap<String, String>,
    /// Recent files for completion
    pub(crate) recent_files: Vec<PathBuf>,
}

impl Default for InputCompletionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl InputCompletionManager {
    /// Create new completion manager
    pub fn new() -> Self {
        Self {
            config: CompletionConfig::default(),
            state: CompletionState::Inactive,
            trigger: None,
            trigger_position: None,
            query: String::new(),
            items: Vec::new(),
            selected_index: 0,
            commands: Self::default_commands(),
            recent_files: Vec::new(),
        }
    }

    /// Create with custom config
    pub fn with_config(config: CompletionConfig) -> Self {
        Self {
            config,
            ..Self::new()
        }
    }

    /// Get default commands
    fn default_commands() -> HashMap<String, String> {
        let mut commands = HashMap::new();
        commands.insert("help".to_string(), "Show help information".to_string());
        commands.insert("clear".to_string(), "Clear conversation".to_string());
        commands.insert("save".to_string(), "Save conversation to file".to_string());
        commands.insert("export".to_string(), "Export conversation".to_string());
        commands.insert("model".to_string(), "Change AI model".to_string());
        commands.insert("theme".to_string(), "Change color theme".to_string());
        commands.insert("settings".to_string(), "Open settings".to_string());
        commands
    }

    /// Register a command
    pub fn register_command(&mut self, name: impl Into<String>, description: impl Into<String>) {
        self.commands.insert(name.into(), description.into());
    }

    /// Add recent file
    pub fn add_recent_file(&mut self, path: PathBuf) {
        // Remove if already exists
        self.recent_files.retain(|p| p != &path);
        // Add to front
        self.recent_files.insert(0, path);
        // Keep limited size
        self.recent_files.truncate(50);
    }

    /// Get state
    pub fn state(&self) -> CompletionState {
        self.state
    }

    /// Check if completion is active
    pub fn is_active(&self) -> bool {
        self.state == CompletionState::Active
    }

    /// Get current items
    pub fn items(&self) -> &[ChatCompletionItem] {
        &self.items
    }

    /// Get selected index
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    /// Get selected item
    pub fn selected_item(&self) -> Option<&ChatCompletionItem> {
        self.items.get(self.selected_index)
    }
}
