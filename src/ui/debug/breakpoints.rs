//! Breakpoints List View
//!
//! UI for managing breakpoints.

use std::path::PathBuf;
use std::sync::Arc;

use gpui::*;
use gpui::prelude::*;
use gpui::prelude::*;

use crate::app::state::AppState;

/// Breakpoint item for display
#[derive(Debug, Clone)]
pub struct BreakpointItem {
    /// Unique ID
    pub id: usize,
    /// File path
    pub file: PathBuf,
    /// Line number
    pub line: i64,
    /// Condition (if any)
    pub condition: Option<String>,
    /// Hit count (if any)
    pub hit_count: Option<i64>,
    /// Is enabled
    pub enabled: bool,
    /// Is verified by adapter
    pub verified: bool,
}

impl BreakpointItem {
    /// Get file name
    pub fn file_name(&self) -> String {
        self.file
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    }

    /// Get display location
    pub fn location(&self) -> String {
        format!("{}:{}", self.file_name(), self.line)
    }
}

/// Events from breakpoints list
#[derive(Debug, Clone)]
pub enum BreakpointsListEvent {
    /// Toggle breakpoint enabled
    Toggle(usize),
    /// Remove breakpoint
    Remove(usize),
    /// Go to breakpoint location
    GoTo(usize),
    /// Edit breakpoint condition
    EditCondition(usize),
    /// Add new breakpoint
    Add,
}

impl EventEmitter<BreakpointsListEvent> for BreakpointsList {}

/// Breakpoints list component
pub struct BreakpointsList {
    app_state: Arc<AppState>,
    breakpoints: Vec<BreakpointItem>,
    selected: Option<usize>,
}

impl BreakpointsList {
    /// Create a new breakpoints list
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            breakpoints: Vec::new(),
            selected: None,
        }
    }

    /// Set breakpoints
    pub fn set_breakpoints(&mut self, breakpoints: Vec<BreakpointItem>, cx: &mut Context<Self>) {
        self.breakpoints = breakpoints;
        cx.notify();
    }

    /// Add a breakpoint
    pub fn add_breakpoint(&mut self, item: BreakpointItem, cx: &mut Context<Self>) {
        self.breakpoints.push(item);
        cx.notify();
    }

    /// Remove a breakpoint
    pub fn remove_breakpoint(&mut self, id: usize, cx: &mut Context<Self>) {
        self.breakpoints.retain(|b| b.id != id);
        cx.notify();
    }

    /// Update breakpoint
    pub fn update_breakpoint(&mut self, id: usize, verified: bool, cx: &mut Context<Self>) {
        if let Some(bp) = self.breakpoints.iter_mut().find(|b| b.id == id) {
            bp.verified = verified;
        }
        cx.notify();
    }

    /// Toggle breakpoint enabled
    pub fn toggle_breakpoint(&mut self, id: usize, cx: &mut Context<Self>) {
        if let Some(bp) = self.breakpoints.iter_mut().find(|b| b.id == id) {
            bp.enabled = !bp.enabled;
        }
        cx.notify();
    }

    /// Get breakpoint count
    pub fn count(&self) -> usize {
        self.breakpoints.len()
    }

    /// Get enabled breakpoint count
    pub fn enabled_count(&self) -> usize {
        self.breakpoints.iter().filter(|b| b.enabled).count()
    }
}

impl Render for BreakpointsList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let breakpoints = self.breakpoints.clone();
        let selected = self.selected;

        div()
            .w_full()
            .flex()
            .flex_col()
            // Header
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_2()
                    .py_1()
                    .bg(theme.colors.surface)
                    .border_b_1()
                    .border_color(theme.colors.border)
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.colors.text)
                            .child(format!("Breakpoints ({})", self.count())),
                    )
                    .child(
                        div()
                            .id("add-breakpoint")
                            .px_2()
                            .py_0p5()
                            .rounded_sm()
                            .text_xs()
                            .text_color(theme.colors.accent)
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.colors.accent.opacity(0.1)))
                            .on_click(cx.listener(|_this, _, _window, cx| {
                                cx.emit(BreakpointsListEvent::Add);
                            }))
                            .child("+ Add"),
                    ),
            )
            // List
            .child(
                div()
                    .flex_1()
                    .id("scroll-breakpoints-list")
                    .overflow_y_scroll()
                    .children(breakpoints.into_iter().map(|bp| {
                        let id = bp.id;
                        let is_selected = selected == Some(id);

                        div()
                            .id(SharedString::from(format!("breakpoint-{}", id)))
                            .flex()
                            .items_center()
                            .gap_2()
                            .px_2()
                            .py_1()
                            .cursor_pointer()
                            .when(is_selected, |d| d.bg(theme.colors.accent.opacity(0.1)))
                            .hover(|s| s.bg(theme.colors.surface))
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                this.selected = Some(id);
                                cx.emit(BreakpointsListEvent::GoTo(id));
                                cx.notify();
                            }))
                            // Enable toggle
                            .child(
                                div()
                                    .id(SharedString::from(format!("bp-toggle-{}", id)))
                                    .w(px(16.0))
                                    .h(px(16.0))
                                    .rounded_sm()
                                    .border_1()
                                    .border_color(if bp.enabled {
                                        theme.colors.error
                                    } else {
                                        theme.colors.border
                                    })
                                    .when(bp.enabled, |d| d.bg(theme.colors.error))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .cursor_pointer()
                                    .on_click(cx.listener(move |_this, _, _window, cx| {
                                        cx.emit(BreakpointsListEvent::Toggle(id));
                                    }))
                                    .when(bp.enabled, |d| {
                                        d.child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.surface)
                                                .child("●"),
                                        )
                                    }),
                            )
                            // Location
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text)
                                            .child(bp.location()),
                                    )
                                    .when(bp.condition.is_some(), |d| {
                                        d.child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.text_muted)
                                                .child(format!("if {}", bp.condition.as_ref().unwrap())),
                                        )
                                    }),
                            )
                            // Verified indicator
                            .when(!bp.verified && bp.enabled, |d| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.warning)
                                        .child("?"),
                                )
                            })
                            // Remove button
                            .child(
                                div()
                                    .id(SharedString::from(format!("bp-remove-{}", id)))
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .cursor_pointer()
                                    .hover(|s| s.text_color(theme.colors.error))
                                    .on_click(cx.listener(move |_this, _, _window, cx| {
                                        cx.emit(BreakpointsListEvent::Remove(id));
                                    }))
                                    .child("×"),
                            )
                    })),
            )
            // Empty state
            .when(self.breakpoints.is_empty(), |d| {
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py_4()
                        .text_xs()
                        .text_color(theme.colors.text_muted)
                        .child("No breakpoints set"),
                )
            })
    }
}
