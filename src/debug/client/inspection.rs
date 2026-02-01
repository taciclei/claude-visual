//! DAP State Inspection
//!
//! Inspect threads, stack frames, variables, and evaluate expressions.

use super::super::protocol::*;
use super::core::DapClient;
use super::types::*;

impl DapClient {
    /// Get threads
    pub async fn threads(&mut self) -> Result<Vec<Thread>, DapClientError> {
        let response = self.send_request("threads", None).await?;

        if !response.success {
            return Err(DapClientError::RequestFailed {
                command: "threads".to_string(),
                message: response.message.unwrap_or_default(),
            });
        }

        let threads = response
            .body
            .and_then(|b| b.get("threads").cloned())
            .and_then(|t| serde_json::from_value(t).ok())
            .unwrap_or_default();

        Ok(threads)
    }

    /// Get stack trace
    pub async fn stack_trace(
        &mut self,
        thread_id: i64,
        start_frame: Option<i64>,
        levels: Option<i64>,
    ) -> Result<Vec<StackFrame>, DapClientError> {
        let mut args = serde_json::json!({ "threadId": thread_id });

        if let Some(start) = start_frame {
            args["startFrame"] = serde_json::json!(start);
        }
        if let Some(lvls) = levels {
            args["levels"] = serde_json::json!(lvls);
        }

        let response = self.send_request("stackTrace", Some(args)).await?;

        if !response.success {
            return Err(DapClientError::RequestFailed {
                command: "stackTrace".to_string(),
                message: response.message.unwrap_or_default(),
            });
        }

        let frames = response
            .body
            .and_then(|b| b.get("stackFrames").cloned())
            .and_then(|f| serde_json::from_value(f).ok())
            .unwrap_or_default();

        Ok(frames)
    }

    /// Get scopes for a stack frame
    pub async fn scopes(&mut self, frame_id: i64) -> Result<Vec<Scope>, DapClientError> {
        let args = serde_json::json!({ "frameId": frame_id });

        let response = self.send_request("scopes", Some(args)).await?;

        if !response.success {
            return Err(DapClientError::RequestFailed {
                command: "scopes".to_string(),
                message: response.message.unwrap_or_default(),
            });
        }

        let scopes = response
            .body
            .and_then(|b| b.get("scopes").cloned())
            .and_then(|s| serde_json::from_value(s).ok())
            .unwrap_or_default();

        Ok(scopes)
    }

    /// Get variables
    pub async fn variables(
        &mut self,
        variables_reference: i64,
    ) -> Result<Vec<Variable>, DapClientError> {
        let args = serde_json::json!({ "variablesReference": variables_reference });

        let response = self.send_request("variables", Some(args)).await?;

        if !response.success {
            return Err(DapClientError::RequestFailed {
                command: "variables".to_string(),
                message: response.message.unwrap_or_default(),
            });
        }

        let variables = response
            .body
            .and_then(|b| b.get("variables").cloned())
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default();

        Ok(variables)
    }

    /// Evaluate expression
    pub async fn evaluate(
        &mut self,
        expression: &str,
        frame_id: Option<i64>,
        context: Option<&str>,
    ) -> Result<Variable, DapClientError> {
        let mut args = serde_json::json!({ "expression": expression });

        if let Some(fid) = frame_id {
            args["frameId"] = serde_json::json!(fid);
        }
        if let Some(ctx) = context {
            args["context"] = serde_json::json!(ctx);
        }

        let response = self.send_request("evaluate", Some(args)).await?;

        if !response.success {
            return Err(DapClientError::RequestFailed {
                command: "evaluate".to_string(),
                message: response.message.unwrap_or_default(),
            });
        }

        let result = response
            .body
            .and_then(|b| serde_json::from_value(b).ok())
            .unwrap_or(Variable {
                name: "result".to_string(),
                value: String::new(),
                var_type: None,
                presentation_hint: None,
                evaluate_name: None,
                variables_reference: 0,
                named_variables: 0,
                indexed_variables: 0,
                memory_reference: None,
            });

        Ok(result)
    }
}
