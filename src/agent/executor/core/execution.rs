//! Core execution logic

use crate::agent::planner::Plan;
use crate::agent::task::ToolCall;

use super::super::types::{ExecutorEvent, ExecutorState, PlanResult};
use super::executor::AgentExecutor;

impl AgentExecutor {
    /// Load a plan for execution
    pub fn load_plan(&mut self, plan: Plan) {
        self.task_tree = plan.to_task_tree();
        self.current_plan = Some(plan);
        self.completed_steps.clear();
        self.state = ExecutorState::Idle;
        self.emit_event(ExecutorEvent::StateChanged(self.state));
    }

    /// Start or resume execution
    pub async fn start(&mut self) -> Result<PlanResult, String> {
        if self.current_plan.is_none() {
            return Err("No plan loaded".to_string());
        }

        self.set_state(ExecutorState::Running);
        self.started_at = Some(std::time::Instant::now());

        // Reset flags
        *self.pause_requested.lock().await = false;
        *self.cancel_requested.lock().await = false;

        // Execute the plan
        let result = self.execute_plan().await;

        // Calculate duration
        let duration_ms = self
            .started_at
            .map(|s| s.elapsed().as_millis() as u64)
            .unwrap_or(0);

        let plan_result = match &result {
            Ok(_) => PlanResult {
                plan_id: self
                    .current_plan
                    .as_ref()
                    .map(|p| p.id.clone())
                    .unwrap_or_default(),
                success: true,
                completed_steps: self.completed_steps.len(),
                total_steps: self
                    .current_plan
                    .as_ref()
                    .map(|p| p.steps.len())
                    .unwrap_or(0),
                error: None,
                duration_ms,
            },
            Err(e) => PlanResult {
                plan_id: self
                    .current_plan
                    .as_ref()
                    .map(|p| p.id.clone())
                    .unwrap_or_default(),
                success: false,
                completed_steps: self.completed_steps.len(),
                total_steps: self
                    .current_plan
                    .as_ref()
                    .map(|p| p.steps.len())
                    .unwrap_or(0),
                error: Some(e.clone()),
                duration_ms,
            },
        };

        self.emit_event(ExecutorEvent::PlanCompleted(plan_result.clone()));
        result.map(|_| plan_result)
    }

    /// Execute the plan
    pub(super) async fn execute_plan(&mut self) -> Result<(), String> {
        let plan = self.current_plan.as_ref().ok_or("No plan")?;
        let total_steps = plan.steps.len();

        loop {
            // Check for cancel
            if *self.cancel_requested.lock().await {
                self.set_state(ExecutorState::Cancelled);
                return Err("Execution cancelled".to_string());
            }

            // Check for pause
            if *self.pause_requested.lock().await {
                self.set_state(ExecutorState::Paused);
                return Ok(());
            }

            // Get next runnable steps
            let plan = self.current_plan.as_ref().ok_or("No plan")?;
            let runnable = plan.runnable_steps(&self.completed_steps);

            if runnable.is_empty() {
                // Check if we're done
                if self.completed_steps.len() >= total_steps {
                    self.set_state(ExecutorState::Completed);
                    return Ok(());
                } else {
                    // Deadlock - no runnable steps but not complete
                    self.set_state(ExecutorState::Failed);
                    return Err("Plan execution deadlocked - no runnable steps".to_string());
                }
            }

            // Execute first runnable step
            let step = runnable[0].clone();

            // Check if approval is required
            if step.requires_approval && !self.should_auto_approve(&step) {
                self.set_state(ExecutorState::WaitingApproval);
                self.emit_event(ExecutorEvent::ApprovalRequired(
                    step.step_number.to_string(),
                    format!("Step {}: {}", step.step_number, step.title),
                ));
                return Ok(());
            }

            // Execute the step
            self.execute_step(&step).await?;

            // Emit progress
            self.emit_event(ExecutorEvent::Progress {
                completed: self.completed_steps.len(),
                total: total_steps,
                current_task: Some(step.title.clone()),
            });
        }
    }

    /// Execute a single step
    pub(super) async fn execute_step(
        &mut self,
        step: &crate::agent::planner::PlanStep,
    ) -> Result<(), String> {
        // Find or create task for this step
        let task_id = format!("step-{}", step.step_number);

        self.emit_event(ExecutorEvent::TaskStarted(task_id.clone()));

        // Execute each tool in the step
        for tool_name in &step.tools {
            let tool_call = ToolCall {
                name: tool_name.clone(),
                arguments: serde_json::json!({}),
                requires_approval: step.requires_approval,
                result: None,
            };

            self.emit_event(ExecutorEvent::ToolExecutionRequested(
                task_id.clone(),
                tool_call.clone(),
            ));

            // Execute tool if we have an executor
            if let Some(executor) = &self.tool_executor {
                match executor.execute(&tool_call).await {
                    Ok(result) => {
                        self.emit_event(ExecutorEvent::ToolExecutionCompleted(
                            tool_name.clone(),
                            result.clone(),
                        ));
                        if !result.success {
                            self.emit_event(ExecutorEvent::TaskFailed(
                                task_id.clone(),
                                result
                                    .error
                                    .unwrap_or_else(|| "Tool execution failed".to_string()),
                            ));
                            return Err(format!("Tool {} failed", tool_name));
                        }
                    }
                    Err(e) => {
                        self.emit_event(ExecutorEvent::TaskFailed(task_id.clone(), e.clone()));
                        return Err(e);
                    }
                }
            }
        }

        // Mark step as completed
        self.completed_steps.push(step.step_number);
        self.emit_event(ExecutorEvent::TaskCompleted(
            task_id,
            format!("Step {} completed", step.step_number),
        ));

        Ok(())
    }
}
