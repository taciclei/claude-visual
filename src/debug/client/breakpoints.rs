//! DAP Breakpoint Management
//!
//! Setting and managing breakpoints.

use super::super::protocol::*;
use super::core::DapClient;
use super::types::*;

impl DapClient {
    /// Set breakpoints
    pub async fn set_breakpoints(
        &mut self,
        source: Source,
        breakpoints: Vec<BreakpointLocation>,
    ) -> Result<Vec<Breakpoint>, DapClientError> {
        let args = serde_json::json!({
            "source": source,
            "breakpoints": breakpoints.iter().map(|bp| {
                serde_json::json!({
                    "line": bp.line,
                    "column": bp.column,
                })
            }).collect::<Vec<_>>(),
        });

        let response = self.send_request("setBreakpoints", Some(args)).await?;

        if !response.success {
            return Err(DapClientError::RequestFailed {
                command: "setBreakpoints".to_string(),
                message: response.message.unwrap_or_default(),
            });
        }

        let breakpoints = response
            .body
            .and_then(|b| b.get("breakpoints").cloned())
            .and_then(|b| serde_json::from_value(b).ok())
            .unwrap_or_default();

        Ok(breakpoints)
    }
}
