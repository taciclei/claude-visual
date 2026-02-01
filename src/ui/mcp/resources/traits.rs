//! Trait implementations for MCP resources panel

use gpui::*;
use gpui::prelude::*;

use super::core::McpResourcesPanel;
use super::types::{McpResourcesPanelEvent, ResourcesTab};

impl EventEmitter<McpResourcesPanelEvent> for McpResourcesPanel {}

impl Focusable for McpResourcesPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for McpResourcesPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();

        div()
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .w_full()
            .bg(theme.colors.surface)
            .rounded_lg()
            .border_1()
            .border_color(theme.colors.border)
            .child(self.render_header(cx))
            .when(self.expanded, |this| {
                match self.active_tab {
                    ResourcesTab::Resources => {
                        let resources = self.filtered_resources();
                        this.child(
                            div()
                                .flex()
                                .flex_col()
                                .p_2()
                                .gap_1()
                                .when(resources.is_empty(), |this| {
                this.child(
                                        div()
                                            .py_4()
                                            .text_center()
                                            .text_sm()
                                            .text_color(theme.colors.text_muted)
                                            .child("No resources available"),
                                    )
                                })
                                .children(
                                    resources.iter().cloned().cloned().enumerate().map(|(i, r)| {
                                        self.render_resource_item(&r, i, cx)
                                    }),
                                ),
                        )
                    }
                    ResourcesTab::Prompts => {
                        let prompts = self.filtered_prompts();
                        this.child(
                            div()
                                .flex()
                                .flex_col()
                                .p_2()
                                .gap_1()
                                .when(prompts.is_empty(), |this| {
                this.child(
                                        div()
                                            .py_4()
                                            .text_center()
                                            .text_sm()
                                            .text_color(theme.colors.text_muted)
                                            .child("No prompts available"),
                                    )
                                })
                                .children(
                                    prompts.iter().cloned().cloned().enumerate().map(|(i, p)| {
                                        self.render_prompt_item(&p, i, cx)
                                    }),
                                ),
                        )
                    }
                }
            })
    }
}
