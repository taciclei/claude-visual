//! Extension registry for managing installed and available extensions

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

use super::{Extension, ExtensionLoader, ExtensionManifest, PluginHost};

/// Registry of all extensions (installed and available)
pub struct ExtensionRegistry {
    /// Installed extensions (id -> Extension)
    installed: Arc<RwLock<HashMap<String, Extension>>>,
    /// Extension loader
    loader: ExtensionLoader,
    /// Plugin host for WASM execution
    host: Arc<RwLock<PluginHost>>,
}

impl ExtensionRegistry {
    /// Create a new extension registry
    pub fn new() -> Result<Self> {
        Ok(Self {
            installed: Arc::new(RwLock::new(HashMap::new())),
            loader: ExtensionLoader::default(),
            host: Arc::new(RwLock::new(PluginHost::new()?)),
        })
    }

    /// Initialize the registry by discovering installed extensions
    pub fn initialize(&self) -> Result<()> {
        let manifests = self.loader.discover()?;
        let mut installed = self.installed.write();

        for manifest in manifests {
            let path = self.loader.extension_path(&manifest.id);
            let extension = Extension::new(manifest.clone(), path);
            installed.insert(manifest.id, extension);
        }

        Ok(())
    }

    /// Install an extension from a local path
    pub fn install(&self, path: &std::path::Path) -> Result<String> {
        let id = self.loader.install_from_path(path)?;

        // Load the extension
        let manifest = self.loader.load_manifest(&self.loader.extension_path(&id))?;
        let extension = Extension::new(manifest, self.loader.extension_path(&id));

        self.installed.write().insert(id.clone(), extension);

        Ok(id)
    }

    /// Uninstall an extension
    pub fn uninstall(&self, id: &str) -> Result<()> {
        // Unload from host first
        let _ = self.host.write().unload_extension(id);

        // Remove from installed
        self.installed.write().remove(id);

        // Delete files
        self.loader.uninstall(id)?;

        Ok(())
    }

    /// Enable an extension (load its WASM if present)
    pub fn enable(&self, id: &str) -> Result<()> {
        let extension = self.installed.read().get(id).cloned();

        if let Some(mut ext) = extension {
            if ext.has_wasm() {
                self.host.write().load_extension(&ext.path)?;
            }
            ext.enabled = true;
            self.installed.write().insert(id.to_string(), ext);
        }

        Ok(())
    }

    /// Disable an extension
    pub fn disable(&self, id: &str) -> Result<()> {
        let _ = self.host.write().unload_extension(id);

        if let Some(ext) = self.installed.write().get_mut(id) {
            ext.enabled = false;
        }

        Ok(())
    }

    /// Get an installed extension
    pub fn get(&self, id: &str) -> Option<Extension> {
        self.installed.read().get(id).cloned()
    }

    /// List all installed extensions
    pub fn list_installed(&self) -> Vec<ExtensionManifest> {
        self.installed
            .read()
            .values()
            .map(|e| e.manifest.clone())
            .collect()
    }

    /// List enabled extensions
    pub fn list_enabled(&self) -> Vec<ExtensionManifest> {
        self.installed
            .read()
            .values()
            .filter(|e| e.enabled)
            .map(|e| e.manifest.clone())
            .collect()
    }

    /// Check if an extension is installed
    pub fn is_installed(&self, id: &str) -> bool {
        self.installed.read().contains_key(id)
    }

    /// Check if an extension is enabled
    pub fn is_enabled(&self, id: &str) -> bool {
        self.installed
            .read()
            .get(id)
            .map(|e| e.enabled)
            .unwrap_or(false)
    }
}

impl Default for ExtensionRegistry {
    fn default() -> Self {
        Self::new().expect("Failed to create extension registry")
    }
}
