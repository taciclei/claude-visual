//! Variables view component

use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;

use crate::app::state::AppState;

use super::events::VariablesViewEvent;
use super::types::{ScopeItem, VariableItem};

impl EventEmitter<VariablesViewEvent> for VariablesView {}

/// Variables view component
pub struct VariablesView {
    app_state: Arc<AppState>,
    scopes: Vec<ScopeItem>,
    selected_variable: Option<String>,
}

impl VariablesView {
    /// Create a new variables view
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            scopes: Vec::new(),
            selected_variable: None,
        }
    }

    /// Set scopes and variables
    pub fn set_scopes(&mut self, scopes: Vec<ScopeItem>, cx: &mut Context<Self>) {
        self.scopes = scopes;
        cx.notify();
    }

    /// Clear all variables
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.scopes.clear();
        cx.notify();
    }

    /// Toggle scope expanded
    pub fn toggle_scope(&mut self, name: &str, cx: &mut Context<Self>) {
        if let Some(scope) = self.scopes.iter_mut().find(|s| s.name == name) {
            scope.expanded = !scope.expanded;
        }
        cx.notify();
    }

    /// Update variable children
    pub fn update_children(
        &mut self,
        reference: i64,
        children: Vec<VariableItem>,
        cx: &mut Context<Self>,
    ) {
        for scope in &mut self.scopes {
            Self::update_variable_children(&mut scope.variables, reference, &children);
        }
        cx.notify();
    }

    fn update_variable_children(
        variables: &mut [VariableItem],
        reference: i64,
        children: &[VariableItem],
    ) {
        for var in variables {
            if var.variables_reference == reference {
                var.children = children.to_vec();
                var.expanded = true;
                return;
            }
            if !var.children.is_empty() {
                Self::update_variable_children(&mut var.children, reference, children);
            }
        }
    }

    /// Render a variable item
    fn render_variable(
        &self,
        var: &VariableItem,
        theme: &crate::app::theme::Theme,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let name = var.name.clone();
        let value = var.display_value();
        let var_type = var.var_type.clone();
        let has_children = var.has_children();
        let expanded = var.expanded;
        let depth = var.depth;
        let reference = var.variables_reference;
        let is_selected = self.selected_variable.as_ref() == Some(&name);

        let text_muted = theme.colors.text_muted;
        let accent_color = theme.colors.accent;
        let text_color = theme.colors.text;
        let surface_color = theme.colors.surface;

        let on_toggle = cx.listener(move |_this, _, _window, cx| {
            if expanded {
                cx.emit(VariablesViewEvent::Collapse(reference));
            } else {
                cx.emit(VariablesViewEvent::Expand(reference));
            }
        });

        let children: Vec<_> = if expanded {
            var.children
                .iter()
                .map(|child| {
                    let mut child = child.clone();
                    child.depth = depth + 1;
                    self.render_variable(&child, theme, cx).into_any_element()
                })
                .collect()
        } else {
            Vec::new()
        };

        div()
            .w_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .id(SharedString::from(format!("var-{}", name)))
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_2()
                    .py_0p5()
                    .ml(px((depth * 16) as f32))
                    .cursor_pointer()
                    .when(is_selected, |d| d.bg(accent_color.opacity(0.1)))
                    .hover(|s| s.bg(surface_color))
                    .when(has_children, |d| {
                        d.child(
                            div()
                                .id(SharedString::from(format!("var-toggle-{}", name)))
                                .w(px(12.0))
                                .h(px(12.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_xs()
                                .text_color(text_muted)
                                .cursor_pointer()
                                .on_click(on_toggle)
                                .child(if expanded { "▼" } else { "▶" }),
                        )
                    })
                    .when(!has_children, |d| d.child(div().w(px(12.0))))
                    .child(
                        div()
                            .text_xs()
                            .font_family("JetBrains Mono")
                            .text_color(accent_color)
                            .child(name.clone()),
                    )
                    .child(div().text_xs().text_color(text_muted).child(":"))
                    .child(
                        div()
                            .flex_1()
                            .text_xs()
                            .font_family("JetBrains Mono")
                            .text_color(text_color)
                            .overflow_hidden()
                            .text_ellipsis()
                            .child(value),
                    )
                    .when(var_type.is_some(), |d| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(text_muted)
                                .child(format!("({})", var_type.as_ref().unwrap())),
                        )
                    }),
            )
            .children(children)
    }
}

impl Render for VariablesView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let scopes = self.scopes.clone();

        let text_muted = theme.colors.text_muted;
        let text_color = theme.colors.text;
        let surface_color = theme.colors.surface;

        div()
            .w_full()
            .flex()
            .flex_col()
            .id("scroll-variables-view")
            .overflow_y_scroll()
            .children(scopes.into_iter().map(|scope| {
                let scope_name = scope.name.clone();
                let scope_name_for_id = scope.name.clone();
                let expanded = scope.expanded;

                let on_scope_toggle = cx.listener(move |this, _, _window, cx| {
                    this.toggle_scope(&scope_name, cx);
                });

                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .id(SharedString::from(format!("scope-{}", scope_name_for_id)))
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py_1()
                            .bg(surface_color)
                            .cursor_pointer()
                            .on_click(on_scope_toggle)
                            .child(div().text_xs().text_color(text_muted).child(if expanded {
                                "▼"
                            } else {
                                "▶"
                            }))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(text_color)
                                    .child(scope.name.clone()),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(text_muted)
                                    .child(format!("({})", scope.variables.len())),
                            ),
                    )
                    .when(expanded, |d| {
                        d.children(
                            scope
                                .variables
                                .iter()
                                .map(|var| self.render_variable(var, &theme, cx)),
                        )
                    })
            }))
            .when(self.scopes.is_empty(), |d| {
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py_4()
                        .text_xs()
                        .text_color(text_muted)
                        .child("No variables available"),
                )
            })
    }
}
