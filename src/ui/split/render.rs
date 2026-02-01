//! Rendering implementation for SplitContainer

use gpui::*;
use gpui::prelude::*;

use super::container::SplitContainer;
use super::types::*;

impl Render for SplitContainer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .id("split-container")
            .track_focus(&self.focus_handle)
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.colors.background)
            .child(self.render_node(&self.root.clone(), cx))
    }
}

impl SplitContainer {
    /// Render a split node recursively
    pub(crate) fn render_node(&self, node: &SplitNode, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();

        match node {
            SplitNode::Pane(pane) => {
                let is_focused = pane.is_focused;
                let pane_id = pane.id.clone();
                let weight = pane.weight;

                div()
                    .id(ElementId::Name(format!("pane-{}", pane_id).into()))
                    .flex_1()
                    .min_w(px(150.0))
                    .min_h(px(150.0))
                    .border_1()
                    .when(is_focused, |this| {
                this.border_color(theme.colors.accent)
                    })
                    .when(!is_focused, |this| {
                this.border_color(theme.colors.border)
                    })
                    .rounded_sm()
                    // Placeholder content - would contain ChatView
                    .child(
                        div()
                            .size_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_sm()
                            .text_color(theme.colors.text_muted)
                            .child(if is_focused {
                                format!("Focused Pane ({}%)", (weight * 100.0) as i32)
                            } else {
                                format!("Pane ({}%)", (weight * 100.0) as i32)
                            }),
                    )
                    .into_any_element()
            }
            SplitNode::Split { direction, children } => {
                let is_horizontal = *direction == SplitDirection::Horizontal;
                let dir = *direction;

                let mut container = div()
                    .flex_1()
                    .flex()
                    .gap_0();

                container = if is_horizontal {
                    container.flex_row()
                } else {
                    container.flex_col()
                };

                // Add children with dividers between them
                for (i, child) in children.iter().enumerate() {
                    if i > 0 {
                        let divider_idx = i - 1;
                        let is_dragging = self.resize_drag.as_ref()
                            .map(|d| d.divider_index == divider_idx)
                            .unwrap_or(false);

                        // Add resize divider with drag handlers
                        container = container.child(
                            div()
                                .id(ElementId::Name(format!("divider-{}", i).into()))
                                .when(is_horizontal, |this| {
                this.w(px(6.0))
                                        .h_full()
                                        .cursor(CursorStyle::ResizeLeftRight)
                                })
                                .when(!is_horizontal, |this| {
                this.h(px(6.0))
                                        .w_full()
                                        .cursor(CursorStyle::ResizeUpDown)
                                })
                                .bg(if is_dragging {
                                    theme.colors.accent
                                } else {
                                    theme.colors.border
                                })
                                .hover(|style| style.bg(theme.colors.accent))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, event: &MouseDownEvent, _window, cx| {
                                        this.start_resize_drag(dir, divider_idx, event.position, cx);
                                    }),
                                )
                                .on_mouse_up(
                                    MouseButton::Left,
                                    cx.listener(|this, _, _window, cx| {
                                        this.end_resize_drag(cx);
                                    }),
                                )
                                .on_mouse_move(
                                    cx.listener(|this, event: &MouseMoveEvent, _window, cx| {
                                        if this.resize_drag.is_some() {
                                            this.update_resize_drag(event.position, cx);
                                        }
                                    }),
                                ),
                        );
                    }
                    container = container.child(self.render_node(child, cx));
                }

                container.into_any_element()
            }
        }
    }
}
