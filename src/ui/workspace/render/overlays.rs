//! Overlay rendering: command palette, settings, shortcuts, diff preview, focus indicator, toasts

use gpui::*;
use gpui::prelude::*;
use crate::app::theme::Theme;
use crate::ui::pct;
use super::super::core::Workspace;

pub fn render_overlays(workspace: &Workspace, theme: &Theme, cx: &mut Context<Workspace>) -> Vec<Div> {
    let mut overlays = Vec::new();

    // Command palette overlay
    if let Some(palette) = &workspace.command_palette {
        overlays.push(div().child(palette.clone()));
    }

    // Settings modal overlay
    if let Some(modal) = &workspace.settings_modal {
        overlays.push(div().child(modal.clone()));
    }

    // Shortcuts panel overlay
    if let Some(panel) = &workspace.shortcuts_panel {
        overlays.push(div().child(panel.clone()));
    }

    // Diff preview overlay
    if let Some((path, diff)) = &workspace.diff_preview {
        overlays.push(div().child(workspace.render_diff_preview(path, diff, theme, cx)));
    }

    // Focus mode indicator
    if workspace.focus_mode {
        overlays.push(render_focus_indicator(theme));
    }

    // Toast notifications (always present)
    overlays.push(div().child(workspace.toast_container.clone()));

    overlays
}

fn render_focus_indicator(theme: &Theme) -> Div {
    div()
        .absolute()
        .bottom_4()
        .left_1_2()
        .px_4()
        .py_2()
        .rounded_lg()
        .bg(theme.colors.surface.opacity(0.9))
        .border_1()
        .border_color(theme.colors.border)
        .shadow_lg()
        .flex()
        .items_center()
        .gap_2()
        .child(
            div()
                .text_sm()
                .text_color(theme.colors.text_muted)
                .child("Focus Mode")
        )
        .child(
            div()
                .text_xs()
                .px_2()
                .py_0p5()
                .rounded_sm()
                .bg(theme.colors.accent.opacity(0.2))
                .text_color(theme.colors.accent)
                .child("⇧⌘F to exit")
        )
}
