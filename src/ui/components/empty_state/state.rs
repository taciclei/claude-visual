//! Main EmptyState component

use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;

use super::types::*;
use crate::app::state::AppState;

/// Empty state component
pub struct EmptyState {
    app_state: Arc<AppState>,
    /// Icon or illustration
    icon: String,
    /// Title text
    title: String,
    /// Description/help text
    description: Option<String>,
    /// Primary action button label
    action_label: Option<String>,
    /// Secondary action button label
    secondary_label: Option<String>,
    /// Size variant
    size: EmptyStateSize,
    /// Whether to show in a bordered container
    bordered: bool,
}

impl EmptyState {
    pub fn new(
        app_state: Arc<AppState>,
        icon: impl Into<String>,
        title: impl Into<String>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            app_state,
            icon: icon.into(),
            title: title.into(),
            description: None,
            action_label: None,
            secondary_label: None,
            size: EmptyStateSize::default(),
            bordered: false,
        }
    }

    /// Create for no results scenario
    pub fn no_results(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        let mut state = Self::new(app_state, "🔍", "No results found", cx);
        state.description = Some("Try adjusting your search or filters".to_string());
        state
    }

    /// Create for no data scenario
    pub fn no_data(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        let mut state = Self::new(app_state, "📭", "No data yet", cx);
        state.description = Some("Get started by adding some content".to_string());
        state
    }

    /// Create for no messages scenario
    pub fn no_messages(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        let mut state = Self::new(app_state, "💬", "No messages", cx);
        state.description = Some("Start a conversation to get going".to_string());
        state
    }

    /// Create for error scenario
    pub fn error(
        app_state: Arc<AppState>,
        message: impl Into<String>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut state = Self::new(app_state, "⚠️", "Something went wrong", cx);
        state.description = Some(message.into());
        state.action_label = Some("Try Again".to_string());
        state
    }

    /// Create for no connection scenario
    pub fn no_connection(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        let mut state = Self::new(app_state, "📡", "No connection", cx);
        state.description = Some("Check your internet connection and try again".to_string());
        state.action_label = Some("Retry".to_string());
        state
    }

    /// Set description
    pub fn set_description(&mut self, description: Option<String>, cx: &mut Context<Self>) {
        self.description = description;
        cx.notify();
    }

    /// Set primary action
    pub fn set_action(&mut self, label: Option<String>, cx: &mut Context<Self>) {
        self.action_label = label;
        cx.notify();
    }

    /// Set secondary action
    pub fn set_secondary_action(&mut self, label: Option<String>, cx: &mut Context<Self>) {
        self.secondary_label = label;
        cx.notify();
    }

    /// Set size
    pub fn set_size(&mut self, size: EmptyStateSize, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }

    /// Set bordered
    pub fn set_bordered(&mut self, bordered: bool, cx: &mut Context<Self>) {
        self.bordered = bordered;
        cx.notify();
    }

    /// Set icon
    pub fn set_icon(&mut self, icon: impl Into<String>, cx: &mut Context<Self>) {
        self.icon = icon.into();
        cx.notify();
    }

    /// Set title
    pub fn set_title(&mut self, title: impl Into<String>, cx: &mut Context<Self>) {
        self.title = title.into();
        cx.notify();
    }
}

impl EventEmitter<EmptyStateEvent> for EmptyState {}

impl Render for EmptyState {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let icon_size = self.size.icon_size();

        div()
            .id("empty-state")
            .w_full()
            .py_8()
            .px_4()
            .when(self.bordered, |d| {
                d.rounded_lg()
                    .border_1()
                    .border_color(theme.colors.border)
                    .bg(theme.colors.surface)
            })
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            // Icon
            .child(
                div()
                    .size(px(icon_size))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(icon_size * 0.8))
                    .child(self.icon.clone()),
            )
            // Title
            .child(
                div()
                    .text_size(self.size.title_size())
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.colors.text)
                    .text_center()
                    .child(self.title.clone()),
            )
            // Description
            .when_some(self.description.clone(), |d, desc| {
                d.child(
                    div()
                        .text_sm()
                        .text_color(theme.colors.text_muted)
                        .text_center()
                        .max_w(px(300.0))
                        .child(desc),
                )
            })
            // Actions
            .when(
                self.action_label.is_some() || self.secondary_label.is_some(),
                |d| {
                    d.child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .mt_2()
                            // Primary action
                            .when_some(self.action_label.clone(), |d, label| {
                                d.child(
                                    div()
                                        .id("empty-state-primary")
                                        .px_4()
                                        .py_2()
                                        .rounded(px(6.0))
                                        .bg(theme.colors.accent)
                                        .text_sm()
                                        .text_color(gpui::white())
                                        .font_weight(FontWeight::MEDIUM)
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.colors.accent.opacity(0.9)))
                                        .on_click(cx.listener(|_this, _, _window, cx| {
                                            cx.emit(EmptyStateEvent::PrimaryAction);
                                        }))
                                        .child(label),
                                )
                            })
                            // Secondary action
                            .when_some(self.secondary_label.clone(), |d, label| {
                                d.child(
                                    div()
                                        .id("empty-state-secondary")
                                        .px_4()
                                        .py_2()
                                        .rounded(px(6.0))
                                        .border_1()
                                        .border_color(theme.colors.border)
                                        .text_sm()
                                        .text_color(theme.colors.text_muted)
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.colors.surface_hover))
                                        .on_click(cx.listener(|_this, _, _window, cx| {
                                            cx.emit(EmptyStateEvent::SecondaryAction);
                                        }))
                                        .child(label),
                                )
                            }),
                    )
                },
            )
    }
}
