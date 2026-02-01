//! State inspection operations (threads, stack, variables, evaluate)

use super::super::client::DapClientError;
use super::super::protocol::*;
use super::core::DebugSession;

impl DebugSession {
    /// Get threads
    pub async fn refresh_threads(&mut self) -> Result<&[Thread], DapClientError> {
        self.threads = self.client.threads().await?;
        Ok(&self.threads)
    }

    /// Get threads (cached)
    pub fn threads(&self) -> &[Thread] {
        &self.threads
    }

    /// Get stack trace
    pub async fn refresh_stack_trace(&mut self) -> Result<&[StackFrame], DapClientError> {
        if let Some(thread_id) = self.current_thread_id {
            self.stack_frames = self.client.stack_trace(thread_id, None, Some(20)).await?;

            // Set current frame to top frame
            if let Some(frame) = self.stack_frames.first() {
                self.current_frame_id = Some(frame.id);
            }
        }
        Ok(&self.stack_frames)
    }

    /// Get stack frames (cached)
    pub fn stack_frames(&self) -> &[StackFrame] {
        &self.stack_frames
    }

    /// Get variables for current frame
    pub async fn refresh_variables(&mut self) -> Result<Vec<(Scope, Vec<Variable>)>, DapClientError> {
        let mut result = Vec::new();

        if let Some(frame_id) = self.current_frame_id {
            let scopes = self.client.scopes(frame_id).await?;

            for scope in scopes {
                let vars = self.client.variables(scope.variables_reference).await?;
                self.variables_cache.insert(scope.variables_reference, vars.clone());
                result.push((scope, vars));
            }
        }

        Ok(result)
    }

    /// Expand variable (get children)
    pub async fn expand_variable(&mut self, variables_reference: i64) -> Result<Vec<Variable>, DapClientError> {
        let vars = self.client.variables(variables_reference).await?;
        self.variables_cache.insert(variables_reference, vars.clone());
        Ok(vars)
    }

    /// Evaluate expression
    pub async fn evaluate(&mut self, expression: &str) -> Result<Variable, DapClientError> {
        self.client.evaluate(expression, self.current_frame_id, Some("repl")).await
    }
}
