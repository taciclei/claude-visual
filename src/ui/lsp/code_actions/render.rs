//! Rendering logic for code actions panel

use gpui::*;
use gpui::prelude::*;

use super::core::CodeActionsPanel;
use super::types::CodeActionsEvent;

impl Render for CodeActionsPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();
        let actions = self.filtered_actions();
        let selected = self.selected_index;

        div()
            .bg(theme.colors.background)
            .border_1()
            .border_color(theme.colors.border)
            .rounded_md()
            .shadow_lg()
            .w(px(320.0))
            .max_h(px(300.0))
            .flex()
            .flex_col()
            .overflow_hidden()
            // Header
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_2()
                    .py_1()
                    .bg(theme.colors.surface)
                    .border_b_1()
                    .border_color(theme.colors.border)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(
                                div()
                                    .text_sm()
                                    .child("💡"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.colors.text)
                                    .child(format!("Code Actions ({})", self.action_count())),
                            ),
                    )
                    .child(
                        div()
                            .id("close-actions")
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .cursor_pointer()
                            .hover(|s| s.text_color(theme.colors.text))
                            .on_click(cx.listener(|_this, _, _window, cx| {
                                cx.emit(CodeActionsEvent::Dismiss);
                            }))
                            .child("×"),
                    ),
            )
            // Actions list
            .child(
                div()
                    .flex_1()
                    .id("scroll-code-actions")
                    .overflow_y_scroll()
                    .children(actions.into_iter().enumerate().map(|(i, action)| {
                        let id = action.id;
                        let is_selected = i == selected;
                        let is_preferred = action.is_preferred;
                        let icon = action.kind.icon();
                        let kind_name = action.kind.display_name();

                        div()
                            .id(SharedString::from(format!("action-{}", id)))
                            .flex()
                            .items_start()
                            .gap_2()
                            .px_2()
                            .py_1p5()
                            .cursor_pointer()
                            .when(is_selected, |d| d.bg(theme.colors.accent.opacity(0.1)))
                            .hover(|s| s.bg(theme.colors.surface))
                            .on_click(cx.listener(move |_this, _, _window, cx| {
                                cx.emit(CodeActionsEvent::Execute(id));
                            }))
                            // Icon
                            .child(
                                div()
                                    .text_sm()
                                    .child(icon.to_string()),
                            )
                            // Content
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .flex_col()
                                    .gap_0p5()
                                    // Title
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_1()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.colors.text)
                                                    .child(action.title.clone()),
                                            )
                                            .when(is_preferred, |d| {
                                                d.child(
                                                    div()
                                                        .text_xs()
                                                        .px_1()
                                                        .rounded_sm()
                                                        .bg(theme.colors.accent.opacity(0.2))
                                                        .text_color(theme.colors.accent)
                                                        .child("Preferred"),
                                                )
                                            }),
                                    )
                                    // Kind and server
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.colors.text_muted)
                                                    .child(kind_name.to_string()),
                                            )
                                            .when(!action.server.is_empty(), |d| {
                                                d.child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(theme.colors.text_muted)
                                                        .child(format!("• {}", action.server)),
                                                )
                                            }),
                                    )
                                    // Diagnostics preview
                                    .when(!action.diagnostics.is_empty(), |d| {
                                        d.child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.warning)
                                                .overflow_hidden()
                                                .text_ellipsis()
                                                .child(action.diagnostics.first().unwrap().clone()),
                                        )
                                    }),
                            )
                            // Keyboard hint
                            .when(is_selected, |d| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .child("↵"),
                                )
                            })
                    })),
            )
            // Empty state
            .when(self.actions.is_empty(), |d| {
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py_4()
                        .text_xs()
                        .text_color(theme.colors.text_muted)
                        .child("No code actions available"),
                )
            })
            // Footer with hints
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_2()
                    .py_1()
                    .bg(theme.colors.surface)
                    .border_t_1()
                    .border_color(theme.colors.border)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child("↑↓ Navigate"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child("↵ Apply"),
                            ),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child("Esc Close"),
                    ),
            )
    }
}
