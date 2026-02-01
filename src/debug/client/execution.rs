//! DAP Execution Control
//!
//! Launch, attach, and control program execution.

use super::super::protocol::*;
use super::core::DapClient;
use super::types::*;

impl DapClient {
    /// Launch a debug session
    pub async fn launch(&mut self, args: LaunchArguments) -> Result<(), DapClientError> {
        if !self.initialized {
            return Err(DapClientError::NotInitialized);
        }

        let response = self
            .send_request("launch", Some(serde_json::to_value(&args).unwrap()))
            .await?;

        if !response.success {
            return Err(DapClientError::RequestFailed {
                command: "launch".to_string(),
                message: response.message.unwrap_or_default(),
            });
        }

        Ok(())
    }

    /// Attach to a running process
    pub async fn attach(&mut self, args: AttachArguments) -> Result<(), DapClientError> {
        if !self.initialized {
            return Err(DapClientError::NotInitialized);
        }

        let response = self
            .send_request("attach", Some(serde_json::to_value(&args).unwrap()))
            .await?;

        if !response.success {
            return Err(DapClientError::RequestFailed {
                command: "attach".to_string(),
                message: response.message.unwrap_or_default(),
            });
        }

        Ok(())
    }

    /// Continue execution
    pub async fn continue_execution(&mut self, thread_id: i64) -> Result<bool, DapClientError> {
        let args = serde_json::json!({ "threadId": thread_id });

        let response = self.send_request("continue", Some(args)).await?;

        if !response.success {
            return Err(DapClientError::RequestFailed {
                command: "continue".to_string(),
                message: response.message.unwrap_or_default(),
            });
        }

        let all_threads_continued = response
            .body
            .and_then(|b| b.get("allThreadsContinued").and_then(|v| v.as_bool()))
            .unwrap_or(false);

        Ok(all_threads_continued)
    }

    /// Step over (next)
    pub async fn next(&mut self, thread_id: i64) -> Result<(), DapClientError> {
        let args = serde_json::json!({ "threadId": thread_id });

        let response = self.send_request("next", Some(args)).await?;

        if !response.success {
            return Err(DapClientError::RequestFailed {
                command: "next".to_string(),
                message: response.message.unwrap_or_default(),
            });
        }

        Ok(())
    }

    /// Step into
    pub async fn step_in(&mut self, thread_id: i64) -> Result<(), DapClientError> {
        let args = serde_json::json!({ "threadId": thread_id });

        let response = self.send_request("stepIn", Some(args)).await?;

        if !response.success {
            return Err(DapClientError::RequestFailed {
                command: "stepIn".to_string(),
                message: response.message.unwrap_or_default(),
            });
        }

        Ok(())
    }

    /// Step out
    pub async fn step_out(&mut self, thread_id: i64) -> Result<(), DapClientError> {
        let args = serde_json::json!({ "threadId": thread_id });

        let response = self.send_request("stepOut", Some(args)).await?;

        if !response.success {
            return Err(DapClientError::RequestFailed {
                command: "stepOut".to_string(),
                message: response.message.unwrap_or_default(),
            });
        }

        Ok(())
    }

    /// Pause execution
    pub async fn pause(&mut self, thread_id: i64) -> Result<(), DapClientError> {
        let args = serde_json::json!({ "threadId": thread_id });

        let response = self.send_request("pause", Some(args)).await?;

        if !response.success {
            return Err(DapClientError::RequestFailed {
                command: "pause".to_string(),
                message: response.message.unwrap_or_default(),
            });
        }

        Ok(())
    }
}
