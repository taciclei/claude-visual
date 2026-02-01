//! Context Panel Main Rendering

use gpui::*;
use gpui::prelude::*;

use super::core::ContextPanel;
use super::types::default_colors;

impl Render for ContextPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = default_colors();
        let is_expanded = self.is_expanded;
        let items = self.context.items();
        let token_count = self.context.token_count();
        let remaining = self.context.remaining_tokens();
        let show_tokens = self.show_token_counts;

        div()
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .bg(theme.surface)
            .border_1()
            .border_color(theme.border)
            .rounded_md()
            .overflow_hidden()
            .child(
                // Header
                div()
                    .id("context-panel-header")
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py_2()
                    .bg(theme.background)
                    .border_b_1()
                    .border_color(theme.border)
                    .cursor_pointer()
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.toggle_expanded(cx);
                    }))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_color(theme.text_muted)
                                    .child(if is_expanded { "▼" } else { "▶" }),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.text)
                                    .child("Context"),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_px()
                                    .bg(theme.accent.opacity(0.2))
                                    .rounded_full()
                                    .text_xs()
                                    .text_color(theme.accent)
                                    .child(format!("{}", items.len())),
                            ),
                    )
                    .when(show_tokens, |el| {
                        el.child(
                            // Token usage indicator
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(format!("{}K / {}K tokens", token_count / 1000, (token_count + remaining) / 1000))
                                .child(
                                    // Progress bar
                                    div()
                                        .w_16()
                                        .h_1()
                                        .bg(theme.border)
                                        .rounded_full()
                                        .child(
                                            div()
                                                .h_full()
                                                .rounded_full()
                                                .bg(if token_count > remaining {
                                                    theme.error
                                                } else if token_count * 2 > remaining {
                                                    theme.warning
                                                } else {
                                                    theme.success
                                                })
                                                .w(relative(token_count as f32 / (token_count + remaining) as f32)),
                                        ),
                                ),
                        )
                    }),
            )
            .when(is_expanded, |el| {
                el.child(self.render_content(&theme, cx))
            })
    }
}
