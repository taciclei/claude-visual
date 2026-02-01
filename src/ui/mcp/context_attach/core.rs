//! Core implementation for MCP context attachment

use std::sync::Arc;

use gpui::*;

use crate::ai::context::{ContextItem, ContextManager};
use crate::app::state::AppState;

use super::types::*;

/// Panel for attaching MCP resources to context
pub struct McpContextAttachPanel {
    pub(crate) app_state: Arc<AppState>,
    /// Available resources to attach
    pub(crate) available_resources: Vec<AttachableResource>,
    /// Attached item IDs (for tracking)
    pub(crate) attached_items: Vec<String>,
    /// Context manager reference
    pub(crate) context_manager: Option<Arc<std::sync::RwLock<ContextManager>>>,
    /// Search filter
    pub(crate) filter_text: String,
    /// Selected index
    pub(crate) selected_index: Option<usize>,
    /// Show attached only
    pub(crate) show_attached_only: bool,
    /// Focus handle
    pub(crate) focus_handle: FocusHandle,
}

impl EventEmitter<McpContextAttachEvent> for McpContextAttachPanel {}

impl McpContextAttachPanel {
    /// Create a new context attach panel
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            available_resources: Vec::new(),
            attached_items: Vec::new(),
            context_manager: None,
            filter_text: String::new(),
            selected_index: None,
            show_attached_only: false,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Set context manager reference
    pub fn set_context_manager(
        &mut self,
        manager: Arc<std::sync::RwLock<ContextManager>>,
        cx: &mut Context<Self>,
    ) {
        self.context_manager = Some(manager);
        cx.notify();
    }

    /// Add available resources
    pub fn set_resources(&mut self, resources: Vec<AttachableResource>, cx: &mut Context<Self>) {
        self.available_resources = resources;
        cx.notify();
    }

    /// Add a single resource
    pub fn add_resource(&mut self, resource: AttachableResource, cx: &mut Context<Self>) {
        // Check if already exists
        if !self
            .available_resources
            .iter()
            .any(|r| r.server == resource.server && r.uri == resource.uri)
        {
            self.available_resources.push(resource);
            cx.notify();
        }
    }

    /// Handle resource content received
    pub fn on_resource_received(
        &mut self,
        server: &str,
        uri: &str,
        content: String,
        cx: &mut Context<Self>,
    ) {
        // Find the resource
        if let Some(resource) = self
            .available_resources
            .iter_mut()
            .find(|r| r.server == server && r.uri == uri)
        {
            // Create context item
            let item = ContextItem::mcp_resource(
                server,
                uri,
                &resource.name,
                &content,
                resource.mime_type.clone(),
            );
            let item_id = item.id.clone();

            // Try to add to context manager
            if let Some(ref manager) = self.context_manager {
                if let Ok(mut mgr) = manager.write() {
                    match mgr.add(item) {
                        Ok(()) => {
                            resource.status = AttachmentStatus::Attached;
                            self.attached_items.push(item_id.clone());
                            cx.emit(McpContextAttachEvent::Attached {
                                item_id,
                                server: server.to_string(),
                                uri: uri.to_string(),
                            });
                        }
                        Err(e) => {
                            resource.status =
                                AttachmentStatus::Failed(format!("Context error: {:?}", e));
                            cx.emit(McpContextAttachEvent::AttachFailed {
                                server: server.to_string(),
                                uri: uri.to_string(),
                                error: format!("{:?}", e),
                            });
                        }
                    }
                }
            } else {
                resource.status = AttachmentStatus::Failed("No context manager".to_string());
            }
            cx.notify();
        }
    }

    /// Request to attach a resource
    pub(super) fn request_attach(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(resource) = self.available_resources.get_mut(index) {
            if matches!(resource.status, AttachmentStatus::Ready) {
                resource.status = AttachmentStatus::Loading;
                cx.emit(McpContextAttachEvent::ReadResource {
                    server: resource.server.clone(),
                    uri: resource.uri.clone(),
                });
                cx.notify();
            }
        }
    }

    /// Detach a resource
    pub(super) fn detach(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(resource) = self.available_resources.get_mut(index) {
            // Find and remove from attached items
            if let Some(ref manager) = self.context_manager {
                if let Ok(mut mgr) = manager.write() {
                    // Find the item ID by matching server/uri in metadata
                    let item_id = mgr
                        .items()
                        .iter()
                        .find(|item| {
                            item.metadata.get("server").map(|s| s.as_str())
                                == Some(&resource.server)
                                && item.metadata.get("uri").map(|s| s.as_str())
                                    == Some(&resource.uri)
                        })
                        .map(|item| item.id.clone());

                    if let Some(id) = item_id {
                        mgr.remove(&id);
                        self.attached_items.retain(|i| i != &id);
                        cx.emit(McpContextAttachEvent::Detach(id));
                    }
                }
            }
            resource.status = AttachmentStatus::Ready;
            cx.notify();
        }
    }

    /// Get filtered resources
    pub(super) fn filtered_resources(&self) -> Vec<(usize, &AttachableResource)> {
        self.available_resources
            .iter()
            .enumerate()
            .filter(|(_, r)| {
                // Filter by attached status if needed
                if self.show_attached_only {
                    if !matches!(r.status, AttachmentStatus::Attached) {
                        return false;
                    }
                }

                // Filter by search text
                if !self.filter_text.is_empty() {
                    let query = self.filter_text.to_lowercase();
                    return r.name.to_lowercase().contains(&query)
                        || r.uri.to_lowercase().contains(&query)
                        || r.server.to_lowercase().contains(&query)
                        || r.description
                            .as_ref()
                            .map(|d| d.to_lowercase().contains(&query))
                            .unwrap_or(false);
                }

                true
            })
            .collect()
    }

    /// Toggle show attached only
    pub(super) fn toggle_attached_only(&mut self, cx: &mut Context<Self>) {
        self.show_attached_only = !self.show_attached_only;
        cx.notify();
    }

    /// Format size for display
    pub(super) fn format_size(&self, bytes: u64) -> String {
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.1} KB", bytes as f64 / 1024.0)
        } else {
            format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
        }
    }

    /// Get icon for resource based on MIME type
    pub(super) fn get_resource_icon(&self, mime_type: Option<&str>) -> &'static str {
        match mime_type {
            Some(m) if m.starts_with("text/") => "📄",
            Some(m) if m.starts_with("image/") => "🖼️",
            Some(m) if m.starts_with("application/json") => "📋",
            Some(m) if m.starts_with("application/xml") => "📋",
            Some(m) if m.contains("directory") || m.contains("folder") => "📁",
            _ => "📦",
        }
    }
}

impl Focusable for McpContextAttachPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
