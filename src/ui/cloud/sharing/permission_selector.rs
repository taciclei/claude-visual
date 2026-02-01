//! Permission selector rendering

use gpui::prelude::*;
use gpui::*;

use super::dialog::ShareDialog;
use super::types::SharePermission;

impl ShareDialog {
    /// Render permission selector
    pub(super) fn render_permission_selector(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.colors.text)
                    .child("Permission"),
            )
            .child(
                div().flex().flex_col().gap_1().children(
                    [
                        SharePermission::View,
                        SharePermission::Comment,
                        SharePermission::Edit,
                    ]
                    .into_iter()
                    .map(|permission| {
                        let is_selected = self.selected_permission == permission;
                        div()
                            .id(ElementId::Name(
                                format!("permission-{:?}", permission).into(),
                            ))
                            .w_full()
                            .py_2()
                            .px_3()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(if is_selected {
                                theme.colors.accent.opacity(0.1)
                            } else {
                                theme.colors.surface
                            })
                            .border_1()
                            .border_color(if is_selected {
                                theme.colors.accent.opacity(0.3)
                            } else {
                                theme.colors.border
                            })
                            .hover(|this| this.bg(theme.colors.surface_hover))
                            .on_click(cx.listener(move |this, _event, _window, cx| {
                                this.set_permission(permission, cx);
                            }))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_3()
                                    .child(
                                        div()
                                            .size_4()
                                            .rounded_full()
                                            .border_2()
                                            .border_color(if is_selected {
                                                theme.colors.accent
                                            } else {
                                                theme.colors.border
                                            })
                                            .when(is_selected, |this| {
                                                this.child(
                                                    div()
                                                        .size_2()
                                                        .m(px(2.0))
                                                        .rounded_full()
                                                        .bg(theme.colors.accent),
                                                )
                                            }),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(theme.colors.text)
                                                    .child(permission.display_name()),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.colors.text_muted)
                                                    .child(permission.description()),
                                            ),
                                    ),
                            )
                    }),
                ),
            )
    }
}
