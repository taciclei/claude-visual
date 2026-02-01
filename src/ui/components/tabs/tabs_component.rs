//! Main Tabs component

use std::sync::Arc;

use gpui::*;

use crate::app::state::AppState;
use super::types::*;

/// Tabs component
pub struct Tabs {
    pub(super) app_state: Arc<AppState>,
    /// Tab items
    pub(super) tabs: Vec<TabItem>,
    /// Currently active tab ID
    pub(super) active: Option<String>,
    /// Style variant
    pub(super) style: TabsStyle,
    /// Size
    pub(super) size: TabsSize,
    /// Whether tabs should fill available width
    pub(super) full_width: bool,
    /// Whether to show close buttons on tabs
    pub(super) closable: bool,
}

impl Tabs {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            tabs: Vec::new(),
            active: None,
            style: TabsStyle::default(),
            size: TabsSize::default(),
            full_width: false,
            closable: false,
        }
    }

    /// Create with tabs
    pub fn with_tabs(app_state: Arc<AppState>, tabs: Vec<TabItem>, cx: &mut Context<Self>) -> Self {
        let active = tabs.first().map(|t| t.id.clone());
        let mut component = Self::new(app_state, cx);
        component.tabs = tabs;
        component.active = active;
        component
    }

    /// Set tabs
    pub fn set_tabs(&mut self, tabs: Vec<TabItem>, cx: &mut Context<Self>) {
        self.tabs = tabs;
        // Reset active if current active no longer exists
        if let Some(active) = &self.active {
            if !self.tabs.iter().any(|t| &t.id == active) {
                self.active = self.tabs.first().map(|t| t.id.clone());
            }
        }
        cx.notify();
    }

    /// Add a tab
    pub fn add_tab(&mut self, tab: TabItem, cx: &mut Context<Self>) {
        if self.active.is_none() {
            self.active = Some(tab.id.clone());
        }
        self.tabs.push(tab);
        cx.notify();
    }

    /// Remove a tab
    pub fn remove_tab(&mut self, id: &str, cx: &mut Context<Self>) {
        self.tabs.retain(|t| t.id != id);
        if self.active.as_deref() == Some(id) {
            self.active = self.tabs.first().map(|t| t.id.clone());
        }
        cx.notify();
    }

    /// Set active tab
    pub fn set_active(&mut self, id: impl Into<String>, cx: &mut Context<Self>) {
        let id = id.into();
        if self.tabs.iter().any(|t| t.id == id) {
            self.active = Some(id.clone());
            cx.emit(TabsEvent::Changed(id));
            cx.notify();
        }
    }

    /// Select a tab
    pub(super) fn select(&mut self, id: String, cx: &mut Context<Self>) {
        if self.active.as_ref() != Some(&id) {
            self.active = Some(id.clone());
            cx.emit(TabsEvent::Changed(id));
            cx.notify();
        }
    }

    /// Set style
    pub fn set_style(&mut self, style: TabsStyle, cx: &mut Context<Self>) {
        self.style = style;
        cx.notify();
    }

    /// Set size
    pub fn set_size(&mut self, size: TabsSize, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }

    /// Set full width
    pub fn set_full_width(&mut self, full_width: bool, cx: &mut Context<Self>) {
        self.full_width = full_width;
        cx.notify();
    }

    /// Set closable
    pub fn set_closable(&mut self, closable: bool, cx: &mut Context<Self>) {
        self.closable = closable;
        cx.notify();
    }

    /// Get active tab
    pub fn active_tab(&self) -> Option<&TabItem> {
        self.active.as_ref().and_then(|id| {
            self.tabs.iter().find(|t| &t.id == id)
        })
    }

    /// Get active tab ID
    pub fn active_id(&self) -> Option<&String> {
        self.active.as_ref()
    }
}

impl EventEmitter<TabsEvent> for Tabs {}
