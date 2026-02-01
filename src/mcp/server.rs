//! MCP Server lifecycle management
//!
//! Provides high-level server connection, status tracking, and reconnection logic.

use super::client::McpManager;
use super::config::McpServerConfig;
use super::protocol::McpError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Server connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerStatus {
    /// Server is disconnected
    Disconnected,
    /// Server is connecting
    Connecting,
    /// Server is connected and ready
    Connected,
    /// Server has an error
    Error,
    /// Server is reconnecting after failure
    Reconnecting,
}

/// Server health information
#[derive(Debug, Clone)]
pub struct ServerHealth {
    /// Current status
    pub status: ServerStatus,
    /// Last successful ping time
    pub last_ping: Option<Instant>,
    /// Number of consecutive failures
    pub failure_count: u32,
    /// Last error message
    pub last_error: Option<String>,
    /// Uptime since last connection
    pub connected_since: Option<Instant>,
}

impl Default for ServerHealth {
    fn default() -> Self {
        Self {
            status: ServerStatus::Disconnected,
            last_ping: None,
            failure_count: 0,
            last_error: None,
            connected_since: None,
        }
    }
}

/// MCP Server registry with health tracking
pub struct McpServerRegistry {
    /// The underlying MCP manager
    manager: McpManager,
    /// Server configurations (for reconnection)
    configs: HashMap<String, McpServerConfig>,
    /// Server health status
    health: HashMap<String, ServerHealth>,
    /// Auto-reconnect enabled
    auto_reconnect: bool,
    /// Maximum reconnection attempts
    max_reconnect_attempts: u32,
    /// Reconnection delay
    reconnect_delay: Duration,
}

impl McpServerRegistry {
    /// Create a new server registry
    pub fn new() -> Self {
        Self {
            manager: McpManager::new(),
            configs: HashMap::new(),
            health: HashMap::new(),
            auto_reconnect: true,
            max_reconnect_attempts: 3,
            reconnect_delay: Duration::from_secs(5),
        }
    }

    /// Connect to an MCP server
    pub fn connect(&mut self, name: impl Into<String>, config: McpServerConfig) -> Result<(), McpError> {
        let name = name.into();

        // Store config for potential reconnection
        self.configs.insert(name.clone(), config.clone());

        // Update health status to connecting
        self.health.insert(name.clone(), ServerHealth {
            status: ServerStatus::Connecting,
            ..Default::default()
        });

        // Attempt connection
        match self.manager.connect(&name, config) {
            Ok(()) => {
                // Update health to connected
                if let Some(health) = self.health.get_mut(&name) {
                    health.status = ServerStatus::Connected;
                    health.connected_since = Some(Instant::now());
                    health.failure_count = 0;
                    health.last_error = None;
                }
                Ok(())
            }
            Err(e) => {
                // Update health to error
                if let Some(health) = self.health.get_mut(&name) {
                    health.status = ServerStatus::Error;
                    health.failure_count += 1;
                    health.last_error = Some(e.to_string());
                }
                Err(e)
            }
        }
    }

    /// Disconnect from an MCP server
    pub fn disconnect(&mut self, name: &str) -> Result<(), McpError> {
        self.manager.disconnect(name)?;
        if let Some(health) = self.health.get_mut(name) {
            health.status = ServerStatus::Disconnected;
            health.connected_since = None;
        }
        Ok(())
    }

    /// Disconnect from all servers
    pub fn disconnect_all(&mut self) {
        self.manager.disconnect_all();
        for health in self.health.values_mut() {
            health.status = ServerStatus::Disconnected;
            health.connected_since = None;
        }
    }

    /// Attempt to reconnect a failed server
    pub fn reconnect(&mut self, name: &str) -> Result<(), McpError> {
        let config = self.configs.get(name)
            .ok_or_else(|| McpError::Connection(format!("No config for server '{}'", name)))?
            .clone();

        // Check reconnection attempts
        if let Some(health) = self.health.get(name) {
            if health.failure_count >= self.max_reconnect_attempts {
                return Err(McpError::Connection(format!(
                    "Max reconnection attempts ({}) exceeded for '{}'",
                    self.max_reconnect_attempts, name
                )));
            }
        }

        // Update status to reconnecting
        if let Some(health) = self.health.get_mut(name) {
            health.status = ServerStatus::Reconnecting;
        }

        // Disconnect if still connected
        let _ = self.manager.disconnect(name);

        // Reconnect
        self.connect(name, config)
    }

    /// Get server health status
    pub fn health(&self, name: &str) -> Option<&ServerHealth> {
        self.health.get(name)
    }

    /// Get all server health statuses
    pub fn all_health(&self) -> impl Iterator<Item = (&String, &ServerHealth)> {
        self.health.iter()
    }

    /// Get connected server count
    pub fn connected_count(&self) -> usize {
        self.health.values()
            .filter(|h| h.status == ServerStatus::Connected)
            .count()
    }

    /// Get total server count
    pub fn total_count(&self) -> usize {
        self.configs.len()
    }

    /// Check if a server is connected
    pub fn is_connected(&self, name: &str) -> bool {
        self.health.get(name)
            .map(|h| h.status == ServerStatus::Connected)
            .unwrap_or(false)
    }

    /// Get the underlying manager for tool/resource operations
    pub fn manager(&self) -> &McpManager {
        &self.manager
    }

    /// Get mutable access to the underlying manager
    pub fn manager_mut(&mut self) -> &mut McpManager {
        &mut self.manager
    }

    /// Set auto-reconnect behavior
    pub fn set_auto_reconnect(&mut self, enabled: bool) {
        self.auto_reconnect = enabled;
    }

    /// Get server names
    pub fn server_names(&self) -> impl Iterator<Item = &String> {
        self.configs.keys()
    }

    /// Check and attempt reconnection for failed servers (call periodically)
    pub fn check_and_reconnect(&mut self) -> Vec<(String, Result<(), McpError>)> {
        if !self.auto_reconnect {
            return Vec::new();
        }

        let failed_servers: Vec<String> = self.health.iter()
            .filter(|(_, h)| h.status == ServerStatus::Error && h.failure_count < self.max_reconnect_attempts)
            .map(|(name, _)| name.clone())
            .collect();

        failed_servers.into_iter()
            .map(|name| {
                let result = self.reconnect(&name);
                (name, result)
            })
            .collect()
    }
}

impl Default for McpServerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe server registry wrapper
pub type SharedMcpRegistry = Arc<Mutex<McpServerRegistry>>;

/// Create a new shared server registry
pub fn create_shared_registry() -> SharedMcpRegistry {
    Arc::new(Mutex::new(McpServerRegistry::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = McpServerRegistry::new();
        assert_eq!(registry.connected_count(), 0);
        assert_eq!(registry.total_count(), 0);
    }

    #[test]
    fn test_health_default() {
        let health = ServerHealth::default();
        assert_eq!(health.status, ServerStatus::Disconnected);
        assert_eq!(health.failure_count, 0);
    }
}
