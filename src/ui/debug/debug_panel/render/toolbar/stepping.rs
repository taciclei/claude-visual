//! Debug stepping controls

use gpui::*;
use gpui::prelude::*;

use crate::ui::debug::debug_panel::{DebugPanel, DebugPanelEvent};

impl DebugPanel {
    /// Render continue button
    pub(super) fn render_continue_button(
        &self,
        is_stopped: bool,
        success_color: Hsla,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        // Extract listener before div chain
        let on_continue = cx.listener(|_this, _, _window, cx| {
            cx.emit(DebugPanelEvent::Continue);
        });

        div()
            .id("debug-continue")
            .w(px(28.0))
            .h(px(28.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .cursor_pointer()
            .when(is_stopped, |d| {
                d.bg(success_color.opacity(0.2))
                    .hover(|s| s.bg(success_color.opacity(0.3)))
                    .on_click(on_continue)
            })
            .when(!is_stopped, |d| {
                d.opacity(0.5)
            })
            .child(
                div()
                    .text_sm()
                    .text_color(success_color)
                    .child("▶▶"),
            )
    }

    /// Render pause button
    pub(super) fn render_pause_button(
        &self,
        is_running: bool,
        warning_color: Hsla,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        // Extract listener before div chain
        let on_pause = cx.listener(|_this, _, _window, cx| {
            cx.emit(DebugPanelEvent::Pause);
        });

        div()
            .id("debug-pause")
            .w(px(28.0))
            .h(px(28.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .cursor_pointer()
            .when(is_running, |d| {
                d.bg(warning_color.opacity(0.2))
                    .hover(|s| s.bg(warning_color.opacity(0.3)))
                    .on_click(on_pause)
            })
            .when(!is_running, |d| {
                d.opacity(0.5)
            })
            .child(
                div()
                    .text_sm()
                    .text_color(warning_color)
                    .child("⏸"),
            )
    }

    /// Render step over button
    pub(super) fn render_step_over_button(
        &self,
        is_stopped: bool,
        surface_color: Hsla,
        border_color: Hsla,
        text_color: Hsla,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        // Extract listener before div chain
        let on_step_over = cx.listener(|_this, _, _window, cx| {
            cx.emit(DebugPanelEvent::StepOver);
        });

        div()
            .id("debug-step-over")
            .w(px(28.0))
            .h(px(28.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .cursor_pointer()
            .when(is_stopped, |d| {
                d.bg(surface_color)
                    .hover(|s| s.bg(border_color))
                    .on_click(on_step_over)
            })
            .when(!is_stopped, |d| {
                d.opacity(0.5)
            })
            .child(
                div()
                    .text_xs()
                    .text_color(text_color)
                    .child("⤵"),
            )
    }

    /// Render step into button
    pub(super) fn render_step_into_button(
        &self,
        is_stopped: bool,
        surface_color: Hsla,
        border_color: Hsla,
        text_color: Hsla,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        // Extract listener before div chain
        let on_step_into = cx.listener(|_this, _, _window, cx| {
            cx.emit(DebugPanelEvent::StepInto);
        });

        div()
            .id("debug-step-into")
            .w(px(28.0))
            .h(px(28.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .cursor_pointer()
            .when(is_stopped, |d| {
                d.bg(surface_color)
                    .hover(|s| s.bg(border_color))
                    .on_click(on_step_into)
            })
            .when(!is_stopped, |d| {
                d.opacity(0.5)
            })
            .child(
                div()
                    .text_xs()
                    .text_color(text_color)
                    .child("↓"),
            )
    }

    /// Render step out button
    pub(super) fn render_step_out_button(
        &self,
        is_stopped: bool,
        surface_color: Hsla,
        border_color: Hsla,
        text_color: Hsla,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        // Extract listener before div chain
        let on_step_out = cx.listener(|_this, _, _window, cx| {
            cx.emit(DebugPanelEvent::StepOut);
        });

        div()
            .id("debug-step-out")
            .w(px(28.0))
            .h(px(28.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .cursor_pointer()
            .when(is_stopped, |d| {
                d.bg(surface_color)
                    .hover(|s| s.bg(border_color))
                    .on_click(on_step_out)
            })
            .when(!is_stopped, |d| {
                d.opacity(0.5)
            })
            .child(
                div()
                    .text_xs()
                    .text_color(text_color)
                    .child("↑"),
            )
    }
}
