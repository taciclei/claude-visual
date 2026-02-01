//! Main theme loader implementation

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use crate::app::theme::Theme;

use super::discovery::discover_themes_sync;
use super::io::load_theme_from_path;
use super::types::{PreloadResult, ThemeLoadError, ThemeLoadResult, ThemeLoadState, ThemeMetadata};

/// Async theme loader
pub struct ThemeLoader {
    /// Directory to scan for themes
    themes_dir: PathBuf,
    /// Cache of loaded themes
    cache: RwLock<HashMap<String, Arc<Theme>>>,
    /// Theme metadata index
    index: RwLock<Vec<ThemeMetadata>>,
    /// Current loading state
    state: RwLock<ThemeLoadState>,
    /// Default theme to use during loading
    default_theme: Theme,
}

impl ThemeLoader {
    /// Create a new theme loader
    pub fn new(themes_dir: PathBuf) -> Self {
        Self {
            themes_dir,
            cache: RwLock::new(HashMap::new()),
            index: RwLock::new(Vec::new()),
            state: RwLock::new(ThemeLoadState::NotStarted),
            default_theme: Theme::dark(),
        }
    }

    /// Create with a custom default theme
    pub fn with_default(themes_dir: PathBuf, default_theme: Theme) -> Self {
        Self {
            themes_dir,
            cache: RwLock::new(HashMap::new()),
            index: RwLock::new(Vec::new()),
            state: RwLock::new(ThemeLoadState::NotStarted),
            default_theme,
        }
    }

    /// Get the themes directory
    pub fn themes_dir(&self) -> &Path {
        &self.themes_dir
    }

    /// Get current loading state
    pub fn state(&self) -> ThemeLoadState {
        *self.state.read().unwrap()
    }

    /// Get the default theme (used during loading)
    pub fn default_theme(&self) -> &Theme {
        &self.default_theme
    }

    /// Get available theme metadata (without full loading)
    pub fn available_themes(&self) -> Vec<ThemeMetadata> {
        self.index.read().unwrap().clone()
    }

    /// Start async theme discovery
    pub async fn discover_themes(&self) -> Result<Vec<ThemeMetadata>, ThemeLoadError> {
        *self.state.write().unwrap() = ThemeLoadState::Loading;

        let themes_dir = self.themes_dir.clone();

        // Run discovery in blocking task
        let result = tokio::task::spawn_blocking(move || {
            discover_themes_sync(&themes_dir)
        })
        .await
        .map_err(|e| ThemeLoadError::ReadError(e.to_string()))?;

        match &result {
            Ok(themes) => {
                let mut index = self.index.write().unwrap();
                *index = themes.clone();
                *self.state.write().unwrap() = ThemeLoadState::Loaded;
            }
            Err(_) => {
                *self.state.write().unwrap() = ThemeLoadState::Failed;
            }
        }

        result
    }

    /// Load a theme by name asynchronously
    pub async fn load_theme(&self, name: &str) -> ThemeLoadResult {
        // Check cache first
        {
            let cache = self.cache.read().unwrap();
            if let Some(theme) = cache.get(name) {
                return Ok((**theme).clone());
            }
        }

        // Find theme path
        let path = {
            let index = self.index.read().unwrap();
            index
                .iter()
                .find(|t| t.name == name)
                .map(|t| t.path.clone())
                .ok_or_else(|| ThemeLoadError::NotFound(name.to_string()))?
        };

        // Load from file
        let theme = load_theme_from_path(&path).await?;

        // Cache the loaded theme
        {
            let mut cache = self.cache.write().unwrap();
            cache.insert(name.to_string(), Arc::new(theme.clone()));
        }

        Ok(theme)
    }

    /// Load a theme from a specific path
    pub async fn load_from_path(&self, path: &Path) -> ThemeLoadResult {
        load_theme_from_path(path).await
    }

    /// Preload all themes into cache
    pub async fn preload_all(&self) -> PreloadResult {
        let themes = self.available_themes();
        let mut loaded = 0;
        let mut errors = Vec::new();

        for meta in themes {
            match self.load_theme(&meta.name).await {
                Ok(_) => loaded += 1,
                Err(e) => errors.push((meta.name.clone(), e)),
            }
        }

        PreloadResult { loaded, errors }
    }

    /// Get a cached theme (returns default if not loaded)
    pub fn get_cached(&self, name: &str) -> Arc<Theme> {
        let cache = self.cache.read().unwrap();
        cache
            .get(name)
            .cloned()
            .unwrap_or_else(|| Arc::new(self.default_theme.clone()))
    }

    /// Check if a theme is cached
    pub fn is_cached(&self, name: &str) -> bool {
        let cache = self.cache.read().unwrap();
        cache.contains_key(name)
    }

    /// Clear the theme cache
    pub fn clear_cache(&self) {
        let mut cache = self.cache.write().unwrap();
        cache.clear();
    }

    /// Get cache size
    pub fn cache_size(&self) -> usize {
        let cache = self.cache.read().unwrap();
        cache.len()
    }
}
