//! Watch view core logic

use std::sync::Arc;

use gpui::*;

use crate::app::state::AppState;

use super::events::WatchViewEvent;
use super::types::{WatchChild, WatchExpression};

/// Watch expressions view component
pub struct WatchView {
    pub(crate) app_state: Arc<AppState>,
    pub(crate) expressions: Vec<WatchExpression>,
    pub(crate) input_text: String,
    pub(crate) is_adding: bool,
    pub(crate) next_id: usize,
    pub(crate) selected_id: Option<usize>,
    pub(crate) editing_id: Option<usize>,
}

impl WatchView {
    /// Create a new watch view
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            expressions: Vec::new(),
            input_text: String::new(),
            is_adding: false,
            next_id: 1,
            selected_id: None,
            editing_id: None,
        }
    }

    /// Add a watch expression
    pub fn add_expression(&mut self, expression: String, cx: &mut Context<Self>) {
        if expression.trim().is_empty() {
            return;
        }

        let watch = WatchExpression::new(self.next_id, expression.clone());
        self.expressions.push(watch);
        self.next_id += 1;
        self.is_adding = false;
        self.input_text.clear();

        cx.emit(WatchViewEvent::Add(expression));
        cx.notify();
    }

    /// Remove a watch expression
    pub fn remove_expression(&mut self, id: usize, cx: &mut Context<Self>) {
        self.expressions.retain(|e| e.id != id);
        if self.selected_id == Some(id) {
            self.selected_id = None;
        }
        cx.notify();
    }

    /// Update expression value
    pub fn update_value(
        &mut self,
        id: usize,
        value: String,
        value_type: Option<String>,
        cx: &mut Context<Self>,
    ) {
        if let Some(expr) = self.expressions.iter_mut().find(|e| e.id == id) {
            expr.value = Some(value);
            expr.value_type = value_type;
            expr.error = None;
            expr.is_evaluating = false;
        }
        cx.notify();
    }

    /// Set error for expression
    pub fn set_error(&mut self, id: usize, error: String, cx: &mut Context<Self>) {
        if let Some(expr) = self.expressions.iter_mut().find(|e| e.id == id) {
            expr.error = Some(error);
            expr.value = None;
            expr.is_evaluating = false;
        }
        cx.notify();
    }

    /// Set evaluating state
    pub fn set_evaluating(&mut self, id: usize, cx: &mut Context<Self>) {
        if let Some(expr) = self.expressions.iter_mut().find(|e| e.id == id) {
            expr.is_evaluating = true;
        }
        cx.notify();
    }

    /// Update children for expanded expression
    pub fn set_children(&mut self, id: usize, children: Vec<WatchChild>, cx: &mut Context<Self>) {
        if let Some(expr) = self.expressions.iter_mut().find(|e| e.id == id) {
            expr.children = children;
            expr.expanded = true;
        }
        cx.notify();
    }

    /// Toggle expand state
    pub fn toggle_expand(&mut self, id: usize, cx: &mut Context<Self>) {
        if let Some(expr) = self.expressions.iter_mut().find(|e| e.id == id) {
            if expr.expanded {
                expr.expanded = false;
            } else if expr.has_children() {
                expr.expanded = true;
            } else {
                cx.emit(WatchViewEvent::Expand(id));
            }
        }
        cx.notify();
    }

    /// Clear all values (e.g., when debug session ends)
    pub fn clear_values(&mut self, cx: &mut Context<Self>) {
        for expr in &mut self.expressions {
            expr.value = None;
            expr.error = None;
            expr.is_evaluating = false;
            expr.children.clear();
            expr.expanded = false;
        }
        cx.notify();
    }

    /// Clear all expressions
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.expressions.clear();
        self.selected_id = None;
        self.editing_id = None;
        cx.notify();
    }

    /// Start adding a new expression
    pub(crate) fn start_adding(&mut self, cx: &mut Context<Self>) {
        self.is_adding = true;
        self.input_text.clear();
        cx.notify();
    }

    /// Cancel adding
    pub(crate) fn cancel_adding(&mut self, cx: &mut Context<Self>) {
        self.is_adding = false;
        self.input_text.clear();
        cx.notify();
    }

    /// Get expression count
    pub fn count(&self) -> usize {
        self.expressions.len()
    }
}
