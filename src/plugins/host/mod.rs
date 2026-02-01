//! WASM plugin host using wasmtime
//!
//! This module provides the runtime environment for executing
//! Zed-compatible WASM extensions.

mod linker;
mod provider;
mod state;

pub use provider::{ExtensionCapability, ExtensionProvider, SharedExtension};
pub use state::HostState;

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use parking_lot::RwLock;
use wasmtime::*;

use self::state::LoadedExtension;
use super::api::ExtensionApi;
use super::{Extension, ExtensionManifest};

/// Plugin host that manages WASM extension execution
pub struct PluginHost {
    engine: Engine,
    loaded: HashMap<String, LoadedExtension>,
    /// Shared extension API
    api: Arc<RwLock<ExtensionApi>>,
}

impl PluginHost {
    /// Create a new plugin host
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        config.async_support(true);
        config.wasm_component_model(true);

        let engine = Engine::new(&config)?;

        Ok(Self {
            engine,
            loaded: HashMap::new(),
            api: Arc::new(RwLock::new(ExtensionApi::new())),
        })
    }

    /// Create a new plugin host with a shared API instance
    pub fn with_api(api: Arc<RwLock<ExtensionApi>>) -> Result<Self> {
        let mut config = Config::new();
        config.async_support(true);
        config.wasm_component_model(true);

        let engine = Engine::new(&config)?;

        Ok(Self {
            engine,
            loaded: HashMap::new(),
            api,
        })
    }

    /// Get the extension API
    pub fn api(&self) -> Arc<RwLock<ExtensionApi>> {
        Arc::clone(&self.api)
    }

    /// Load an extension from a directory
    pub fn load_extension(&mut self, path: &Path) -> Result<String> {
        // Read manifest
        let manifest_path = path.join("extension.toml");
        let manifest_content = std::fs::read_to_string(&manifest_path)
            .context("Failed to read extension.toml")?;
        let manifest: ExtensionManifest = toml::from_str(&manifest_content)
            .context("Failed to parse extension.toml")?;

        let extension_id = manifest.id.clone();
        let extension = Extension::new(manifest, path.to_path_buf());

        // Load WASM if present
        if extension.has_wasm() {
            let wasm_bytes = std::fs::read(extension.wasm_path())
                .context("Failed to read extension.wasm")?;

            let module = Module::new(&self.engine, &wasm_bytes)
                .context("Failed to compile WASM module")?;

            let host_state = state::HostState {
                extension_id: extension_id.clone(),
                workdir: None,
                api: Arc::clone(&self.api),
                string_buffer: Vec::with_capacity(4096),
            };

            let mut store = Store::new(&self.engine, host_state);

            // Create linker with host functions
            let linker = linker::create_linker(&self.engine)?;

            // Instantiate the module
            let instance = linker.instantiate(&mut store, &module)
                .context("Failed to instantiate WASM module")?;

            // Call the extension's init function if it exists
            if let Some(init) = instance.get_typed_func::<(), ()>(&mut store, "init").ok() {
                init.call(&mut store, ())
                    .context("Failed to call extension init function")?;
                tracing::info!("Extension {} initialized", extension_id);
            }

            self.loaded.insert(
                extension_id.clone(),
                LoadedExtension {
                    extension,
                    module,
                    store,
                    instance: Some(instance),
                },
            );
        } else {
            // Extension without WASM (themes only, etc.)
            let host_state = state::HostState {
                extension_id: extension_id.clone(),
                workdir: None,
                api: Arc::clone(&self.api),
                string_buffer: Vec::new(),
            };
            let store = Store::new(&self.engine, host_state);

            self.loaded.insert(
                extension_id.clone(),
                LoadedExtension {
                    extension,
                    module: Module::new(&self.engine, "(module)")?,
                    store,
                    instance: None,
                },
            );
        }

        Ok(extension_id)
    }

    /// Unload an extension
    pub fn unload_extension(&mut self, id: &str) -> Result<()> {
        if let Some(_ext) = self.loaded.remove(id) {
            // Clean up API resources for this extension
            self.api.read().cleanup_extension(id);
            tracing::info!("Extension {} unloaded", id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Extension not found"))
        }
    }

    /// Get list of loaded extension IDs
    pub fn loaded_extensions(&self) -> Vec<&str> {
        self.loaded.keys().map(|s| s.as_str()).collect()
    }

    /// Check if an extension is loaded
    pub fn is_loaded(&self, id: &str) -> bool {
        self.loaded.contains_key(id)
    }

    /// Call a function on an extension
    pub fn call_extension_function<P, R>(
        &mut self,
        extension_id: &str,
        function_name: &str,
        params: P,
    ) -> Result<R>
    where
        P: wasmtime::WasmParams,
        R: wasmtime::WasmResults,
    {
        let ext = self.loaded.get_mut(extension_id)
            .ok_or_else(|| anyhow::anyhow!("Extension not loaded: {}", extension_id))?;

        let instance = ext.instance.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Extension has no WASM instance"))?;

        let func: TypedFunc<P, R> = instance
            .get_typed_func(&mut ext.store, function_name)
            .context(format!("Function not found: {}", function_name))?;

        func.call(&mut ext.store, params)
            .context(format!("Failed to call function: {}", function_name))
    }

    /// Get the extension manifest
    pub fn get_manifest(&self, extension_id: &str) -> Option<&ExtensionManifest> {
        self.loaded.get(extension_id).map(|ext| &ext.extension.manifest)
    }
}

impl Default for PluginHost {
    fn default() -> Self {
        Self::new().expect("Failed to create plugin host")
    }
}
