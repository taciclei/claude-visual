//! Collapse/expand buttons render functions

use gpui::*;
use gpui::prelude::*;
use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Renders collapse and expand all buttons
    pub fn render_collapse_expand_buttons(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let all_collapsed = self.are_all_collapsed(cx);
        let all_expanded = self.are_all_expanded(cx);

        div()
            .flex()
            .items_center()
            .gap_1()
            // Collapse all button
            .child(
                div()
                    .id("collapse-all")
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .text_xs()
                    .text_color(if all_collapsed {
                        theme.colors.accent
                    } else {
                        theme.colors.text_muted
                    })
                    .hover(|s| s.bg(theme.colors.surface_hover).text_color(theme.colors.text))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.collapse_all(cx);
                    }))
                    .child("▶")
                    .child("Collapse")
                    .child(
                        div()
                            .text_color(theme.colors.text_muted.opacity(0.5))
                            .ml_1()
                            .child("⌘[")
                    )
            )
            // Expand all button
            .child(
                div()
                    .id("expand-all")
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .text_xs()
                    .text_color(if all_expanded {
                        theme.colors.accent
                    } else {
                        theme.colors.text_muted
                    })
                    .hover(|s| s.bg(theme.colors.surface_hover).text_color(theme.colors.text))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.expand_all(cx);
                    }))
                    .child("▼")
                    .child("Expand")
                    .child(
                        div()
                            .text_color(theme.colors.text_muted.opacity(0.5))
                            .ml_1()
                            .child("⌘]")
                    )
            )
    }
}
