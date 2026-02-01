//! Theme loader for Zed-compatible themes

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;

use crate::app::theme::Theme;
use crate::plugins::ExtensionManifest;
use super::types::{ThemeMetadata, ZedThemeFile};
use super::converter::convert_variant;

/// Theme loader for Zed-compatible themes
pub struct ThemeLoader {
    /// Loaded themes (name -> Theme)
    pub(crate) themes: HashMap<String, Theme>,
    /// Theme metadata (name -> metadata)
    pub(crate) metadata: HashMap<String, ThemeMetadata>,
}

impl ThemeLoader {
    /// Create a new theme loader
    pub fn new() -> Self {
        Self {
            themes: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    /// Load themes from an extension directory
    pub fn load_extension(&mut self, extension_path: &Path) -> Result<Vec<String>> {
        let manifest_path = extension_path.join("extension.toml");
        let manifest_content = std::fs::read_to_string(&manifest_path)
            .context("Failed to read extension.toml")?;
        let manifest: ExtensionManifest = toml::from_str(&manifest_content)
            .context("Failed to parse extension.toml")?;

        let mut loaded_names = Vec::new();

        if let Some(theme_configs) = &manifest.themes {
            for theme_config in theme_configs {
                let theme_path = extension_path.join(&theme_config.path);
                if theme_path.exists() {
                    match self.load_file_with_metadata(&theme_path, Some(&manifest.id)) {
                        Ok(names) => loaded_names.extend(names),
                        Err(e) => {
                            tracing::warn!(
                                "Failed to load theme from {}: {}",
                                theme_path.display(),
                                e
                            );
                        }
                    }
                }
            }
        }

        Ok(loaded_names)
    }

    /// Load a theme from a JSON file with metadata
    pub fn load_file_with_metadata(
        &mut self,
        path: &Path,
        extension_id: Option<&str>,
    ) -> Result<Vec<String>> {
        let content = std::fs::read_to_string(path)
            .context("Failed to read theme file")?;

        self.load_json_with_metadata(&content, extension_id, Some(path.to_path_buf()))
    }

    /// Load a theme from a JSON file
    pub fn load_file(&mut self, path: &Path) -> Result<Vec<String>> {
        self.load_file_with_metadata(path, None)
    }

    /// Load a theme from JSON string
    pub fn load_json(&mut self, json: &str) -> Result<Vec<String>> {
        self.load_json_with_metadata(json, None, None)
    }

    /// Load a theme from JSON string with metadata
    pub fn load_json_with_metadata(
        &mut self,
        json: &str,
        extension_id: Option<&str>,
        file_path: Option<std::path::PathBuf>,
    ) -> Result<Vec<String>> {
        let theme_file: ZedThemeFile = serde_json::from_str(json)
            .context("Failed to parse theme JSON")?;

        let mut loaded_names = Vec::new();

        for variant in theme_file.themes {
            let theme = convert_variant(&theme_file.name, &variant)?;
            let full_name = format!("{} - {}", theme_file.name, variant.name);

            // Store metadata
            self.metadata.insert(
                full_name.clone(),
                ThemeMetadata {
                    name: full_name.clone(),
                    extension_id: extension_id.map(String::from),
                    author: theme_file.author.clone(),
                    path: file_path.clone(),
                },
            );

            self.themes.insert(full_name.clone(), theme);
            loaded_names.push(full_name);
        }

        Ok(loaded_names)
    }

    /// Get a loaded theme by name
    pub fn get(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name)
    }

    /// Get theme metadata by name
    pub fn get_metadata(&self, name: &str) -> Option<&ThemeMetadata> {
        self.metadata.get(name)
    }

    /// List all loaded theme names
    pub fn list(&self) -> Vec<&str> {
        self.themes.keys().map(|s| s.as_str()).collect()
    }

    /// List themes from a specific extension
    pub fn list_by_extension(&self, extension_id: &str) -> Vec<&str> {
        self.metadata
            .iter()
            .filter(|(_, m)| m.extension_id.as_deref() == Some(extension_id))
            .map(|(name, _)| name.as_str())
            .collect()
    }

    /// Get all loaded themes
    pub fn all(&self) -> &HashMap<String, Theme> {
        &self.themes
    }

    /// Unload all themes from an extension
    pub fn unload_extension(&mut self, extension_id: &str) -> Vec<String> {
        let to_remove: Vec<String> = self
            .metadata
            .iter()
            .filter(|(_, m)| m.extension_id.as_deref() == Some(extension_id))
            .map(|(name, _)| name.clone())
            .collect();

        for name in &to_remove {
            self.themes.remove(name);
            self.metadata.remove(name);
        }

        to_remove
    }

    /// Unload a specific theme
    pub fn unload(&mut self, name: &str) -> bool {
        self.metadata.remove(name);
        self.themes.remove(name).is_some()
    }

    /// Check if a theme exists
    pub fn contains(&self, name: &str) -> bool {
        self.themes.contains_key(name)
    }

    /// Get number of loaded themes
    pub fn len(&self) -> usize {
        self.themes.len()
    }

    /// Check if no themes are loaded
    pub fn is_empty(&self) -> bool {
        self.themes.is_empty()
    }
}

impl Default for ThemeLoader {
    fn default() -> Self {
        Self::new()
    }
}
