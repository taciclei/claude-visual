//! Main content area rendering with tab bar and split mode

use super::super::core::Workspace;
use crate::app::theme::Theme;
use crate::ui::chat::view::ChatView;
use gpui::prelude::*;
use gpui::*;

pub fn render_content(
    workspace: &Workspace,
    theme: &Theme,
    active_chat_view: Option<Entity<ChatView>>,
    cx: &mut Context<Workspace>,
) -> impl IntoElement {
    div()
        .id("main-content")
        .track_focus(&workspace.main_focus)
        .flex_1()
        .h_full()
        .flex()
        .flex_col()
        // Tab bar (hidden in split mode or focus mode)
        .when(!workspace.split_mode && !workspace.focus_mode, |this| {
            this.child(workspace.tab_bar.clone())
        })
        // Split mode header
        .when(workspace.split_mode, |this| {
            this.child(render_split_header(workspace, theme, cx))
        })
        // Split container or active chat view
        .when(workspace.split_mode, |this| {
            if let Some(container) = &workspace.split_container {
                this.child(container.clone())
            } else {
                this.child(div())
            }
        })
        .when(!workspace.split_mode, |this| {
            if let Some(chat_view) = active_chat_view.clone() {
                this.child(chat_view)
            } else {
                this.child(div())
            }
        })
        // Status bar at bottom (hidden in focus mode)
        .when(!workspace.focus_mode, |this| {
            this.child(workspace.status_bar.clone())
        })
}

fn render_split_header(workspace: &Workspace, theme: &Theme, cx: &mut Context<Workspace>) -> Div {
    let pane_count = workspace
        .split_container
        .as_ref()
        .map(|c| c.read(cx).pane_count())
        .unwrap_or(1);

    div()
        .h(px(36.0))
        .px_4()
        .flex()
        .items_center()
        .justify_between()
        .border_b_1()
        .border_color(theme.colors.border)
        .bg(theme.colors.surface)
        .child(render_split_info(pane_count, theme))
        .child(render_split_controls(theme, cx))
}

fn render_split_info(pane_count: usize, theme: &Theme) -> Div {
    div()
        .flex()
        .items_center()
        .gap_2()
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(theme.colors.text)
                .child("Split View"),
        )
        .child(
            div()
                .text_xs()
                .text_color(theme.colors.text_muted)
                .child(format!("{} panes", pane_count)),
        )
}

fn render_split_controls(theme: &Theme, cx: &mut Context<Workspace>) -> Div {
    let split_h = cx.listener(|this, _, _window, cx| {
        this.split_horizontal(cx);
    });
    let split_v = cx.listener(|this, _, _window, cx| {
        this.split_vertical(cx);
    });
    let close = cx.listener(|this, _, _window, cx| {
        this.close_split_pane(cx);
    });
    let exit = cx.listener(|this, _, _window, cx| {
        this.toggle_split_mode(cx);
    });

    let surface_hover = theme.colors.surface_hover;
    let text = theme.colors.text;
    let text_muted = theme.colors.text_muted;
    let error = theme.colors.error;

    div()
        .flex()
        .items_center()
        .gap_2()
        .child(
            div()
                .id("split-horizontal")
                .px_2()
                .py_1()
                .rounded_md()
                .cursor_pointer()
                .text_xs()
                .text_color(text_muted)
                .hover(move |s| s.bg(surface_hover).text_color(text))
                .on_click(split_h)
                .child("Split ─"),
        )
        .child(
            div()
                .id("split-vertical")
                .px_2()
                .py_1()
                .rounded_md()
                .cursor_pointer()
                .text_xs()
                .text_color(text_muted)
                .hover(move |s| s.bg(surface_hover).text_color(text))
                .on_click(split_v)
                .child("Split │"),
        )
        .child(
            div()
                .id("close-split-pane")
                .px_2()
                .py_1()
                .rounded_md()
                .cursor_pointer()
                .text_xs()
                .text_color(text_muted)
                .hover(move |s| s.bg(error.opacity(0.2)).text_color(error))
                .on_click(close)
                .child("Close"),
        )
        .child(
            div()
                .id("exit-split-mode")
                .px_2()
                .py_1()
                .rounded_md()
                .cursor_pointer()
                .text_xs()
                .text_color(text_muted)
                .hover(move |s| s.bg(surface_hover).text_color(text))
                .on_click(exit)
                .child("Exit Split"),
        )
}
