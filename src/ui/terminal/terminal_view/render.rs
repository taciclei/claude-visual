//! Rendering implementation

use gpui::*;
use gpui::prelude::*;
use super::core::TerminalView;
use super::types::default_colors;

impl Render for TerminalView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = default_colors();
        let is_focused = self.focus_handle.is_focused(_window);
        let is_running = self.is_running;

        div()
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e1e)) // Dark terminal background
            .border_1()
            .border_color(if is_focused {
                theme.accent
            } else {
                theme.border
            })
            .rounded_md()
            .overflow_hidden()
            .on_key_down(cx.listener(|this, event, window, cx| {
                this.handle_key(event, window, cx);
            }))
            .child(
                // Terminal header
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py_1()
                    .bg(rgb(0x2d2d2d))
                    .border_b_1()
                    .border_color(rgb(0x404040))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                // Traffic light buttons
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(div().w_3().h_3().rounded_full().bg(rgb(0xFF5F56)))
                                    .child(div().w_3().h_3().rounded_full().bg(rgb(0xFFBD2E)))
                                    .child(div().w_3().h_3().rounded_full().bg(rgb(0x27C93F))),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xCCCCCC))
                                    .child("Terminal"),
                            ),
                    )
                    .child(
                        // Status indicator
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .w_2()
                                    .h_2()
                                    .rounded_full()
                                    .bg(if is_running {
                                        rgb(0x27C93F) // Green
                                    } else {
                                        rgb(0x666666) // Gray
                                    }),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x888888))
                                    .child(if is_running { "Running" } else { "Stopped" }),
                            ),
                    ),
            )
            .child(
                // Terminal content
                div()
                    .flex_1()
                    .p_2()
                    .id("scroll-terminal-content")
                    .overflow_y_scroll()
                    .font_family("Menlo, Monaco, 'Courier New', monospace")
                    .text_sm()
                    .child(self.render_lines(cx)),
            )
            .child(
                // Input line (when focused)
                div()
                    .flex()
                    .items_center()
                    .px_2()
                    .py_1()
                    .bg(rgb(0x2d2d2d))
                    .border_t_1()
                    .border_color(rgb(0x404040))
                    .child(
                        div()
                            .text_color(rgb(0x27C93F))
                            .text_sm()
                            .child("$ "),
                    )
                    .child(
                        div()
                            .flex_1()
                            .text_color(rgb(0xCCCCCC))
                            .text_sm()
                            .child(self.input_buffer.clone()),
                    )
                    .when(is_focused, |el| {
                        el.child(
                            // Cursor
                            div()
                                .w_2()
                                .h_4()
                                .bg(rgb(0xCCCCCC)),
                        )
                    }),
            )
    }
}

impl TerminalView {
    pub(crate) fn render_lines(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        let visible_start = self.scroll_offset;
        let visible_end = (self.scroll_offset + self.size.1 as usize).min(self.lines.len());

        div()
            .flex()
            .flex_col()
            .children(self.lines[visible_start..visible_end].iter().map(|line| {
                div().flex().children(line.spans.iter().map(|span| {
                    let (r, g, b) = span
                        .style
                        .fg_color
                        .as_ref()
                        .map(|c| c.to_rgb())
                        .unwrap_or((204, 204, 204));

                    div()
                        .text_color(rgb((r as u32) << 16 | (g as u32) << 8 | b as u32))
                        .when(span.style.bold, |el| el.font_weight(FontWeight::BOLD))
                        .when(span.style.italic, |el| el.italic())
                        .when(span.style.underline, |el| {
                            el                        })
                        .child(span.text.clone())
                }))
            }))
            .child(
                // Current line being built
                div().flex().children(self.current_line.spans.iter().map(|span| {
                    let (r, g, b) = span
                        .style
                        .fg_color
                        .as_ref()
                        .map(|c| c.to_rgb())
                        .unwrap_or((204, 204, 204));

                    div()
                        .text_color(rgb((r as u32) << 16 | (g as u32) << 8 | b as u32))
                        .when(span.style.bold, |el| el.font_weight(FontWeight::BOLD))
                        .when(span.style.italic, |el| el.italic())
                        .child(span.text.clone())
                })),
            )
    }
}
