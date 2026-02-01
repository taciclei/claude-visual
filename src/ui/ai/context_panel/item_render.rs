//! Context Panel Item Rendering

use gpui::*;
use gpui::prelude::*;

use crate::ai::context::ContextItem;

use super::core::ContextPanel;
use super::types::{SimpleColors, ContextPanelEvent};

impl ContextPanel {
    pub(crate) fn render_content(&self, theme: &SimpleColors, cx: &mut Context<Self>) -> impl IntoElement {
        let items = self.context.items();

        div()
            .flex()
            .flex_col()
            .max_h_64()
            .id("scroll-context-panel")
            .overflow_y_scroll()
            .child(
                // Action bar
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(theme.border)
                    .child(
                        // Add file button
                        div()
                            .id("context-add-file-button")
                            .px_2()
                            .py_1()
                            .text_xs()
                            .text_color(theme.text)
                            .bg(theme.surface_hover)
                            .rounded_sm()
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.accent.opacity(0.2)))
                            .on_click(cx.listener(|_this, _, _window, cx| {
                                cx.emit(ContextPanelEvent::AttachFileRequested);
                            }))
                            .child("+ File"),
                    )
                    .child(
                        // Add snippet button
                        div()
                            .id("context-add-snippet-button")
                            .px_2()
                            .py_1()
                            .text_xs()
                            .text_color(theme.text)
                            .bg(theme.surface_hover)
                            .rounded_sm()
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.accent.opacity(0.2)))
                            .on_click(cx.listener(|_this, _, _window, cx| {
                                cx.emit(ContextPanelEvent::AttachSnippetRequested);
                            }))
                            .child("+ Snippet"),
                    )
                    .child(div().flex_1()) // Spacer
                    .when(!items.is_empty(), |el| {
                        el.child(
                            // Clear all button
                            div()
                                .id("context-clear-all-button")
                                .px_2()
                                .py_1()
                                .text_xs()
                                .text_color(theme.error)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.error.opacity(0.1)))
                                .rounded_sm()
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.clear_all(cx);
                                }))
                                .child("Clear All"),
                        )
                    }),
            )
            .when(items.is_empty(), |el| {
                el.child(
                    // Empty state
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .py_8()
                        .gap_2()
                        .child(
                            div()
                                .text_2xl()
                                .text_color(theme.text_muted)
                                .child("📎"),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(theme.text_muted)
                                .child("No context attached"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child("Use @file or drag files to add context"),
                        ),
                )
            })
            .children(items.iter().map(|item| {
                self.render_item(item, &theme, cx)
            }))
    }

    pub(crate) fn render_item(&self, item: &ContextItem, theme: &SimpleColors, cx: &mut Context<Self>) -> impl IntoElement {
        let item_id = item.id.clone();
        let item_id_for_pin = item.id.clone();
        let item_id_for_remove = item.id.clone();
        let is_selected = self.selected_item_id.as_deref() == Some(&item.id);
        let is_pinned = item.pinned;

        div()
            .id(SharedString::from(format!("context-item-{}", item.id)))
            .flex()
            .items_center()
            .gap_2()
            .px_3()
            .py_2()
            .border_b_1()
            .border_color(theme.border.opacity(0.5))
            .cursor_pointer()
            .bg(if is_selected {
                theme.accent.opacity(0.1)
            } else {
                theme.surface
            })
            .hover(|s| s.bg(theme.surface_hover))
            .on_click(cx.listener(move |this, _, _window, cx| {
                let new_selection = if this.selected_item_id.as_deref() == Some(&item_id) {
                    None
                } else {
                    Some(item_id.clone())
                };
                this.select_item(new_selection, cx);
            }))
            .child(
                // Type icon
                div()
                    .text_sm()
                    .child(Self::icon_for_type(&item.item_type)),
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.text)
                                    .text_ellipsis()
                                    .child(item.name.clone()),
                            )
                            .when(is_pinned, |el| {
                                el.child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.warning)
                                        .child("📌"),
                                )
                            }),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(format!("{} tokens", item.token_count)),
                    ),
            )
            .child(
                // Pin button
                div()
                    .id(SharedString::from(format!("context-pin-{}", item.id)))
                    .px_1()
                    .py_px()
                    .text_xs()
                    .text_color(if is_pinned {
                        theme.warning
                    } else {
                        theme.text_muted
                    })
                    .cursor_pointer()
                    .hover(|s| s.text_color(theme.warning))
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        this.toggle_pin(&item_id_for_pin, cx);
                    }))
                    .child(if is_pinned { "📌" } else { "⚲" }),
            )
            .child(
                // Remove button
                div()
                    .id(SharedString::from(format!("context-remove-{}", item.id)))
                    .px_1()
                    .py_px()
                    .text_xs()
                    .text_color(theme.text_muted)
                    .cursor_pointer()
                    .hover(|s| s.text_color(theme.error))
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        this.remove_item(&item_id_for_remove, cx);
                    }))
                    .child("✕"),
            )
    }
}
