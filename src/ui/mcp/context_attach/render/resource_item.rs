//! Individual resource item rendering

use gpui::prelude::*;
use gpui::*;

use super::super::core::McpContextAttachPanel;
use super::super::types::*;

impl McpContextAttachPanel {
    /// Render a single resource item
    pub(super) fn render_resource_item(
        &self,
        index: usize,
        resource: &AttachableResource,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let is_attached = matches!(resource.status, AttachmentStatus::Attached);
        let is_loading = matches!(resource.status, AttachmentStatus::Loading);

        // Status color
        let status_color = match &resource.status {
            AttachmentStatus::Ready => theme.colors.text_muted,
            AttachmentStatus::Loading => theme.colors.warning,
            AttachmentStatus::Attached => theme.colors.success,
            AttachmentStatus::Failed(_) => theme.colors.error,
        };

        let error_color = theme.colors.error;
        let accent_color = theme.colors.accent;
        let surface_hover = theme.colors.surface_hover;

        let detach_listener = cx.listener(move |this, _, _window, cx| {
            this.detach(index, cx);
        });

        let attach_listener = cx.listener(move |this, _, _window, cx| {
            this.request_attach(index, cx);
        });

        div()
            .id(ElementId::Name(format!("resource-{}", index).into()))
            .px_3()
            .py_2()
            .border_b_1()
            .border_color(theme.colors.border)
            .hover(|s| s.bg(surface_hover))
            .cursor_pointer()
            // Main content
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            // Status indicator
                            .child(
                                div()
                                    .size(px(8.0))
                                    .rounded_full()
                                    .bg(status_color)
                                    .when(is_loading, |d| {
                                        d.border_1().border_color(status_color.opacity(0.5))
                                    }),
                            )
                            // Resource icon
                            .child(
                                div()
                                    .text_sm()
                                    .child(self.get_resource_icon(resource.mime_type.as_deref())),
                            )
                            // Name and server
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(theme.colors.text)
                                            .child(resource.name.clone()),
                                    )
                                    .child(
                                        div().text_xs().text_color(theme.colors.text_muted).child(
                                            format!("{} • {}", resource.server, resource.uri),
                                        ),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            // Size
                            .when_some(resource.size, |d, size| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .child(self.format_size(size)),
                                )
                            })
                            // Action button
                            .when(!is_loading, |d| {
                                if is_attached {
                                    d.child(
                                        div()
                                            .id(ElementId::Name(format!("detach-{}", index).into()))
                                            .px_2()
                                            .py_1()
                                            .rounded_sm()
                                            .text_xs()
                                            .text_color(error_color)
                                            .hover(|s| s.bg(error_color.opacity(0.1)))
                                            .on_click(detach_listener)
                                            .child("Detach"),
                                    )
                                } else {
                                    d.child(
                                        div()
                                            .id(ElementId::Name(format!("attach-{}", index).into()))
                                            .px_2()
                                            .py_1()
                                            .rounded_sm()
                                            .text_xs()
                                            .text_color(accent_color)
                                            .hover(|s| s.bg(accent_color.opacity(0.1)))
                                            .on_click(attach_listener)
                                            .child("Attach"),
                                    )
                                }
                            })
                            // Loading spinner
                            .when(is_loading, |d| {
                                d.child(
                                    div()
                                        .px_2()
                                        .text_xs()
                                        .text_color(theme.colors.warning)
                                        .child("..."),
                                )
                            }),
                    ),
            )
            // Description
            .when_some(resource.description.clone(), |d, desc| {
                d.child(
                    div()
                        .mt_1()
                        .pl(px(24.0))
                        .text_xs()
                        .text_color(theme.colors.text_muted)
                        .child(desc),
                )
            })
            // Error message
            .when(
                matches!(&resource.status, AttachmentStatus::Failed(_)),
                |d| {
                    if let AttachmentStatus::Failed(err) = &resource.status {
                        d.child(
                            div()
                                .mt_1()
                                .pl(px(24.0))
                                .text_xs()
                                .text_color(theme.colors.error)
                                .child(err.clone()),
                        )
                    } else {
                        d
                    }
                },
            )
    }
}
