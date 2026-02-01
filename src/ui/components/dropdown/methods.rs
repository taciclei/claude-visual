//! Dropdown methods and state management

use gpui::*;

use super::state::Dropdown;
use super::types::*;
use crate::app::state::AppState;
use std::sync::Arc;

impl Dropdown {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            options: Vec::new(),
            selected: None,
            is_open: false,
            size: DropdownSize::default(),
            placeholder: "Select...".to_string(),
            disabled: false,
            searchable: false,
            search_query: String::new(),
            label: None,
            error: None,
        }
    }

    /// Create with options
    pub fn with_options(
        app_state: Arc<AppState>,
        options: Vec<DropdownOption>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut dropdown = Self::new(app_state, cx);
        dropdown.options = options;
        dropdown
    }

    /// Set options
    pub fn set_options(&mut self, options: Vec<DropdownOption>, cx: &mut Context<Self>) {
        self.options = options;
        cx.notify();
    }

    /// Add an option
    pub fn add_option(&mut self, option: DropdownOption, cx: &mut Context<Self>) {
        self.options.push(option);
        cx.notify();
    }

    /// Set selected option by ID
    pub fn set_selected(&mut self, id: Option<String>, cx: &mut Context<Self>) {
        self.selected = id;
        cx.notify();
    }

    /// Select an option and emit event
    pub(super) fn select(&mut self, id: String, cx: &mut Context<Self>) {
        self.selected = Some(id.clone());
        self.is_open = false;
        self.search_query.clear();
        cx.emit(DropdownEvent::Changed(id));
        cx.notify();
    }

    /// Toggle dropdown open/closed
    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        self.is_open = !self.is_open;
        if self.is_open {
            cx.emit(DropdownEvent::Opened);
        } else {
            self.search_query.clear();
            cx.emit(DropdownEvent::Closed);
        }
        cx.notify();
    }

    /// Close dropdown
    pub fn close(&mut self, cx: &mut Context<Self>) {
        if self.is_open {
            self.is_open = false;
            self.search_query.clear();
            cx.emit(DropdownEvent::Closed);
            cx.notify();
        }
    }

    /// Set size
    pub fn set_size(&mut self, size: DropdownSize, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }

    /// Set placeholder
    pub fn set_placeholder(&mut self, placeholder: impl Into<String>, cx: &mut Context<Self>) {
        self.placeholder = placeholder.into();
        cx.notify();
    }

    /// Set disabled
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        cx.notify();
    }

    /// Set searchable
    pub fn set_searchable(&mut self, searchable: bool, cx: &mut Context<Self>) {
        self.searchable = searchable;
        cx.notify();
    }

    /// Set label
    pub fn set_label(&mut self, label: Option<String>, cx: &mut Context<Self>) {
        self.label = label;
        cx.notify();
    }

    /// Set error
    pub fn set_error(&mut self, error: Option<String>, cx: &mut Context<Self>) {
        self.error = error;
        cx.notify();
    }

    /// Get selected option
    pub fn selected_option(&self) -> Option<&DropdownOption> {
        self.selected
            .as_ref()
            .and_then(|id| self.options.iter().find(|o| &o.id == id))
    }

    /// Get filtered options (for search)
    pub(super) fn filtered_options(&self) -> Vec<&DropdownOption> {
        if self.search_query.is_empty() {
            self.options.iter().collect()
        } else {
            let query = self.search_query.to_lowercase();
            self.options
                .iter()
                .filter(|o| o.label.to_lowercase().contains(&query))
                .collect()
        }
    }
}
