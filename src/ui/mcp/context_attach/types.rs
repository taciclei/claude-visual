//! Type definitions for MCP context attachment

/// Status of an attachment operation
#[derive(Debug, Clone)]
pub enum AttachmentStatus {
    /// Ready to attach
    Ready,
    /// Loading content
    Loading,
    /// Successfully attached
    Attached,
    /// Failed to attach
    Failed(String),
}

/// An item that can be attached to context
#[derive(Debug, Clone)]
pub struct AttachableResource {
    /// Server name
    pub(crate) server: String,
    /// Resource URI
    pub(crate) uri: String,
    /// Display name
    pub(crate) name: String,
    /// Description
    pub(crate) description: Option<String>,
    /// MIME type
    pub(crate) mime_type: Option<String>,
    /// Estimated size (bytes)
    pub(crate) size: Option<u64>,
    /// Attachment status
    pub(crate) status: AttachmentStatus,
}

impl AttachableResource {
    /// Create a new attachable resource
    pub fn new(
        server: impl Into<String>,
        uri: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            server: server.into(),
            uri: uri.into(),
            name: name.into(),
            description: None,
            mime_type: None,
            size: None,
            status: AttachmentStatus::Ready,
        }
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set MIME type
    pub fn with_mime_type(mut self, mime_type: impl Into<String>) -> Self {
        self.mime_type = Some(mime_type.into());
        self
    }

    /// Set size
    pub fn with_size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }
}

/// Events emitted by the context attachment panel
pub enum McpContextAttachEvent {
    /// Request to read a resource
    ReadResource {
        server: String,
        uri: String,
    },
    /// Resource content received
    ResourceReceived {
        server: String,
        uri: String,
        content: String,
    },
    /// Resource attached to context
    Attached {
        item_id: String,
        server: String,
        uri: String,
    },
    /// Failed to attach
    AttachFailed {
        server: String,
        uri: String,
        error: String,
    },
    /// Detach resource from context
    Detach(String),
    /// Panel closed
    Closed,
}
