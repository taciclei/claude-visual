//! Share options rendering (password and expiry)

use gpui::*;
use gpui::prelude::*;

use super::dialog::ShareDialog;
use super::types::ExpiryOption;

impl ShareDialog {
    /// Render options
    pub(super) fn render_options(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex()
            .flex_col()
            .gap_4()
            // Password protection
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .id("toggle-password")
                                    .size_5()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .border_1()
                                    .border_color(if self.use_password {
                                        theme.colors.accent
                                    } else {
                                        theme.colors.border
                                    })
                                    .bg(if self.use_password {
                                        theme.colors.accent
                                    } else {
                                        theme.colors.surface
                                    })
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.toggle_password(cx);
                                    }))
                                    .when(self.use_password, |this| {
                this.flex().items_center().justify_center().child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.background)
                                                .child("✓"),
                                        )
                                    }),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.colors.text)
                                    .child("Password protection"),
                            ),
                    )
                    .when(self.use_password, |this| {
                this.child(
                            div()
                                .w_full()
                                .h(px(36.0))
                                .px_3()
                                .rounded_md()
                                .bg(theme.colors.surface)
                                .border_1()
                                .border_color(theme.colors.border)
                                .flex()
                                .items_center()
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(theme.colors.text_muted)
                                        .child("Enter password..."),
                                ),
                        )
                    }),
            )
            // Expiry
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.colors.text)
                            .child("Link expires"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .gap_2()
                            .children(
                                [
                                    ExpiryOption::Never,
                                    ExpiryOption::OneHour,
                                    ExpiryOption::OneDay,
                                    ExpiryOption::OneWeek,
                                    ExpiryOption::OneMonth,
                                ]
                                .into_iter()
                                .map(|option| {
                                    let is_selected = self.expiry_option == option;
                                    div()
                                        .id(ElementId::Name(
                                            format!("expiry-{:?}", option).into(),
                                        ))
                                        .px_3()
                                        .py_1()
                                        .rounded_md()
                                        .cursor_pointer()
                                        .bg(if is_selected {
                                            theme.colors.accent.opacity(0.2)
                                        } else {
                                            theme.colors.surface
                                        })
                                        .text_sm()
                                        .text_color(if is_selected {
                                            theme.colors.accent
                                        } else {
                                            theme.colors.text_muted
                                        })
                                        .hover(|this| {
                                            this.bg(if is_selected {
                                                theme.colors.accent.opacity(0.3)
                                            } else {
                                                theme.colors.surface_hover
                                            })
                                        })
                                        .on_click(cx.listener(move |this, _event, _window, cx| {
                                            this.set_expiry(option, cx);
                                        }))
                                        .child(option.display_name())
                                }),
                            ),
                    ),
            )
    }
}
