//! Rendering implementations for MCP resources panel

use gpui::*;
use gpui::prelude::*;

use super::core::McpResourcesPanel;
use super::types::{McpResourcesPanelEvent, PromptItem, ResourceItem, ResourcesTab};

impl McpResourcesPanel {
    /// Render a resource item
    pub(crate) fn render_resource_item(&self, resource: &ResourceItem, index: usize, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();
        let is_selected = self.selected_index == Some(index);
        let server = resource.server.clone();
        let uri = resource.resource.uri.clone();

        div()
            .id(ElementId::Name(format!("mcp-resource-{}", index).into()))
            .w_full()
            .py_2()
            .px_3()
            .rounded_md()
            .cursor_pointer()
            .when(is_selected, |this| this.bg(theme.colors.surface_hover))
            .hover(|this| this.bg(theme.colors.surface_hover))
            .on_click(cx.listener(move |_this, _event, _window, cx| {
                cx.emit(McpResourcesPanelEvent::ReadResource {
                    server: server.clone(),
                    uri: uri.clone(),
                });
            }))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.colors.text)
                                    .child(resource.resource.name.clone()),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child(format!("({})", resource.server)),
                            ),
                    )
                    .child(
                        div()
                            .text_xs()
                            .font_family("monospace")
                            .text_color(theme.colors.text_muted)
                            .overflow_hidden()
                            .child(resource.resource.uri.clone()),
                    )
                    .when_some(resource.resource.description.clone(), |this, desc| {
                        this.child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .child(desc),
                        )
                    }),
            )
    }

    /// Render a prompt item
    pub(crate) fn render_prompt_item(&self, prompt: &PromptItem, index: usize, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();
        let is_selected = self.selected_index == Some(index);
        let server = prompt.server.clone();
        let prompt_name = prompt.prompt.name.clone();

        div()
            .id(ElementId::Name(format!("mcp-prompt-{}", index).into()))
            .w_full()
            .py_2()
            .px_3()
            .rounded_md()
            .cursor_pointer()
            .when(is_selected, |this| this.bg(theme.colors.surface_hover))
            .hover(|this| this.bg(theme.colors.surface_hover))
            .on_click(cx.listener(move |_this, _event, _window, cx| {
                cx.emit(McpResourcesPanelEvent::UsePrompt {
                    server: server.clone(),
                    prompt_name: prompt_name.clone(),
                });
            }))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.colors.text)
                                    .child(format!("/{}", prompt.prompt.name)),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child(format!("({})", prompt.server)),
                            ),
                    )
                    .when_some(prompt.prompt.description.clone(), |this, desc| {
                        this.child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .child(desc),
                        )
                    })
                    .when_some(prompt.prompt.arguments.clone(), |this, args| {
                        if !args.is_empty() {
                            this.child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .flex_wrap()
                                    .gap_1()
                                    .mt_1()
                                    .children(args.iter().map(|arg| {
                                        div()
                                            .px_2()
                                            .py_0p5()
                                            .rounded_md()
                                            .bg(theme.colors.surface)
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child(format!(
                                                "{}{}",
                                                arg.name,
                                                if arg.required.unwrap_or(false) { "*" } else { "" }
                                            ))
                                    })),
                            )
                        } else {
                            div()
                        }
                    }),
            )
    }

    /// Render tab buttons
    pub(crate) fn render_tabs(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();

        div()
            .flex()
            .flex_row()
            .gap_1()
            .child(
                div()
                    .id("tab-resources")
                    .px_3()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .bg(if self.active_tab == ResourcesTab::Resources {
                        theme.colors.accent.opacity(0.2)
                    } else {
                        theme.colors.surface
                    })
                    .text_color(if self.active_tab == ResourcesTab::Resources {
                        theme.colors.accent
                    } else {
                        theme.colors.text_muted
                    })
                    .text_xs()
                    .child(format!("Resources ({})", self.resources.len()))
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.set_tab(ResourcesTab::Resources, cx);
                    })),
            )
            .child(
                div()
                    .id("tab-prompts")
                    .px_3()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .bg(if self.active_tab == ResourcesTab::Prompts {
                        theme.colors.accent.opacity(0.2)
                    } else {
                        theme.colors.surface
                    })
                    .text_color(if self.active_tab == ResourcesTab::Prompts {
                        theme.colors.accent
                    } else {
                        theme.colors.text_muted
                    })
                    .text_xs()
                    .child(format!("Prompts ({})", self.prompts.len()))
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.set_tab(ResourcesTab::Prompts, cx);
                    })),
            )
    }

    /// Render the header
    pub(crate) fn render_header(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();

        div()
            .flex()
            .flex_col()
            .gap_2()
            .px_3()
            .py_2()
            .border_b_1()
            .border_color(theme.colors.border)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .id("resources-header-toggle")
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.colors.text)
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _event, _window, cx| {
                                this.toggle_expanded(cx);
                            }))
                            .child("MCP Resources & Prompts"),
                    ),
            )
            .when(self.expanded, |this| {
                this.child(self.render_tabs(cx))
            })
    }
}
