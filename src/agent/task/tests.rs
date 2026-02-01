//! Task management tests

use super::*;

#[test]
fn test_task_creation() {
    let task = AgentTask::new("Test task", "Description");
    assert_eq!(task.status, TaskStatus::Pending);
    assert!(task.started_at.is_none());
}

#[test]
fn test_task_lifecycle() {
    let mut task = AgentTask::new("Test", "Desc");

    task.start();
    assert_eq!(task.status, TaskStatus::Running);
    assert!(task.started_at.is_some());

    task.complete("Done!");
    assert_eq!(task.status, TaskStatus::Completed);
    assert!(task.completed_at.is_some());
}

#[test]
fn test_task_tree() {
    let mut tree = TaskTree::new();

    let root = AgentTask::new("Root", "Root task");
    let root_id = tree.add_root(root);

    let sub = AgentTask::subtask(&root_id, "Sub", "Subtask");
    tree.add_subtask(&root_id, sub);

    assert_eq!(tree.roots().len(), 1);
    assert_eq!(tree.children(&root_id).len(), 1);
}

#[test]
fn test_completion_percentage() {
    let mut tree = TaskTree::new();

    let mut task1 = AgentTask::new("Task 1", "");
    task1.complete("Done");
    tree.add_root(task1);

    let task2 = AgentTask::new("Task 2", "");
    tree.add_root(task2);

    assert_eq!(tree.completion_percentage(), 50.0);
}
