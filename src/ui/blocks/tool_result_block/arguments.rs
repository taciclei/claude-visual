//! Arguments section rendering for tool result blocks

use gpui::*;
use gpui::prelude::*;

use super::types::ToolResultBlock;

impl ToolResultBlock {
    pub(super) fn render_arguments(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();
        let args_expanded = self.args_expanded;
        let result_clone = self.result.clone();
        let theme_colors = theme.colors.clone();

        // Extract listener before div chain
        let on_args_click = cx.listener(|this, _, _window, cx| {
            this.toggle_args_expanded(cx);
        });

        div().when_some(result_clone.arguments.clone(), |d, args| {
            // Clone for move closure
            let theme_colors_inner = theme_colors.clone();

            d.child(
                div()
                    .border_b_1()
                    .border_color(theme_colors.border)
                    // Args header
                    .child(
                        div()
                            .id("args-header")
                            .px_3()
                            .py_1()
                            .flex()
                            .items_center()
                            .gap_2()
                            .cursor_pointer()
                            .hover(|style| style.bg(theme_colors_inner.surface_hover))
                            .on_click(on_args_click)
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme_colors_inner.text_muted)
                                    .child(if args_expanded { "▾" } else { "▸" }),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme_colors_inner.text_muted)
                                    .child("Arguments"),
                            ),
                    )
                    // Args content
                    .when(args_expanded, |d| {
                        d.child(
                            div()
                                .px_3()
                                .py_2()
                                .bg(theme_colors_inner.background)
                                .text_xs()
                                .font_family("JetBrains Mono")
                                .text_color(theme_colors_inner.text_muted)
                                .whitespace_nowrap()
                                .max_h(px(150.0))
                                .id("scroll-tool-args")
                                .overflow_y_scroll()
                                .child(self.format_json(&args)),
                        )
                    }),
            )
        })
    }
}
