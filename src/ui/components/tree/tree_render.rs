//! Tree rendering logic

use gpui::*;
use gpui::prelude::*;
use super::tree_state::Tree;
use super::types::{TreeNode, TreeEvent};

impl Tree {
    /// Render a node recursively
    pub(super) fn render_node(
        &mut self,
        node: &TreeNode,
        depth: usize,
        cx: &mut Context<Self>,
        theme: &crate::app::theme::Theme,
    ) -> Div {
        let is_expanded = self.is_expanded(&node.id);
        let is_selected = self.selected.as_deref() == Some(&node.id)
            || self.selected_multiple.contains(&node.id);
        let has_children = node.has_children();
        let opacity = if node.disabled { 0.5 } else { 1.0 };

        let node_id = node.id.clone();
        let chevron = if has_children {
            if is_expanded { "▼" } else { "▶" }
        } else {
            " "
        };

        let icon = node.icon.clone().unwrap_or_else(|| {
            if has_children {
                if is_expanded { "📂".to_string() } else { "📁".to_string() }
            } else {
                "📄".to_string()
            }
        });

        // Copy theme colors for move closures
        let surface_hover = theme.colors.surface_hover;
        let accent_color = theme.colors.accent;
        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;

        // Extract listeners before div chains
        let node_click_listener = cx.listener({
            let node_id = node_id.clone();
            move |this, _, _window, cx| {
                this.select(&node_id, cx);
            }
        });

        let chevron_click_listener = cx.listener({
            let node_id = node_id.clone();
            move |this, _, _window, cx| {
                this.toggle(&node_id, cx);
            }
        });

        div()
            .w_full()
            .flex()
            .flex_col()
            // Node row
            .child(
                div()
                    .id(SharedString::from(format!("tree-node-{}", node.id)))
                    .h(px(28.0))
                    .w_full()
                    .pl(px(depth as f32 * self.indent))
                    .pr_2()
                    .flex()
                    .items_center()
                    .gap_1()
                    .rounded(px(4.0))
                    .opacity(opacity)
                    .when(is_selected, |d| {
                        d.bg(accent_color.opacity(0.15))
                    })
                    .when(!node.disabled, |d| {
                        d.cursor_pointer()
                            .hover(|s| s.bg(surface_hover))
                            .on_click(node_click_listener)
                    })
                    // Chevron
                    .child(
                        div()
                            .id(SharedString::from(format!("tree-chevron-{}", node.id)))
                            .w(px(16.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_xs()
                            .text_color(text_muted)
                            .when(has_children && !node.disabled, |d| {
                                d.cursor_pointer()
                                    .on_click(chevron_click_listener)
                            })
                            .child(chevron)
                    )
                    // Icon
                    .child(
                        div()
                            .text_sm()
                            .child(icon)
                    )
                    // Label
                    .child(
                        div()
                            .flex_1()
                            .text_sm()
                            .text_color(if is_selected {
                                accent_color
                            } else {
                                text_color
                            })
                            .overflow_hidden()
                            .text_ellipsis()
                            .child(node.label.clone())
                    )
            )
            // Children (if expanded)
            .when(is_expanded && has_children, |d| {
                d.child(
                    div()
                        .w_full()
                        .flex()
                        .flex_col()
                        .children(node.children.iter().map(|child| {
                            self.render_node(child, depth + 1, cx, theme)
                        }))
                )
            })
    }
}

impl EventEmitter<TreeEvent> for Tree {}

impl Focusable for Tree {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Tree {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_clone = {
            let theme = self.app_state.theme.read(cx);
            theme.clone()
        };
        let theme = &theme_clone;

        // Clone nodes for iteration
        let nodes = self.nodes.clone();

        div()
            .id("tree")
            .w_full()
            .flex()
            .flex_col()
            .track_focus(&self.focus_handle)
            .children(nodes.iter().map(|node| {
                self.render_node(node, 0, cx, &theme)
            }))
    }
}
