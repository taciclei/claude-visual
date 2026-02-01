//! Collapsible card component

use std::sync::Arc;
use gpui::*;
use gpui::prelude::*;

use crate::app::state::AppState;

/// Collapsible card component
pub struct CollapsibleCard {
    pub(crate) app_state: Arc<AppState>,
    /// Title
    pub(crate) title: String,
    /// Whether card is expanded
    pub(crate) expanded: bool,
    /// Icon when collapsed
    pub(crate) collapsed_icon: String,
    /// Icon when expanded
    pub(crate) expanded_icon: String,
}

impl CollapsibleCard {
    pub fn new(app_state: Arc<AppState>, title: impl Into<String>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            title: title.into(),
            expanded: true,
            collapsed_icon: "▸".to_string(),
            expanded_icon: "▾".to_string(),
        }
    }

    /// Set expanded state
    pub fn set_expanded(&mut self, expanded: bool, cx: &mut Context<Self>) {
        self.expanded = expanded;
        cx.notify();
    }

    /// Toggle expanded state
    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        self.expanded = !self.expanded;
        cx.notify();
    }

    /// Set custom icons
    pub fn set_icons(&mut self, collapsed: impl Into<String>, expanded: impl Into<String>, cx: &mut Context<Self>) {
        self.collapsed_icon = collapsed.into();
        self.expanded_icon = expanded.into();
        cx.notify();
    }
}

impl Render for CollapsibleCard {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let icon = if self.expanded { &self.expanded_icon } else { &self.collapsed_icon };

        div()
            .id("collapsible-card")
            .w_full()
            .rounded_lg()
            .border_1()
            .border_color(theme.colors.border)
            .bg(theme.colors.surface)
            .overflow_hidden()
            // Header
            .child(
                div()
                    .id("collapsible-header")
                    .px_4()
                    .py_3()
                    .flex()
                    .items_center()
                    .gap_2()
                    .cursor_pointer()
                    .hover(|s| s.bg(theme.colors.surface_hover))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.toggle(cx);
                    }))
                    // Toggle icon
                    .child(
                        div()
                            .text_color(theme.colors.text_muted)
                            .text_sm()
                            .child(icon.clone())
                    )
                    // Title
                    .child(
                        div()
                            .flex_1()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.colors.text)
                            .child(self.title.clone())
                    )
            )
            // Content (shown when expanded)
            .when(self.expanded, |d| {
                d.child(
                    div()
                        .px_4()
                        .py_3()
                        .border_t_1()
                        .border_color(theme.colors.border)
                )
            })
    }
}
