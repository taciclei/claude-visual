//! MCP Servers Panel
//!
//! UI component for managing MCP server connections.

mod core;
mod empty_state;
mod header;
mod render;
mod server_item;
mod types;

pub use core::McpServersPanel;
pub use types::{McpServersPanelEvent, ServerConnectionStatus, ServerItem};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_status_colors() {
        assert_ne!(
            ServerConnectionStatus::Connected.color(),
            ServerConnectionStatus::Disconnected.color()
        );
        assert_ne!(
            ServerConnectionStatus::Failed.color(),
            ServerConnectionStatus::Connected.color()
        );
    }

    #[test]
    fn test_server_status_labels() {
        assert_eq!(ServerConnectionStatus::Connected.label(), "Connected");
        assert_eq!(ServerConnectionStatus::Disconnected.label(), "Disconnected");
        assert_eq!(ServerConnectionStatus::Connecting.label(), "Connecting...");
        assert_eq!(ServerConnectionStatus::Failed.label(), "Failed");
    }
}
