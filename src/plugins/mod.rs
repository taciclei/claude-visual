//! Plugin system for loading Zed-compatible extensions
//!
//! This module provides infrastructure for loading WASM-based plugins
//! compatible with Zed's extension format.
//!
//! Enable with the `plugins` feature flag:
//! ```toml
//! claude-visual = { features = ["plugins"] }
//! ```

pub mod api;
pub mod commands;
#[cfg(feature = "plugins")]
mod host;
pub mod icons;
#[cfg(feature = "plugins")]
mod loader;
#[cfg(feature = "plugins")]
mod registry;
pub mod themes;

pub use api::{ExtensionApi, ExtensionContext};
pub use commands::{CommandRegistry, CommandResult, SlashCommand};
#[cfg(feature = "plugins")]
pub use host::PluginHost;
pub use icons::{IconLoader, IconTheme, UiIconKind};
#[cfg(feature = "plugins")]
pub use loader::ExtensionLoader;
#[cfg(feature = "plugins")]
pub use registry::ExtensionRegistry;
pub use themes::ThemeLoader;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Extension manifest (extension.toml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub repository: Option<String>,
    pub lib: Option<LibConfig>,
    pub themes: Option<Vec<ThemeConfig>>,
    pub icon_themes: Option<Vec<IconThemeConfig>>,
    pub languages: Option<Vec<LanguageConfig>>,
    pub grammars: Option<std::collections::HashMap<String, GrammarConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibConfig {
    pub kind: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconThemeConfig {
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarConfig {
    pub repository: String,
    #[serde(rename = "rev")]
    pub revision: Option<String>,
    pub path: Option<PathBuf>,
}

/// Extension state
#[derive(Debug, Clone)]
pub struct Extension {
    pub manifest: ExtensionManifest,
    pub path: PathBuf,
    pub enabled: bool,
}

impl Extension {
    /// Create a new extension from a manifest and path
    pub fn new(manifest: ExtensionManifest, path: PathBuf) -> Self {
        Self {
            manifest,
            path,
            enabled: true,
        }
    }

    /// Get the extension's WASM file path
    pub fn wasm_path(&self) -> PathBuf {
        self.path.join("extension.wasm")
    }

    /// Check if this extension has a WASM component
    pub fn has_wasm(&self) -> bool {
        self.wasm_path().exists()
    }
}

/// Placeholder for when plugins feature is disabled
#[cfg(not(feature = "plugins"))]
pub struct PluginHost;

#[cfg(not(feature = "plugins"))]
impl PluginHost {
    pub fn new() -> Self {
        Self
    }

    pub fn load_extension(&self, _path: &std::path::Path) -> anyhow::Result<()> {
        anyhow::bail!("Plugin support not enabled. Compile with --features plugins")
    }
}

#[cfg(not(feature = "plugins"))]
impl Default for PluginHost {
    fn default() -> Self {
        Self::new()
    }
}
