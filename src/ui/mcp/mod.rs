//! MCP UI Components
//!
//! UI for managing MCP server connections and tools.

mod context_attach;
mod logs;
mod progress;
mod resources;
mod server_config;
mod servers;
mod tools;

pub use context_attach::{
    AttachableResource, AttachmentStatus, McpContextAttachEvent, McpContextAttachPanel,
};
pub use logs::{LogEntry, LogFilter, LogLevel, McpLogsPanel, McpLogsPanelEvent};
pub use progress::{ActiveExecution, ExecutionPhase, ToolProgressPanel, ToolProgressPanelEvent};
pub use resources::{
    McpResourcesPanel, McpResourcesPanelEvent, PromptItem, ResourceItem, ResourcesTab,
};
pub use server_config::{EditingServerConfig, ServerConfigEditor, ServerConfigEditorEvent};
pub use servers::{McpServersPanel, McpServersPanelEvent, ServerConnectionStatus, ServerItem};
pub use tools::{McpToolsPanel, McpToolsPanelEvent, PendingToolCall, ToolApprovalStatus, ToolItem};
