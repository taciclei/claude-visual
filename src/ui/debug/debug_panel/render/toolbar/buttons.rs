//! Primary debug control buttons

use gpui::*;
use gpui::prelude::*;

use crate::ui::debug::debug_panel::{DebugPanel, DebugPanelEvent};
use crate::debug::DebugState;

impl DebugPanel {
    /// Render start/stop button
    pub(super) fn render_start_stop_button(
        &self,
        is_idle: bool,
        success_color: Hsla,
        error_color: Hsla,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        // Extract listeners before div chains
        let on_start = cx.listener(|this, _, _window, cx| {
            cx.emit(DebugPanelEvent::Start);
            this.state = DebugState::Initializing;
            cx.notify();
        });
        let on_stop = cx.listener(|_this, _, _window, cx| {
            cx.emit(DebugPanelEvent::Stop);
        });

        div()
            .id("debug-start-stop")
            .w(px(28.0))
            .h(px(28.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .cursor_pointer()
            .when(is_idle, |d| {
                d.bg(success_color.opacity(0.2))
                    .hover(|s| s.bg(success_color.opacity(0.3)))
                    .on_click(on_start)
                    .child(
                        div()
                            .text_sm()
                            .text_color(success_color)
                            .child("▶"),
                    )
            })
            .when(!is_idle, |d| {
                d.bg(error_color.opacity(0.2))
                    .hover(|s| s.bg(error_color.opacity(0.3)))
                    .on_click(on_stop)
                    .child(
                        div()
                            .text_sm()
                            .text_color(error_color)
                            .child("⏹"),
                    )
            })
    }

    /// Render restart button
    pub(super) fn render_restart_button(
        &self,
        surface_color: Hsla,
        border_color: Hsla,
        warning_color: Hsla,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        // Extract listener before div chain
        let on_restart = cx.listener(|_this, _, _window, cx| {
            cx.emit(DebugPanelEvent::Restart);
        });

        div()
            .id("debug-restart")
            .w(px(28.0))
            .h(px(28.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .cursor_pointer()
            .bg(surface_color)
            .hover(|s| s.bg(border_color))
            .on_click(on_restart)
            .child(
                div()
                    .text_sm()
                    .text_color(warning_color)
                    .child("↻"),
            )
    }
}
