//! Extension loader for downloading and installing extensions

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use super::ExtensionManifest;

/// Handles extension discovery and installation
pub struct ExtensionLoader {
    /// Directory where extensions are installed
    extensions_dir: PathBuf,
}

impl ExtensionLoader {
    /// Create a new extension loader
    pub fn new(extensions_dir: PathBuf) -> Self {
        Self { extensions_dir }
    }

    /// Get the default extensions directory
    pub fn default_extensions_dir() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("claude-visual")
            .join("extensions")
    }

    /// Discover installed extensions
    pub fn discover(&self) -> Result<Vec<ExtensionManifest>> {
        let mut extensions = Vec::new();

        if !self.extensions_dir.exists() {
            return Ok(extensions);
        }

        for entry in std::fs::read_dir(&self.extensions_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Ok(manifest) = self.load_manifest(&path) {
                    extensions.push(manifest);
                }
            }
        }

        Ok(extensions)
    }

    /// Load extension manifest from a directory
    pub fn load_manifest(&self, path: &Path) -> Result<ExtensionManifest> {
        let manifest_path = path.join("extension.toml");
        let content =
            std::fs::read_to_string(&manifest_path).context("Failed to read extension.toml")?;
        toml::from_str(&content).context("Failed to parse extension.toml")
    }

    /// Get the installation path for an extension
    pub fn extension_path(&self, id: &str) -> PathBuf {
        self.extensions_dir.join(id)
    }

    /// Install an extension from a local path (copy)
    pub fn install_from_path(&self, source: &Path) -> Result<String> {
        let manifest = self.load_manifest(source)?;
        let dest = self.extension_path(&manifest.id);

        if dest.exists() {
            std::fs::remove_dir_all(&dest)?;
        }

        copy_dir_recursive(source, &dest)?;

        Ok(manifest.id)
    }

    /// Uninstall an extension
    pub fn uninstall(&self, id: &str) -> Result<()> {
        let path = self.extension_path(id);
        if path.exists() {
            std::fs::remove_dir_all(&path)?;
        }
        Ok(())
    }

    /// Check if an extension is installed
    pub fn is_installed(&self, id: &str) -> bool {
        self.extension_path(id).exists()
    }
}

impl Default for ExtensionLoader {
    fn default() -> Self {
        Self::new(Self::default_extensions_dir())
    }
}

/// Recursively copy a directory
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
