//! Split view rendering

use gpui::*;
use gpui::prelude::*;

use super::view::SplitView;
use super::node::SplitNode;
use super::types::SplitDirection;

impl Render for SplitView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .id("split-view")
            .track_focus(&self.focus_handle)
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.colors.background)
            .child(self.render_node(&self.root.clone(), cx))
    }
}

impl SplitView {
    /// Render a split node recursively
    pub(super) fn render_node(&self, node: &SplitNode, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();

        match node {
            SplitNode::Leaf(pane) => {
                let is_focused = pane.is_focused;
                let pane_id = pane.id.clone();

                let accent_color = theme.colors.accent;
                let border_color = theme.colors.border;
                let text_muted = theme.colors.text_muted;

                div()
                    .id(ElementId::Name(format!("pane-{}", pane_id).into()))
                    .flex_1()
                    .min_w(px(200.0))
                    .min_h(px(200.0))
                    .border_2()
                    .when(is_focused, |this| {
                        this.border_color(accent_color)
                    })
                    .when(!is_focused, |this| {
                        this.border_color(border_color)
                    })
                    .rounded_sm()
                    .child(
                        div()
                            .size_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_sm()
                            .text_color(text_muted)
                            .child(if is_focused {
                                "Focused Pane"
                            } else {
                                "Pane"
                            }),
                    )
                    .into_any_element()
            }
            SplitNode::Split { direction, children } => {
                let is_horizontal = *direction == SplitDirection::Horizontal;
                let border_color = theme.colors.border;
                let accent_color = theme.colors.accent;

                let mut container = div()
                    .flex_1()
                    .flex()
                    .gap_1();

                container = if is_horizontal {
                    container.flex_row()
                } else {
                    container.flex_col()
                };

                for (i, child) in children.iter().enumerate() {
                    if i > 0 {
                        // Divider
                        container = container.child(
                            div()
                                .id(ElementId::Name(format!("divider-{}", i).into()))
                                .when(is_horizontal, |this| {
                                    this.w(px(4.0))
                                        .h_full()
                                        .cursor(CursorStyle::ResizeLeftRight)
                                })
                                .when(!is_horizontal, |this| {
                                    this.h(px(4.0))
                                        .w_full()
                                        .cursor(CursorStyle::ResizeUpDown)
                                })
                                .bg(border_color)
                                .hover(|style| style.bg(accent_color)),
                        );
                    }
                    container = container.child(self.render_node(child, cx));
                }

                container.into_any_element()
            }
        }
    }
}
