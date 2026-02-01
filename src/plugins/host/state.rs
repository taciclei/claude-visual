//! WASM runtime state management

use std::sync::Arc;
use parking_lot::RwLock;
use wasmtime::{Module, Store, Instance};

use crate::plugins::api::ExtensionApi;
use crate::plugins::Extension;

/// Extension loaded into the runtime
pub(super) struct LoadedExtension {
    #[allow(dead_code)]
    pub extension: Extension,
    #[allow(dead_code)]
    pub module: Module,
    #[allow(dead_code)]
    pub store: Store<HostState>,
    #[allow(dead_code)]
    pub instance: Option<Instance>,
}

/// Host state passed to WASM modules
pub struct HostState {
    /// Extension ID
    pub extension_id: String,
    /// Working directory
    pub workdir: Option<std::path::PathBuf>,
    /// Reference to the shared API
    pub api: Arc<RwLock<ExtensionApi>>,
    /// Memory buffer for string passing
    pub string_buffer: Vec<u8>,
}
