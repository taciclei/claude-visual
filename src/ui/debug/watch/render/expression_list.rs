//! Expression list rendering

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;
use super::super::core::WatchView;
use super::super::events::WatchViewEvent;
use super::super::WatchExpression;

impl WatchView {
    pub fn render_expression_list(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let expressions = self.expressions.clone();
        let selected_id = self.selected_id;

        div()
            .flex_1()
            .id("scroll-watch-expressions")
            .overflow_y_scroll()
            .children(expressions.into_iter().map(|expr| {
                self.render_expression(expr, selected_id, theme, cx)
            }))
    }

    fn render_expression(
        &self,
        expr: WatchExpression,
        selected_id: Option<usize>,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let id = expr.id;
        let is_selected = selected_id == Some(id);
        let has_error = expr.has_error();
        let is_evaluating = expr.is_evaluating;
        let has_children = expr.has_children();
        let expanded = expr.expanded;

        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;
        let accent = theme.colors.accent;
        let error = theme.colors.error;
        let surface = theme.colors.surface;

        let on_select = cx.listener(move |this, _, _window, cx| {
            this.selected_id = Some(id);
            cx.notify();
        });

        let on_toggle = cx.listener(move |this, _, _window, cx| {
            this.toggle_expand(id, cx);
        });

        let on_refresh = cx.listener(move |_this, _, _window, cx| {
            cx.emit(WatchViewEvent::Refresh(id));
        });

        let on_remove = cx.listener(move |this, _, _window, cx| {
            this.remove_expression(id, cx);
            cx.emit(WatchViewEvent::Remove(id));
        });

        div()
            .w_full()
            .flex()
            .flex_col()
            // Main row
            .child(
                div()
                    .id(SharedString::from(format!("watch-{}", id)))
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_2()
                    .py_1()
                    .cursor_pointer()
                    .when(is_selected, |d| d.bg(accent.opacity(0.1)))
                    .hover(|s| s.bg(surface))
                    .on_click(on_select)
                    // Expand toggle
                    .when(has_children, |d| {
                        d.child(
                            div()
                                .id(SharedString::from(format!("watch-toggle-{}", id)))
                                .w(px(12.0))
                                .text_xs()
                                .text_color(text_muted)
                                .cursor_pointer()
                                .on_click(on_toggle)
                                .child(if expanded { "▼" } else { "▶" }),
                        )
                    })
                    .when(!has_children, |d| d.child(div().w(px(12.0))))
                    // Expression
                    .child(
                        div()
                            .text_xs()
                            .font_family("JetBrains Mono")
                            .text_color(accent)
                            .child(expr.expression.clone()),
                    )
                    // Separator
                    .child(
                        div()
                            .text_xs()
                            .text_color(text_muted)
                            .child("="),
                    )
                    // Value
                    .child(
                        div()
                            .flex_1()
                            .text_xs()
                            .font_family("JetBrains Mono")
                            .when(has_error, |d| d.text_color(error))
                            .when(!has_error && !is_evaluating, |d| d.text_color(text_color))
                            .when(is_evaluating, |d| d.text_color(text_muted))
                            .overflow_hidden()
                            .text_ellipsis()
                            .child(expr.display_value()),
                    )
                    // Type
                    .when(expr.value_type.is_some(), |d| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(text_muted)
                                .child(format!("({})", expr.value_type.as_ref().unwrap())),
                        )
                    })
                    // Refresh button
                    .child(
                        div()
                            .id(SharedString::from(format!("watch-refresh-{}", id)))
                            .text_xs()
                            .text_color(text_muted)
                            .cursor_pointer()
                            .hover(|s| s.text_color(text_color))
                            .on_click(on_refresh)
                            .child("↻"),
                    )
                    // Remove button
                    .child(
                        div()
                            .id(SharedString::from(format!("watch-remove-{}", id)))
                            .text_xs()
                            .text_color(text_muted)
                            .cursor_pointer()
                            .hover(|s| s.text_color(error))
                            .on_click(on_remove)
                            .child("×"),
                    ),
            )
            // Children (when expanded)
            .when(expanded, |d| {
                d.children(expr.children.iter().map(|child| {
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .px_2()
                        .py_0p5()
                        .ml(px(24.0))
                        // Name
                        .child(
                            div()
                                .text_xs()
                                .font_family("JetBrains Mono")
                                .text_color(text_muted)
                                .child(child.name.clone()),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(text_muted)
                                .child(":"),
                        )
                        // Value
                        .child(
                            div()
                                .flex_1()
                                .text_xs()
                                .font_family("JetBrains Mono")
                                .text_color(text_color)
                                .overflow_hidden()
                                .text_ellipsis()
                                .child(child.value.clone()),
                        )
                        // Type
                        .when(child.value_type.is_some(), |d| {
                            d.child(
                                div()
                                    .text_xs()
                                    .text_color(text_muted)
                                    .child(format!("({})", child.value_type.as_ref().unwrap())),
                            )
                        })
                }))
            })
    }
}
