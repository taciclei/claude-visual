//! Global application state management

use std::path::PathBuf;
use std::sync::Arc;

use gpui::{App, AppContext, Entity};
use parking_lot::RwLock;

use crate::plugins::icons::IconLoader;
use crate::plugins::themes::ThemeLoader;
use crate::project::manager::ProjectManager;
use crate::storage::database::Database;

use super::settings::Settings;
use super::theme::Theme;

/// Global application state
pub struct AppState {
    /// User settings
    pub settings: Entity<Settings>,
    /// Current theme
    pub theme: Entity<Theme>,
    /// Project manager
    pub project_manager: Entity<ProjectManager>,
    /// Database connection
    pub database: Arc<Database>,
    /// Current working directory
    pub current_directory: Arc<RwLock<Option<PathBuf>>>,
    /// Theme loader for extension themes
    pub theme_loader: Arc<RwLock<ThemeLoader>>,
    /// Icon theme loader for extension icon themes
    pub icon_loader: Arc<RwLock<IconLoader>>,
}

impl AppState {
    /// Create new application state
    pub fn new(cx: &mut App) -> Arc<Self> {
        // Initialize database
        let database = Database::open().expect("Failed to open database");
        database.initialize().expect("Failed to initialize database");
        let database = Arc::new(database);

        // Load settings
        let settings = cx.new(|_| Settings::load().unwrap_or_default());

        // Load theme
        let theme = cx.new(|_| Theme::default());

        // Create project manager
        let project_manager = cx.new(|_| ProjectManager::new(database.clone()));

        // Initialize theme loader
        let theme_loader = Arc::new(RwLock::new(ThemeLoader::new()));

        // Initialize icon loader
        let icon_loader = Arc::new(RwLock::new(IconLoader::new()));

        Arc::new(Self {
            settings,
            theme,
            project_manager,
            database,
            current_directory: Arc::new(RwLock::new(None)),
            theme_loader,
            icon_loader,
        })
    }

    /// Get the theme loader
    pub fn theme_loader(&self) -> Arc<RwLock<ThemeLoader>> {
        Arc::clone(&self.theme_loader)
    }

    /// Load themes from an extension
    pub fn load_extension_themes(&self, extension_path: &std::path::Path) -> anyhow::Result<Vec<String>> {
        self.theme_loader.write().load_extension(extension_path)
    }

    /// Get a theme by name (from loader or built-in)
    pub fn get_theme(&self, name: &str) -> Option<Theme> {
        self.theme_loader.read().get(name).cloned()
    }

    /// List all available themes (built-in + extension)
    pub fn list_all_themes(&self) -> Vec<String> {
        let mut themes = vec![
            "Dark".to_string(),
            "Light".to_string(),
            "High Contrast Dark".to_string(),
            "High Contrast Light".to_string(),
        ];
        themes.extend(self.theme_loader.read().list().iter().map(|s| s.to_string()));
        themes
    }

    /// Set the current working directory
    pub fn set_current_directory(&self, path: Option<PathBuf>) {
        *self.current_directory.write() = path;
    }

    /// Get the current working directory
    pub fn current_directory(&self) -> Option<PathBuf> {
        self.current_directory.read().clone()
    }

    /// Get the icon loader
    pub fn icon_loader(&self) -> Arc<RwLock<IconLoader>> {
        Arc::clone(&self.icon_loader)
    }

    /// Load icon themes from an extension
    pub fn load_extension_icon_themes(
        &self,
        extension_path: &std::path::Path,
    ) -> anyhow::Result<Vec<String>> {
        self.icon_loader.write().load_extension(extension_path)
    }

    /// Get the current icon theme
    pub fn current_icon_theme(&self) -> Option<crate::plugins::icons::IconTheme> {
        self.icon_loader.read().current().cloned()
    }

    /// Set the current icon theme
    pub fn set_icon_theme(&self, id: &str) -> bool {
        self.icon_loader.write().set_current(id)
    }

    /// List all available icon themes
    pub fn list_icon_themes(&self) -> Vec<String> {
        let mut themes = vec!["Default".to_string()];
        themes.extend(
            self.icon_loader
                .read()
                .list()
                .iter()
                .map(|s| s.to_string()),
        );
        themes
    }
}
