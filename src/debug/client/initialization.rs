//! DAP Client Initialization
//!
//! Starting and initializing debug adapters.

use std::process::{Command, Stdio};

use tokio::sync::mpsc;

use super::super::protocol::*;
use super::core::DapClient;
use super::types::*;

impl DapClient {
    /// Start the debug adapter
    pub fn start(&mut self) -> Result<mpsc::UnboundedReceiver<DapEvent>, DapClientError> {
        if self.process.is_some() {
            return Err(DapClientError::AlreadyRunning);
        }

        // Build command
        let mut cmd = Command::new(&self.config.command);
        cmd.args(&self.config.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if let Some(cwd) = &self.config.cwd {
            cmd.current_dir(cwd);
        }

        for (key, value) in &self.config.env {
            cmd.env(key, value);
        }

        // Spawn process
        let mut child = cmd
            .spawn()
            .map_err(|e| DapClientError::SpawnError(e.to_string()))?;

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| DapClientError::SpawnError("Failed to get stdin".to_string()))?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| DapClientError::SpawnError("Failed to get stdout".to_string()))?;

        // Create event channel
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        self.event_sender = Some(event_tx.clone());

        // Start reader thread
        let pending_requests = self.pending_requests.clone();
        std::thread::spawn(move || {
            Self::read_messages(stdout, pending_requests, event_tx);
        });

        self.process = Some(child);
        self.stdin = Some(stdin);

        Ok(event_rx)
    }

    /// Initialize the debug adapter
    pub async fn initialize(&mut self, adapter_id: &str) -> Result<Capabilities, DapClientError> {
        let args = InitializeArguments {
            adapter_id: adapter_id.to_string(),
            ..Default::default()
        };

        let response = self
            .send_request("initialize", Some(serde_json::to_value(&args).unwrap()))
            .await?;

        if !response.success {
            return Err(DapClientError::RequestFailed {
                command: "initialize".to_string(),
                message: response.message.unwrap_or_default(),
            });
        }

        let capabilities: Capabilities = response
            .body
            .map(|b| serde_json::from_value(b).unwrap_or_default())
            .unwrap_or_default();

        self.capabilities = Some(capabilities.clone());
        self.initialized = true;

        Ok(capabilities)
    }

    /// Send configurationDone request
    pub async fn configuration_done(&mut self) -> Result<(), DapClientError> {
        let response = self.send_request("configurationDone", None).await?;

        if !response.success {
            return Err(DapClientError::RequestFailed {
                command: "configurationDone".to_string(),
                message: response.message.unwrap_or_default(),
            });
        }

        Ok(())
    }
}
