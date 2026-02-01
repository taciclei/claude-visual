//! Task tree for hierarchical task management

use super::node::TaskNode;
use super::task::AgentTask;
use super::types::TaskStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Task tree for hierarchical task management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTree {
    /// All tasks by ID
    tasks: HashMap<String, TaskNode>,
    /// Root task IDs
    roots: Vec<String>,
    /// Currently active task ID
    active_task_id: Option<String>,
}

impl Default for TaskTree {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskTree {
    /// Create a new task tree
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            roots: Vec::new(),
            active_task_id: None,
        }
    }

    /// Add a root task
    pub fn add_root(&mut self, task: AgentTask) -> String {
        let id = task.id.clone();
        self.tasks.insert(
            id.clone(),
            TaskNode {
                task,
                children: Vec::new(),
                is_expanded: true,
            },
        );
        self.roots.push(id.clone());
        id
    }

    /// Add a subtask
    pub fn add_subtask(&mut self, parent_id: &str, task: AgentTask) -> Option<String> {
        let id = task.id.clone();

        // Add task to map
        self.tasks.insert(
            id.clone(),
            TaskNode {
                task,
                children: Vec::new(),
                is_expanded: true,
            },
        );

        // Add to parent's children
        if let Some(parent) = self.tasks.get_mut(parent_id) {
            parent.children.push(id.clone());
            Some(id)
        } else {
            // Remove orphan task
            self.tasks.remove(&id);
            None
        }
    }

    /// Get a task by ID
    pub fn get(&self, id: &str) -> Option<&AgentTask> {
        self.tasks.get(id).map(|n| &n.task)
    }

    /// Get a mutable task by ID
    pub fn get_mut(&mut self, id: &str) -> Option<&mut AgentTask> {
        self.tasks.get_mut(id).map(|n| &mut n.task)
    }

    /// Get task node by ID
    pub fn get_node(&self, id: &str) -> Option<&TaskNode> {
        self.tasks.get(id)
    }

    /// Get children of a task
    pub fn children(&self, id: &str) -> Vec<&AgentTask> {
        self.tasks
            .get(id)
            .map(|n| n.children.iter().filter_map(|cid| self.get(cid)).collect())
            .unwrap_or_default()
    }

    /// Get root tasks
    pub fn roots(&self) -> Vec<&AgentTask> {
        self.roots.iter().filter_map(|id| self.get(id)).collect()
    }

    /// Get all tasks (flat list)
    pub fn all_tasks(&self) -> Vec<&AgentTask> {
        self.tasks.values().map(|n| &n.task).collect()
    }

    /// Get active task
    pub fn active_task(&self) -> Option<&AgentTask> {
        self.active_task_id.as_ref().and_then(|id| self.get(id))
    }

    /// Set active task
    pub fn set_active(&mut self, id: Option<String>) {
        self.active_task_id = id;
    }

    /// Toggle node expansion
    pub fn toggle_expanded(&mut self, id: &str) {
        if let Some(node) = self.tasks.get_mut(id) {
            node.is_expanded = !node.is_expanded;
        }
    }

    /// Get next pending task
    pub fn next_pending(&self) -> Option<&AgentTask> {
        // DFS to find first pending task
        for root_id in &self.roots {
            if let Some(task) = self.find_pending_dfs(root_id) {
                return Some(task);
            }
        }
        None
    }

    fn find_pending_dfs(&self, id: &str) -> Option<&AgentTask> {
        if let Some(node) = self.tasks.get(id) {
            // Check children first (depth-first)
            for child_id in &node.children {
                if let Some(task) = self.find_pending_dfs(child_id) {
                    return Some(task);
                }
            }
            // Then check self
            if node.task.status == TaskStatus::Pending {
                return Some(&node.task);
            }
        }
        None
    }

    /// Count tasks by status
    pub fn count_by_status(&self) -> HashMap<TaskStatus, usize> {
        let mut counts = HashMap::new();
        for task in self.all_tasks() {
            *counts.entry(task.status).or_insert(0) += 1;
        }
        counts
    }

    /// Get completion percentage
    pub fn completion_percentage(&self) -> f32 {
        let total = self.tasks.len();
        if total == 0 {
            return 0.0;
        }
        let completed = self
            .all_tasks()
            .iter()
            .filter(|t| t.status == TaskStatus::Completed)
            .count();
        (completed as f32 / total as f32) * 100.0
    }

    /// Check if all tasks are complete
    pub fn is_complete(&self) -> bool {
        self.all_tasks().iter().all(|t| t.status.is_terminal())
    }

    /// Check if any task failed
    pub fn has_failures(&self) -> bool {
        self.all_tasks()
            .iter()
            .any(|t| t.status == TaskStatus::Failed)
    }

    /// Clear all tasks
    pub fn clear(&mut self) {
        self.tasks.clear();
        self.roots.clear();
        self.active_task_id = None;
    }
}
