//! Debug session lifecycle operations (start, launch, attach, stop, restart)

use super::super::client::DapClientError;
use super::super::protocol::*;
use super::core::DebugSession;
use super::types::{DebugState, SessionEvent};

impl DebugSession {
    /// Start the debug session
    pub async fn start(&mut self, adapter_id: &str) -> Result<(), DapClientError> {
        self.set_state(DebugState::Initializing);

        // Start adapter
        let event_rx = self.client.start()?;
        self.event_receiver = Some(event_rx);

        // Initialize
        let capabilities = self.client.initialize(adapter_id).await?;
        self.capabilities = Some(capabilities);

        Ok(())
    }

    /// Launch a program
    pub async fn launch(&mut self, args: LaunchArguments) -> Result<(), DapClientError> {
        self.launch_config = Some(args.clone());

        // Set breakpoints first
        super::breakpoints::sync_breakpoints(self).await?;

        // Launch
        self.client.launch(args).await?;

        // Configuration done
        if self.capabilities.as_ref().map(|c| c.supports_configuration_done_request).unwrap_or(false) {
            self.client.configuration_done().await?;
        }

        self.set_state(DebugState::Running);

        Ok(())
    }

    /// Attach to a process
    pub async fn attach(&mut self, args: AttachArguments) -> Result<(), DapClientError> {
        // Set breakpoints first
        super::breakpoints::sync_breakpoints(self).await?;

        // Attach
        self.client.attach(args).await?;

        // Configuration done
        if self.capabilities.as_ref().map(|c| c.supports_configuration_done_request).unwrap_or(false) {
            self.client.configuration_done().await?;
        }

        self.set_state(DebugState::Running);

        Ok(())
    }

    /// Stop the debug session
    pub async fn stop(&mut self) -> Result<(), DapClientError> {
        self.client.disconnect(true).await?;
        self.set_state(DebugState::Terminated);
        let _ = self.session_event_sender.send(SessionEvent::Terminated);
        Ok(())
    }

    /// Restart the debug session
    pub async fn restart(&mut self) -> Result<(), DapClientError> {
        // Stop current session
        self.client.disconnect(true).await?;

        // Restart with same configuration
        if let Some(config) = self.launch_config.clone() {
            self.client.launch(config).await?;
            self.set_state(DebugState::Running);
        }

        Ok(())
    }
}
