//! Call Stack View Rendering
//!
//! Rendering logic for call stack view.

use gpui::prelude::*;
use gpui::*;

use super::component::CallStackView;
use super::types::{CallStackViewEvent, StackFrameItem};

impl CallStackView {
    /// Render a stack frame
    pub(super) fn render_frame(
        &self,
        thread_id: i64,
        frame: &StackFrameItem,
        theme: &crate::app::theme::Theme,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let frame_id = frame.id;
        let is_current = frame.is_current;
        let name = frame.display_name().to_string();
        let location = frame.short_location();

        // Copy theme colors for move closure
        let hover_bg = theme.colors.surface;
        let accent_color = theme.colors.accent;
        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;

        // Extract listener before div chain
        let on_click_listener = cx.listener(move |_this, _, _window, cx| {
            cx.emit(CallStackViewEvent::SelectFrame {
                thread_id,
                frame_id,
            });
        });

        div()
            .id(SharedString::from(format!(
                "frame-{}-{}",
                thread_id, frame_id
            )))
            .flex()
            .items_center()
            .gap_2()
            .px_2()
            .py_1()
            .ml(px(16.0))
            .cursor_pointer()
            .when(is_current, |d| {
                d.bg(accent_color.opacity(0.15))
                    .border_l_2()
                    .border_color(accent_color)
            })
            .hover(move |s| s.bg(hover_bg))
            .on_click(on_click_listener)
            // Current indicator
            .when(is_current, |d| {
                d.child(div().text_xs().text_color(accent_color).child("→"))
            })
            .when(!is_current, |d| d.child(div().w(px(12.0))))
            // Function name
            .child(
                div()
                    .flex_1()
                    .text_xs()
                    .font_family("JetBrains Mono")
                    .text_color(text_color)
                    .overflow_hidden()
                    .text_ellipsis()
                    .child(name),
            )
            // Location
            .child(div().text_xs().text_color(text_muted).child(location))
    }
}

impl Render for CallStackView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let threads = self.threads.clone();

        // Copy theme colors for move closures
        let surface_bg = theme.colors.surface;
        let border_color = theme.colors.border;
        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;
        let accent_color = theme.colors.accent;

        // Extract listeners before div chains
        let refresh_listener = cx.listener(|_this, _, _window, cx| {
            cx.emit(CallStackViewEvent::Refresh);
        });

        let copy_listener = cx.listener(|_this, _, _window, cx| {
            cx.emit(CallStackViewEvent::CopyStackTrace);
        });

        div()
            .w_full()
            .flex()
            .flex_col()
            // Header
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_2()
                    .py_1()
                    .bg(surface_bg)
                    .border_b_1()
                    .border_color(border_color)
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(text_color)
                            .child(format!("Call Stack ({} threads)", threads.len())),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            // Refresh button
                            .child(
                                div()
                                    .id("refresh-stack")
                                    .px_2()
                                    .py_0p5()
                                    .rounded_sm()
                                    .text_xs()
                                    .text_color(text_muted)
                                    .cursor_pointer()
                                    .hover(move |s| s.bg(border_color))
                                    .on_click(refresh_listener)
                                    .child("↻"),
                            )
                            // Copy button
                            .child(
                                div()
                                    .id("copy-stack")
                                    .px_2()
                                    .py_0p5()
                                    .rounded_sm()
                                    .text_xs()
                                    .text_color(text_muted)
                                    .cursor_pointer()
                                    .hover(move |s| s.bg(border_color))
                                    .on_click(copy_listener)
                                    .child("📋"),
                            ),
                    ),
            )
            // Threads and frames
            .child(
                div()
                    .flex_1()
                    .id("scroll-call-stack")
                    .overflow_y_scroll()
                    .children(threads.into_iter().map(|thread| {
                        let thread_id = thread.id;
                        let expanded = thread.expanded;
                        let is_current = thread.is_current;

                        // Copy theme colors for move closure
                        let hover_bg = surface_bg;
                        let accent_bg = accent_color;
                        let thread_text_color = text_color;
                        let thread_text_muted = text_muted;
                        let thread_accent_color = accent_color;

                        // Extract listener before div chain
                        let toggle_listener = cx.listener(move |this, _, _window, cx| {
                            this.toggle_thread(thread_id, cx);
                        });

                        div()
                            .w_full()
                            .flex()
                            .flex_col()
                            // Thread header
                            .child(
                                div()
                                    .id(SharedString::from(format!("thread-{}", thread_id)))
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py_1()
                                    .cursor_pointer()
                                    .when(is_current, move |d| d.bg(accent_bg.opacity(0.05)))
                                    .hover(move |s| s.bg(hover_bg))
                                    .on_click(toggle_listener)
                                    // Expand toggle
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(thread_text_muted)
                                            .child(if expanded { "▼" } else { "▶" }),
                                    )
                                    // Thread icon
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(if is_current {
                                                thread_accent_color
                                            } else {
                                                thread_text_muted
                                            })
                                            .child("⚡"),
                                    )
                                    // Thread name
                                    .child(
                                        div()
                                            .flex_1()
                                            .text_xs()
                                            .font_weight(if is_current {
                                                FontWeight::MEDIUM
                                            } else {
                                                FontWeight::NORMAL
                                            })
                                            .text_color(thread_text_color)
                                            .child(thread.name.clone()),
                                    )
                                    // Frame count
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(thread_text_muted)
                                            .child(format!("{} frames", thread.frame_count())),
                                    ),
                            )
                            // Stack frames
                            .when(expanded, |d| {
                                d.children(
                                    thread.frames.iter().map(|frame| {
                                        self.render_frame(thread_id, frame, &theme, cx)
                                    }),
                                )
                            })
                    })),
            )
            // Empty state
            .when(self.threads.is_empty(), |d| {
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py_4()
                        .text_xs()
                        .text_color(text_muted)
                        .child("No call stack available"),
                )
            })
    }
}
