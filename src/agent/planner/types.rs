//! Plan data structures

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::agent::task::{AgentTask, TaskPriority, TaskTree};

/// A step in an execution plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStep {
    /// Step number
    pub step_number: usize,
    /// Step title
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Tools that might be used
    pub tools: Vec<String>,
    /// Estimated tokens/cost
    pub estimated_tokens: Option<usize>,
    /// Dependencies (step numbers)
    pub depends_on: Vec<usize>,
    /// Risk level (0-10)
    pub risk_level: u8,
    /// Whether step requires approval
    pub requires_approval: bool,
}

/// An execution plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    /// Plan ID
    pub id: String,
    /// Plan title/goal
    pub title: String,
    /// Overall description
    pub description: String,
    /// Ordered steps
    pub steps: Vec<PlanStep>,
    /// Estimated total tokens
    pub estimated_total_tokens: Option<usize>,
    /// Created timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Plan metadata
    pub metadata: HashMap<String, String>,
}

impl Plan {
    /// Create a new plan
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.into(),
            description: description.into(),
            steps: Vec::new(),
            estimated_total_tokens: None,
            created_at: chrono::Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Add a step to the plan
    pub fn add_step(&mut self, step: PlanStep) {
        self.steps.push(step);
    }

    /// Convert plan to task tree
    pub fn to_task_tree(&self) -> TaskTree {
        let mut tree = TaskTree::new();

        // Create root task for the plan
        let root = AgentTask::new(&self.title, &self.description)
            .with_priority(TaskPriority::High);
        let root_id = tree.add_root(root);

        // Create subtasks for each step
        for step in &self.steps {
            let priority = if step.risk_level > 7 {
                TaskPriority::Critical
            } else if step.risk_level > 4 {
                TaskPriority::High
            } else {
                TaskPriority::Normal
            };

            let task = AgentTask::subtask(&root_id, &step.title, &step.description)
                .with_priority(priority);
            tree.add_subtask(&root_id, task);
        }

        tree
    }

    /// Get steps that can run (dependencies met)
    pub fn runnable_steps(&self, completed: &[usize]) -> Vec<&PlanStep> {
        self.steps
            .iter()
            .filter(|s| {
                !completed.contains(&s.step_number)
                    && s.depends_on.iter().all(|d| completed.contains(d))
            })
            .collect()
    }

    /// Get critical path (longest dependency chain)
    pub fn critical_path(&self) -> Vec<usize> {
        // Simple implementation - find longest chain
        let mut longest: Vec<usize> = Vec::new();

        for step in &self.steps {
            let path = self.path_to_step(step.step_number);
            if path.len() > longest.len() {
                longest = path;
            }
        }

        longest
    }

    fn path_to_step(&self, step_num: usize) -> Vec<usize> {
        let mut path = vec![step_num];

        if let Some(step) = self.steps.iter().find(|s| s.step_number == step_num) {
            if let Some(&dep) = step.depends_on.first() {
                let mut dep_path = self.path_to_step(dep);
                dep_path.push(step_num);
                if dep_path.len() > path.len() {
                    path = dep_path;
                }
            }
        }

        path
    }
}
