//! Icon theme loader and management

use super::types::*;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Icon theme loader
#[derive(Debug, Default)]
pub struct IconLoader {
    /// Loaded icon themes
    themes: HashMap<String, IconTheme>,
    /// Theme metadata
    metadata: HashMap<String, IconThemeMetadata>,
    /// Current active theme ID
    current_theme: Option<String>,
}

impl IconLoader {
    /// Create a new icon loader
    pub fn new() -> Self {
        Self::default()
    }

    /// Load an icon theme from a directory
    pub fn load_from_directory(&mut self, path: &Path) -> Result<String> {
        // Look for icon theme manifest
        let manifest_path = if path.join("icons.json").exists() {
            path.join("icons.json")
        } else if path.join("icon-theme.json").exists() {
            path.join("icon-theme.json")
        } else {
            anyhow::bail!("No icon theme manifest found in {:?}", path);
        };

        let content = std::fs::read_to_string(&manifest_path)
            .context("Failed to read icon theme manifest")?;

        let manifest: IconThemeManifest =
            serde_json::from_str(&content).context("Failed to parse icon theme manifest")?;

        let id = manifest.id.clone();
        let name = manifest.label.clone();

        let metadata = IconThemeMetadata {
            id: id.clone(),
            name,
            extension_id: None,
            path: Some(path.to_path_buf()),
        };

        let theme = IconTheme {
            metadata: metadata.clone(),
            manifest,
            base_path: path.to_path_buf(),
        };

        self.themes.insert(id.clone(), theme);
        self.metadata.insert(id.clone(), metadata);

        Ok(id)
    }

    /// Load icon themes from an extension
    pub fn load_extension(&mut self, extension_path: &Path) -> Result<Vec<String>> {
        let manifest_path = extension_path.join("extension.toml");
        if !manifest_path.exists() {
            return Ok(vec![]);
        }

        let content = std::fs::read_to_string(&manifest_path)?;
        let manifest: ExtensionManifestPartial = toml::from_str(&content)?;

        let extension_id = manifest.id.clone();
        let mut loaded = Vec::new();

        if let Some(icon_themes) = manifest.icon_themes {
            for icon_config in icon_themes {
                let theme_path = extension_path.join(&icon_config.path);
                if theme_path.exists() {
                    match self.load_icon_theme_file(&theme_path, Some(&extension_id)) {
                        Ok(id) => loaded.push(id),
                        Err(e) => {
                            eprintln!("Failed to load icon theme {:?}: {}", theme_path, e);
                        }
                    }
                }
            }
        }

        Ok(loaded)
    }

    /// Load a single icon theme file
    fn load_icon_theme_file(
        &mut self,
        path: &Path,
        extension_id: Option<&str>,
    ) -> Result<String> {
        let content = std::fs::read_to_string(path)?;
        let manifest: IconThemeManifest = serde_json::from_str(&content)?;

        let id = manifest.id.clone();
        let name = manifest.label.clone();

        let base_path = path.parent().unwrap_or(Path::new(".")).to_path_buf();

        let metadata = IconThemeMetadata {
            id: id.clone(),
            name,
            extension_id: extension_id.map(String::from),
            path: Some(path.to_path_buf()),
        };

        let theme = IconTheme {
            metadata: metadata.clone(),
            manifest,
            base_path,
        };

        self.themes.insert(id.clone(), theme);
        self.metadata.insert(id.clone(), metadata);

        Ok(id)
    }

    /// Get an icon theme by ID
    pub fn get(&self, id: &str) -> Option<&IconTheme> {
        self.themes.get(id)
    }

    /// Get the current active theme
    pub fn current(&self) -> Option<&IconTheme> {
        self.current_theme.as_ref().and_then(|id| self.get(id))
    }

    /// Set the current active theme
    pub fn set_current(&mut self, id: &str) -> bool {
        if self.themes.contains_key(id) {
            self.current_theme = Some(id.to_string());
            true
        } else {
            false
        }
    }

    /// List all loaded icon theme IDs
    pub fn list(&self) -> Vec<&str> {
        self.themes.keys().map(|s| s.as_str()).collect()
    }

    /// List icon themes by extension
    pub fn list_by_extension(&self, extension_id: &str) -> Vec<&str> {
        self.metadata
            .iter()
            .filter(|(_, m)| m.extension_id.as_deref() == Some(extension_id))
            .map(|(id, _)| id.as_str())
            .collect()
    }

    /// Get metadata for a theme
    pub fn get_metadata(&self, id: &str) -> Option<&IconThemeMetadata> {
        self.metadata.get(id)
    }

    /// Unload themes from an extension
    pub fn unload_extension(&mut self, extension_id: &str) -> Vec<String> {
        let to_remove: Vec<String> = self
            .metadata
            .iter()
            .filter(|(_, m)| m.extension_id.as_deref() == Some(extension_id))
            .map(|(id, _)| id.clone())
            .collect();

        for id in &to_remove {
            self.themes.remove(id);
            self.metadata.remove(id);
            if self.current_theme.as_deref() == Some(id) {
                self.current_theme = None;
            }
        }

        to_remove
    }
}

/// Partial extension manifest for reading icon theme config
#[derive(Debug, Deserialize)]
struct ExtensionManifestPartial {
    id: String,
    icon_themes: Option<Vec<IconThemeConfig>>,
}

/// Icon theme configuration in extension manifest
#[derive(Debug, Deserialize)]
struct IconThemeConfig {
    path: PathBuf,
}
