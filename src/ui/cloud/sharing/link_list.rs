//! Existing links list rendering

use gpui::prelude::*;
use gpui::*;

use super::dialog::ShareDialog;
use super::types::ShareDialogEvent;

impl ShareDialog {
    /// Render existing links
    pub(super) fn render_existing_links(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div().when(!self.existing_links.is_empty(), |this| {
                this.child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .pt_4()
                    .border_t_1()
                    .border_color(theme.colors.border)
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.colors.text)
                            .child("Existing Links"),
                    )
                    .children(
                        self.existing_links.iter().map(|link| {
                            let link_id = link.id.clone();
                            let link_url = link.url.clone();
                            let is_copied = self.copied_link_id.as_ref() == Some(&link.id);

                            div()
                                .w_full()
                                .p_3()
                                .rounded_md()
                                .bg(theme.colors.surface)
                                .border_1()
                                .border_color(theme.colors.border)
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
                                                .justify_between()
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_row()
                                                        .items_center()
                                                        .gap_2()
                                                        .child(
                                                            div()
                                                                .px_2()
                                                                .py_0p5()
                                                                .rounded_md()
                                                                .bg(theme.colors.accent.opacity(0.2))
                                                                .text_xs()
                                                                .text_color(theme.colors.accent)
                                                                .child(link.permission.display_name()),
                                                        )
                                                        .when(link.password_protected, |this| {
                this.child(
                                                                div()
                                                                    .text_xs()
                                                                    .text_color(theme.colors.text_muted)
                                                                    .child("Password protected"),
                                                            )
                                                        }),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(theme.colors.text_muted)
                                                        .child(format!(
                                                            "{} views",
                                                            link.access_count
                                                        )),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_family("monospace")
                                                .text_color(theme.colors.text_muted)
                                                .overflow_hidden()
                                                .child(link.url.clone()),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .justify_between()
                                                .child(
                                                    link.expires_at.map_or_else(
                                                        || {
                                                            div()
                                                                .text_xs()
                                                                .text_color(theme.colors.text_muted)
                                                                .child("Never expires")
                                                        },
                                                        |expires| {
                                                            let expired = expires < chrono::Utc::now();
                                                            div()
                                                                .text_xs()
                                                                .text_color(if expired {
                                                                    theme.colors.error
                                                                } else {
                                                                    theme.colors.text_muted
                                                                })
                                                                .child(if expired {
                                                                    "Expired".to_string()
                                                                } else {
                                                                    format!(
                                                                        "Expires {}",
                                                                        expires.format("%b %d, %Y")
                                                                    )
                                                                })
                                                        },
                                                    ),
                                                )
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_row()
                                                        .gap_2()
                                                        .child(
                                                            div()
                                                                .id(ElementId::Name(
                                                                    format!("copy-{}", link.id).into(),
                                                                ))
                                                                .px_2()
                                                                .py_1()
                                                                .rounded_md()
                                                                .cursor_pointer()
                                                                .bg(if is_copied {
                                                                    theme.colors.success.opacity(0.2)
                                                                } else {
                                                                    theme.colors.surface_hover
                                                                })
                                                                .text_xs()
                                                                .text_color(if is_copied {
                                                                    theme.colors.success
                                                                } else {
                                                                    theme.colors.text_muted
                                                                })
                                                                .on_click(cx.listener(
                                                                    move |_this, _event, _window, cx| {
                                                                        cx.emit(
                                                                            ShareDialogEvent::CopyLink(
                                                                                link_url.clone(),
                                                                            ),
                                                                        );
                                                                    },
                                                                ))
                                                                .child(if is_copied {
                                                                    "Copied!"
                                                                } else {
                                                                    "Copy"
                                                                }),
                                                        )
                                                        .child(
                                                            div()
                                                                .id(ElementId::Name(
                                                                    format!("revoke-{}", link.id)
                                                                        .into(),
                                                                ))
                                                                .px_2()
                                                                .py_1()
                                                                .rounded_md()
                                                                .cursor_pointer()
                                                                .bg(theme.colors.error.opacity(0.1))
                                                                .text_xs()
                                                                .text_color(theme.colors.error)
                                                                .hover(|this| {
                                                                    this.bg(
                                                                        theme.colors.error.opacity(0.2),
                                                                    )
                                                                })
                                                                .on_click(cx.listener(
                                                                    move |_this, _event, _window, cx| {
                                                                        cx.emit(
                                                                            ShareDialogEvent::RevokeLink(
                                                                                link_id.clone(),
                                                                            ),
                                                                        );
                                                                    },
                                                                ))
                                                                .child("Revoke"),
                                                        ),
                                                ),
                                        ),
                                )
                        }),
                    ),
            )
        })
    }
}
