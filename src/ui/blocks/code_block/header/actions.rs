//! Code block header action buttons

use gpui::prelude::*;
use gpui::*;

use crate::ui::blocks::code_block::{types::CodeDisplayMode, view::CodeBlockView};

pub(super) fn render_execute_button(
    theme: &crate::app::theme::Theme,
    cx: &mut Context<CodeBlockView>,
) -> impl IntoElement {
    let success = theme.colors.success;
    let on_click = cx.listener(|this, _, window, cx| {
        this.execute(window, cx);
    });

    div()
        .id("execute-button")
        .px_2()
        .py_1()
        .rounded_sm()
        .text_xs()
        .bg(success.opacity(0.1))
        .text_color(success)
        .border_1()
        .border_color(success.opacity(0.3))
        .hover(|style| {
            style
                .bg(success.opacity(0.2))
                .border_color(success.opacity(0.5))
        })
        .cursor_pointer()
        .on_click(on_click)
        .child("Run")
}

pub(super) fn render_save_button(
    theme: &crate::app::theme::Theme,
    cx: &mut Context<CodeBlockView>,
) -> impl IntoElement {
    let text_muted = theme.colors.text_muted;
    let surface_hover = theme.colors.surface_hover;
    let text = theme.colors.text;
    let on_click = cx.listener(|this, _, window, cx| {
        this.save_to_file(window, cx);
    });

    div()
        .id("save-button")
        .px_2()
        .py_1()
        .rounded_sm()
        .text_xs()
        .text_color(text_muted)
        .hover(move |style| style.bg(surface_hover).text_color(text))
        .cursor_pointer()
        .on_click(on_click)
        .child("Save")
}

pub(super) fn render_copy_button(
    show_copied: bool,
    theme: &crate::app::theme::Theme,
    cx: &mut Context<CodeBlockView>,
) -> impl IntoElement {
    let success = theme.colors.success;
    let text_muted = theme.colors.text_muted;
    let surface_hover = theme.colors.surface_hover;
    let text = theme.colors.text;
    let on_click = cx.listener(|this, _, _window, cx| {
        this.copy_to_clipboard(cx);
    });

    div()
        .id("copy-button")
        .px_2()
        .py_1()
        .rounded_sm()
        .text_xs()
        .text_color(if show_copied { success } else { text_muted })
        .bg(if show_copied {
            success.opacity(0.1)
        } else {
            gpui::transparent_black()
        })
        .hover(move |style| style.bg(surface_hover).text_color(text))
        .cursor_pointer()
        .on_click(on_click)
        .child(if show_copied { "Copied!" } else { "Copy" })
}

pub(super) fn render_search_button(
    search_visible: bool,
    theme: &crate::app::theme::Theme,
    cx: &mut Context<CodeBlockView>,
) -> impl IntoElement {
    let accent = theme.colors.accent;
    let text_muted = theme.colors.text_muted;
    let surface_hover = theme.colors.surface_hover;
    let text = theme.colors.text;
    let on_click = cx.listener(|this, _, _window, cx| {
        this.toggle_search(cx);
    });

    div()
        .id("search-button")
        .px_2()
        .py_1()
        .rounded_sm()
        .text_xs()
        .text_color(if search_visible { accent } else { text_muted })
        .bg(if search_visible {
            accent.opacity(0.1)
        } else {
            gpui::transparent_black()
        })
        .hover(move |style| style.bg(surface_hover).text_color(text))
        .cursor_pointer()
        .on_click(on_click)
        .child("Search")
}

pub(super) fn render_diff_button(
    view: &CodeBlockView,
    theme: &crate::app::theme::Theme,
    cx: &mut Context<CodeBlockView>,
) -> impl IntoElement {
    let is_diff_mode = view.display_mode == CodeDisplayMode::Diff;
    let (added, removed) = view.diff_stats();
    let accent = theme.colors.accent;
    let text_muted = theme.colors.text_muted;
    let surface_hover = theme.colors.surface_hover;
    let text = theme.colors.text;
    let on_click = cx.listener(|this, _, _window, cx| {
        this.toggle_display_mode(cx);
    });

    div()
        .id("diff-button")
        .px_2()
        .py_1()
        .rounded_sm()
        .text_xs()
        .text_color(if is_diff_mode { accent } else { text_muted })
        .bg(if is_diff_mode {
            accent.opacity(0.1)
        } else {
            gpui::transparent_black()
        })
        .hover(move |style| style.bg(surface_hover).text_color(text))
        .cursor_pointer()
        .on_click(on_click)
        .child(if is_diff_mode {
            format!("Diff +{} -{}", added, removed)
        } else {
            "Diff".to_string()
        })
}

pub(super) fn render_explain_button(
    theme: &crate::app::theme::Theme,
    cx: &mut Context<CodeBlockView>,
) -> impl IntoElement {
    let info = theme.colors.info;
    let on_click = cx.listener(|this, _, _window, cx| {
        this.explain_code(cx);
    });

    div()
        .id("explain-button")
        .px_2()
        .py_1()
        .rounded_sm()
        .text_xs()
        .bg(info.opacity(0.1))
        .text_color(info)
        .border_1()
        .border_color(info.opacity(0.2))
        .hover(|style| style.bg(info.opacity(0.2)).border_color(info.opacity(0.4)))
        .cursor_pointer()
        .on_click(on_click)
        .child("💡 Explain")
}

pub(super) fn render_improve_button(
    theme: &crate::app::theme::Theme,
    cx: &mut Context<CodeBlockView>,
) -> impl IntoElement {
    let text_muted = theme.colors.text_muted;
    let surface_hover = theme.colors.surface_hover;
    let text = theme.colors.text;
    let on_click = cx.listener(|this, _, _window, cx| {
        this.improve_code(cx);
    });

    div()
        .id("improve-button")
        .px_2()
        .py_1()
        .rounded_sm()
        .text_xs()
        .text_color(text_muted)
        .hover(move |style| style.bg(surface_hover).text_color(text))
        .cursor_pointer()
        .on_click(on_click)
        .child("✨ Improve")
}

pub(super) fn render_test_button(
    theme: &crate::app::theme::Theme,
    cx: &mut Context<CodeBlockView>,
) -> impl IntoElement {
    let text_muted = theme.colors.text_muted;
    let surface_hover = theme.colors.surface_hover;
    let text = theme.colors.text;
    let on_click = cx.listener(|this, _, _window, cx| {
        this.add_tests(cx);
    });

    div()
        .id("test-button")
        .px_2()
        .py_1()
        .rounded_sm()
        .text_xs()
        .text_color(text_muted)
        .hover(move |style| style.bg(surface_hover).text_color(text))
        .cursor_pointer()
        .on_click(on_click)
        .child("🧪 Test")
}

pub(super) fn render_review_button(
    theme: &crate::app::theme::Theme,
    cx: &mut Context<CodeBlockView>,
) -> impl IntoElement {
    let warning = theme.colors.warning;
    let on_click = cx.listener(|this, _, _window, cx| {
        this.review_code(cx);
    });

    div()
        .id("review-button")
        .px_2()
        .py_1()
        .rounded_sm()
        .text_xs()
        .bg(warning.opacity(0.1))
        .text_color(warning)
        .border_1()
        .border_color(warning.opacity(0.2))
        .hover(|style| {
            style
                .bg(warning.opacity(0.2))
                .border_color(warning.opacity(0.4))
        })
        .cursor_pointer()
        .on_click(on_click)
        .child("👀 Review")
}

pub(super) fn render_refactor_button(
    theme: &crate::app::theme::Theme,
    cx: &mut Context<CodeBlockView>,
) -> impl IntoElement {
    let accent = theme.colors.accent;
    let on_click = cx.listener(|this, _, _window, cx| {
        this.refactor_code(cx);
    });

    div()
        .id("refactor-button")
        .px_2()
        .py_1()
        .rounded_sm()
        .text_xs()
        .bg(accent.opacity(0.1))
        .text_color(accent)
        .border_1()
        .border_color(accent.opacity(0.2))
        .hover(|style| {
            style
                .bg(accent.opacity(0.2))
                .border_color(accent.opacity(0.4))
        })
        .cursor_pointer()
        .on_click(on_click)
        .child("♻️ Refactor")
}

pub(super) fn render_collapse_button(
    collapsed: bool,
    theme: &crate::app::theme::Theme,
    cx: &mut Context<CodeBlockView>,
) -> impl IntoElement {
    let text_muted = theme.colors.text_muted;
    let surface_hover = theme.colors.surface_hover;
    let text = theme.colors.text;
    let on_click = cx.listener(|this, _, _window, cx| {
        this.toggle_collapsed(cx);
    });

    div()
        .id("collapse-button")
        .px_2()
        .py_1()
        .rounded_sm()
        .text_xs()
        .text_color(text_muted)
        .hover(move |style| style.bg(surface_hover).text_color(text))
        .cursor_pointer()
        .on_click(on_click)
        .child(if collapsed { "Expand" } else { "Collapse" })
}
