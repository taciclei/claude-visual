//! DAP Client Lifecycle
//!
//! Disconnect and cleanup operations.

use super::core::DapClient;
use super::types::*;

impl DapClient {
    /// Disconnect from adapter
    pub async fn disconnect(&mut self, terminate: bool) -> Result<(), DapClientError> {
        let args = serde_json::json!({
            "terminateDebuggee": terminate,
        });

        let _ = self.send_request("disconnect", Some(args)).await;

        // Kill process
        if let Some(mut process) = self.process.take() {
            let _ = process.kill();
        }

        self.stdin = None;
        self.initialized = false;
        self.capabilities = None;

        Ok(())
    }
}
