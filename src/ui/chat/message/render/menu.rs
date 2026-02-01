//! Context menu and action buttons rendering

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;
use super::super::view::MessageView;
use super::super::types::MessageAction;

impl MessageView {
    /// Render the context menu
    pub(in crate::ui::chat::message) fn render_context_menu(&self, theme: &Theme, cx: &mut Context<Self>) -> Div {
        let actions = self.available_actions();

        div()
            .absolute()
            .top(self.context_menu_position.y)
            .left(self.context_menu_position.x)
            .min_w(px(180.0))
            .bg(theme.colors.surface)
            .border_1()
            .border_color(theme.colors.border)
            .rounded_md()
            .shadow_lg()
            .py_1()

            .children(actions.into_iter().enumerate().map(|(idx, action)| {
                let is_destructive = action.is_destructive();
                let label = action.label();
                let shortcut = action.shortcut();
                let icon = action.icon();

                // Add separator before Delete
                let needs_separator = action == MessageAction::Delete && idx > 0;

                div()
                    .w_full()
                    .when(needs_separator, |d| {
                        d.child(
                            div()
                                .h(px(1.0))
                                .w_full()
                                .bg(theme.colors.border)
                                .my_1()
                        )
                    })
                    .child(
                        div()
                            .id(ElementId::Name(format!("action-{:?}", action).into()))
                            .flex()
                            .items_center()
                            .justify_between()
                            .px_3()
                            .py_1p5()
                            .cursor_pointer()
                            .hover(|style| style.bg(theme.colors.surface_hover))
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                this.execute_action(action, cx);
                            }))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .child(icon)
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(if is_destructive {
                                                theme.colors.error
                                            } else {
                                                theme.colors.text
                                            })
                                            .child(label)
                                    )
                            )
                            .when_some(shortcut, |this, sc| {
                                this.child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .font_family("monospace")
                                        .child(sc)
                                )
                            })
                    )
            }))
    }

    /// Render more actions button (three dots)
    pub(in crate::ui::chat::message) fn render_more_button(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_clone = theme.clone();
        div()
            .id("more-actions")
            .px_2()
            .py_1()
            .rounded_sm()
            .text_xs()
            .text_color(theme.colors.text_muted)
            .hover(move |style| {
                style
                    .bg(theme_clone.colors.surface_hover)
                    .text_color(theme_clone.colors.text)
            })
            .cursor_pointer()
            .on_click(cx.listener(|this, _, _window, cx| {
                // Toggle context menu near the button
                if this.show_context_menu {
                    this.hide_context_menu(cx);
                } else {
                    // Show at a fixed offset from the message
                    this.show_context_menu(Point::new(px(200.0), px(0.0)), cx);
                }
            }))
            .child("•••")
    }
}
