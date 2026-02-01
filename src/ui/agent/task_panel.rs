//! Task Panel UI Component
//!
//! Displays the hierarchical task tree with status indicators.

use gpui::*;
use gpui::prelude::*;
use gpui::prelude::*;

use crate::agent::task::{AgentTask, TaskNode, TaskStatus, TaskTree};
use crate::app::theme::Theme;

/// Events emitted by the task panel
#[derive(Debug, Clone)]
pub enum TaskPanelEvent {
    /// Task was selected
    TaskSelected(String),
    /// Task was expanded/collapsed
    TaskToggled(String),
    /// Task action requested (retry, cancel, etc.)
    TaskAction(String, TaskAction),
}

/// Actions that can be performed on a task
#[derive(Debug, Clone)]
pub enum TaskAction {
    Retry,
    Cancel,
    ViewDetails,
}

/// Task panel component for displaying task tree
pub struct TaskPanel {
    /// Task tree to display
    task_tree: TaskTree,
    /// Currently selected task ID
    selected_task_id: Option<String>,
    /// Theme
    theme: Theme,
}

impl TaskPanel {
    /// Create a new task panel
    pub fn new(theme: Theme) -> Self {
        Self {
            task_tree: TaskTree::new(),
            selected_task_id: None,
            theme,
        }
    }

    /// Set the task tree to display
    pub fn set_task_tree(&mut self, tree: TaskTree, cx: &mut Context<Self>) {
        self.task_tree = tree;
        cx.notify();
    }

    /// Select a task
    pub fn select_task(&mut self, task_id: Option<String>, cx: &mut Context<Self>) {
        self.selected_task_id = task_id;
        cx.notify();
    }

    /// Toggle task expansion
    pub fn toggle_task(&mut self, task_id: &str, cx: &mut Context<Self>) {
        self.task_tree.toggle_expanded(task_id);
        cx.emit(TaskPanelEvent::TaskToggled(task_id.to_string()));
        cx.notify();
    }

    /// Render a task node
    fn render_task_node(&self, task: &AgentTask, node: &TaskNode, depth: usize, cx: &mut Context<Self>) -> impl IntoElement {
        let task_id = task.id.clone();
        let is_selected = self.selected_task_id.as_ref() == Some(&task_id);
        let has_children = !node.children.is_empty();
        let is_expanded = node.is_expanded;

        let bg_color = if is_selected {
            self.theme.colors.surface_hover
        } else {
            self.theme.colors.surface
        };

        let status_color = match task.status {
            TaskStatus::Pending => self.theme.colors.text_muted,
            TaskStatus::Running => self.theme.colors.accent,
            TaskStatus::Completed => self.theme.colors.success,
            TaskStatus::Failed => self.theme.colors.error,
            TaskStatus::Skipped => self.theme.colors.text_muted,
            TaskStatus::Paused => self.theme.colors.warning,
            TaskStatus::WaitingApproval => self.theme.colors.warning,
            TaskStatus::Cancelled => self.theme.colors.error,
        };

        let task_id_click = task_id.clone();
        let task_id_toggle = task_id.clone();

        div()
            .flex()
            .flex_col()
            .w_full()
            .child(
                div()
                    .id(ElementId::Name(task_id.clone().into()))
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_2()
                    .py_1()
                    .pl(px((depth * 16 + 8) as f32))
                    .bg(bg_color)
                    .hover(|s| s.bg(self.theme.colors.surface_hover))
                    .rounded_sm()
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        this.selected_task_id = Some(task_id_click.clone());
                        cx.emit(TaskPanelEvent::TaskSelected(task_id_click.clone()));
                        cx.notify();
                    }))
                    .child(
                        // Expand/collapse indicator
                        if has_children {
                            div()
                                .id(SharedString::from(format!("task-toggle-{}", task_id)))
                                .w_4()
                                .h_4()
                                .flex()
                                .items_center()
                                .justify_center()
                                .cursor_pointer()
                                .on_click(cx.listener(move |this, _, _window, cx| {
                                    this.toggle_task(&task_id_toggle, cx);
                                }))
                                .child(
                                    if is_expanded { "▼" } else { "▶" }
                                )
                        } else {
                            div().id("task-spacer").w_4().h_4()
                        }
                    )
                    .child(
                        // Status indicator
                        div()
                            .w_3()
                            .h_3()
                            .rounded_full()
                            .bg(status_color)
                    )
                    .child(
                        // Task title
                        div()
                            .flex_1()
                            .text_sm()
                            .text_color(self.theme.colors.text)
                            .child(task.title.clone())
                    )
                    .child(
                        // Status icon
                        div()
                            .text_sm()
                            .child(task.status.icon())
                    )
            )
            .when(is_expanded && has_children, |el| {
                el.children(
                    node.children.iter().filter_map(|child_id| {
                        let child_task = self.task_tree.get(child_id)?;
                        let child_node = self.task_tree.get_node(child_id)?;
                        Some(self.render_task_node(child_task, child_node, depth + 1, cx))
                    })
                )
            })
    }

    /// Render the header with summary
    fn render_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let counts = self.task_tree.count_by_status();
        let completed = counts.get(&TaskStatus::Completed).copied().unwrap_or(0);
        let total = self.task_tree.all_tasks().len();
        let percentage = self.task_tree.completion_percentage();

        div()
            .flex()
            .flex_col()
            .gap_2()
            .p_3()
            .border_b_1()
            .border_color(self.theme.colors.border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(self.theme.colors.text)
                            .child("Tasks")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(self.theme.colors.text_muted)
                            .child(format!("{}/{}", completed, total))
                    )
            )
            .child(
                // Progress bar
                div()
                    .w_full()
                    .h_1()
                    .bg(self.theme.colors.surface)
                    .rounded_full()
                    .overflow_hidden()
                    .child(
                        div()
                            .h_full()
                            .bg(self.theme.colors.success)
                            .rounded_full()
                            .w(relative(percentage / 100.0))
                    )
            )
    }
}

impl Render for TaskPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let roots = self.task_tree.roots();

        div()
            .flex()
            .flex_col()
            .h_full()
            .bg(self.theme.colors.background)
            .child(self.render_header(cx))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .id("task-scroll-container")
                    .overflow_y_scroll()
                    .p_2()
                    .children(
                        roots.iter().filter_map(|task| {
                            let node = self.task_tree.get_node(&task.id)?;
                            Some(self.render_task_node(task, node, 0, cx))
                        })
                    )
            )
    }
}

impl EventEmitter<TaskPanelEvent> for TaskPanel {}
