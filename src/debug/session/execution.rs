//! Execution control operations (continue, step, pause)

use super::super::client::DapClientError;
use super::core::DebugSession;
use super::types::DebugState;

impl DebugSession {
    /// Continue execution
    pub async fn continue_execution(&mut self) -> Result<(), DapClientError> {
        if let Some(thread_id) = self.current_thread_id {
            self.client.continue_execution(thread_id).await?;
            self.set_state(DebugState::Running);
        }
        Ok(())
    }

    /// Step over
    pub async fn step_over(&mut self) -> Result<(), DapClientError> {
        if let Some(thread_id) = self.current_thread_id {
            self.client.next(thread_id).await?;
            self.set_state(DebugState::Running);
        }
        Ok(())
    }

    /// Step into
    pub async fn step_into(&mut self) -> Result<(), DapClientError> {
        if let Some(thread_id) = self.current_thread_id {
            self.client.step_in(thread_id).await?;
            self.set_state(DebugState::Running);
        }
        Ok(())
    }

    /// Step out
    pub async fn step_out(&mut self) -> Result<(), DapClientError> {
        if let Some(thread_id) = self.current_thread_id {
            self.client.step_out(thread_id).await?;
            self.set_state(DebugState::Running);
        }
        Ok(())
    }

    /// Pause execution
    pub async fn pause(&mut self) -> Result<(), DapClientError> {
        if let Some(thread_id) = self.current_thread_id {
            self.client.pause(thread_id).await?;
        }
        Ok(())
    }
}
