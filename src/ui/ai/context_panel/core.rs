//! Context Panel Core Logic

use gpui::*;

use crate::ai::context::{ContextItem, ContextItemType, ContextManager};

use super::types::ContextPanelEvent;

/// Context panel state
pub struct ContextPanel {
    /// Context manager
    pub(crate) context: ContextManager,
    /// Whether panel is expanded
    pub(crate) is_expanded: bool,
    /// Selected item for details view
    pub(crate) selected_item_id: Option<String>,
    /// Focus handle
    pub(crate) focus_handle: FocusHandle,
    /// Show token counts
    pub(crate) show_token_counts: bool,
}

impl ContextPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            context: ContextManager::default(),
            is_expanded: true,
            selected_item_id: None,
            focus_handle: cx.focus_handle(),
            show_token_counts: true,
        }
    }

    /// Get the context manager
    pub fn context(&self) -> &ContextManager {
        &self.context
    }

    /// Get mutable context manager
    pub fn context_mut(&mut self) -> &mut ContextManager {
        &mut self.context
    }

    /// Toggle panel expansion
    pub fn toggle_expanded(&mut self, cx: &mut Context<Self>) {
        self.is_expanded = !self.is_expanded;
        cx.notify();
    }

    /// Add a context item
    pub fn add_item(&mut self, item: ContextItem, cx: &mut Context<Self>) {
        if self.context.add(item).is_ok() {
            cx.notify();
        }
    }

    /// Remove a context item
    pub fn remove_item(&mut self, id: &str, cx: &mut Context<Self>) {
        self.context.remove(id);
        if self.selected_item_id.as_deref() == Some(id) {
            self.selected_item_id = None;
        }
        cx.emit(ContextPanelEvent::RemoveItem(id.to_string()));
        cx.notify();
    }

    /// Toggle pin on item
    pub fn toggle_pin(&mut self, id: &str, cx: &mut Context<Self>) {
        self.context.toggle_pin(id);
        cx.emit(ContextPanelEvent::TogglePin(id.to_string()));
        cx.notify();
    }

    /// Clear all context
    pub fn clear_all(&mut self, cx: &mut Context<Self>) {
        self.context.clear();
        self.selected_item_id = None;
        cx.emit(ContextPanelEvent::ClearAll);
        cx.notify();
    }

    /// Select item for details
    pub fn select_item(&mut self, id: Option<String>, cx: &mut Context<Self>) {
        self.selected_item_id = id;
        cx.notify();
    }

    /// Format context for AI prompt
    pub fn format_for_prompt(&self) -> String {
        self.context.format_for_prompt()
    }

    /// Get icon for item type
    pub(crate) fn icon_for_type(item_type: &ContextItemType) -> &'static str {
        match item_type {
            ContextItemType::File => "📄",
            ContextItemType::Snippet => "✂️",
            ContextItemType::Directory => "📁",
            ContextItemType::Diff => "📊",
            ContextItemType::SearchResults => "🔍",
            ContextItemType::Web => "🌐",
            ContextItemType::Image => "🖼️",
            ContextItemType::Custom(_) => "📎",
            ContextItemType::McpResource => "🔌",
            ContextItemType::McpPrompt => "💬",
        }
    }
}

impl EventEmitter<ContextPanelEvent> for ContextPanel {}

impl Focusable for ContextPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
