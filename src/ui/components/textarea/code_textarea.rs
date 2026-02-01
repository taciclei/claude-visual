//! Simple code editor textarea

use gpui::prelude::*;
use gpui::*;
use std::sync::Arc;

use crate::app::state::AppState;

/// Simple code editor textarea
pub struct CodeTextarea {
    pub(crate) app_state: Arc<AppState>,
    /// Code content
    pub(crate) code: String,
    /// Language for display
    pub(crate) language: Option<String>,
    /// Number of rows
    pub(crate) rows: usize,
    /// Whether to show line numbers
    pub(crate) show_line_numbers: bool,
    /// Whether editor is readonly
    pub(crate) readonly: bool,
    /// Focus handle
    pub(crate) focus_handle: FocusHandle,
}

impl CodeTextarea {
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            code: String::new(),
            language: None,
            rows: 10,
            show_line_numbers: true,
            readonly: false,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn set_code(&mut self, code: impl Into<String>, cx: &mut Context<Self>) {
        self.code = code.into();
        cx.notify();
    }

    pub fn set_language(&mut self, language: Option<String>, cx: &mut Context<Self>) {
        self.language = language;
        cx.notify();
    }

    pub fn set_rows(&mut self, rows: usize, cx: &mut Context<Self>) {
        self.rows = rows.max(1);
        cx.notify();
    }

    pub fn set_show_line_numbers(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_line_numbers = show;
        cx.notify();
    }

    pub fn set_readonly(&mut self, readonly: bool, cx: &mut Context<Self>) {
        self.readonly = readonly;
        cx.notify();
    }

    pub fn code(&self) -> &str {
        &self.code
    }
}

impl Focusable for CodeTextarea {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CodeTextarea {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let line_height = 18.0_f32;
        let height = (self.rows as f32 * line_height) + 16.0;

        let lines: Vec<String> = self.code.lines().map(|s| s.to_string()).collect();
        let line_count = lines.len().max(1);
        let line_number_width = (line_count.to_string().len() as f32 * 8.0).max(24.0);

        div()
            .id("code-textarea")
            .w_full()
            .h(px(height))
            .rounded(px(6.0))
            .border_1()
            .border_color(theme.colors.border)
            .bg(theme.colors.surface)
            .overflow_hidden()
            .flex()
            // Line numbers gutter
            .when(self.show_line_numbers, |d| {
                d.child(
                    div()
                        .w(px(line_number_width + 16.0))
                        .h_full()
                        .px_2()
                        .py_2()
                        .bg(theme.colors.surface_hover)
                        .border_r_1()
                        .border_color(theme.colors.border)
                        .flex()
                        .flex_col()
                        .children((1..=line_count).map(|n| {
                            div()
                                .h(px(line_height))
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .text_right()
                                .child(n.to_string())
                        })),
                )
            })
            // Code content
            .child({
                let lines_for_render: Vec<String> = lines.clone();
                let is_empty = lines_for_render.is_empty();
                div()
                    .flex_1()
                    .h_full()
                    .px_3()
                    .py_2()
                    .id("scroll-code-content")
                    .overflow_y_scroll()
                    .font_family("monospace")
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .children(lines_for_render.into_iter().map(|line| {
                                div()
                                    .h(px(line_height))
                                    .text_xs()
                                    .text_color(theme.colors.text)
                                    .child(if line.is_empty() {
                                        " ".to_string()
                                    } else {
                                        line
                                    })
                            }))
                            .when(is_empty, |d| {
                                d.child(
                                    div()
                                        .h(px(line_height))
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .child(" "),
                                )
                            }),
                    )
            })
            // Language badge
            .when_some(self.language.clone(), |d, lang| {
                d.child(
                    div()
                        .absolute()
                        .top_2()
                        .right_2()
                        .px_2()
                        .py_0p5()
                        .rounded(px(4.0))
                        .bg(theme.colors.accent.opacity(0.15))
                        .text_xs()
                        .text_color(theme.colors.accent)
                        .child(lang),
                )
            })
    }
}
