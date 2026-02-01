//! Generic dialog component

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Generic dialog component
#[derive(Clone, IntoElement)]
pub struct Dialog {
    /// Title
    pub(crate) title: String,
    /// Description/message
    pub(crate) description: Option<String>,
    /// Icon
    pub(crate) icon: Option<String>,
    /// Size
    pub(crate) size: DialogSize,
    /// Buttons
    pub(crate) buttons: Vec<DialogButton>,
    /// Show close button
    pub(crate) show_close: bool,
    /// Close on backdrop click
    pub(crate) close_on_backdrop: bool,
    /// Center vertically
    pub(crate) centered: bool,
}

impl Dialog {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            icon: None,
            size: DialogSize::default(),
            buttons: Vec::new(),
            show_close: true,
            close_on_backdrop: true,
            centered: true,
        }
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn size(mut self, size: DialogSize) -> Self {
        self.size = size;
        self
    }

    pub fn button(mut self, button: DialogButton) -> Self {
        self.buttons.push(button);
        self
    }

    pub fn buttons(mut self, buttons: Vec<DialogButton>) -> Self {
        self.buttons = buttons;
        self
    }

    pub fn hide_close(mut self) -> Self {
        self.show_close = false;
        self
    }

    pub fn no_backdrop_close(mut self) -> Self {
        self.close_on_backdrop = false;
        self
    }

    pub fn top_aligned(mut self) -> Self {
        self.centered = false;
        self
    }
}

impl RenderOnce for Dialog {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.2, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);
        let danger = hsla(0.0, 0.7, 0.5, 1.0);
        let backdrop = hsla(0.0, 0.0, 0.0, 0.6);

        let width = if matches!(self.size, DialogSize::FullWidth) {
            relative(0.9)
        } else {
            px(self.size.width()).into()
        };

        let mut container = div()
            .absolute()
            .inset_0()
            .bg(backdrop)
            .flex()
            .items_center()
            .justify_center();

        if !self.centered {
            container = container.pt(px(100.0)).items_start();
        }

        container.child(
            div()
                .w(width)
                .max_w(relative(0.95))
                .bg(surface)
                .rounded(px(12.0))
                .border_1()
                .border_color(border)
                .shadow_xl()
                .flex()
                .flex_col()
                // Header
                .child(
                    div()
                        .w_full()
                        .px_5()
                        .pt_5()
                        .pb_3()
                        .flex()
                        .items_start()
                        .gap_4()
                        // Icon
                        .when_some(self.icon, |d, icon| {
                            d.child(
                                div()
                                    .size(px(40.0))
                                    .rounded_full()
                                    .bg(accent.opacity(0.15))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_lg()
                                    .flex_shrink_0()
                                    .child(icon)
                            )
                        })
                        // Title and description
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .child(
                                    div()
                                        .text_base()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(text)
                                        .child(self.title)
                                )
                                .when_some(self.description, |d, desc| {
                                    d.child(
                                        div()
                                            .text_sm()
                                            .text_color(text_muted)
                                            .child(desc)
                                    )
                                })
                        )
                        // Close button
                        .when(self.show_close, |d| {
                            d.child(
                                div()
                                    .size(px(28.0))
                                    .rounded(px(4.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_sm()
                                    .text_color(text_muted)
                                    .cursor_pointer()
                                    .hover(|s| s.bg(surface_hover).text_color(text))
                                    .child("×")
                            )
                        })
                )
                // Buttons
                .when(!self.buttons.is_empty(), |d| {
                    d.child(
                        div()
                            .w_full()
                            .px_5()
                            .py_4()
                            .flex()
                            .items_center()
                            .justify_end()
                            .gap_2()
                            .children(
                                self.buttons.into_iter().map(|btn| {
                                    let mut button = div()
                                        .px_4()
                                        .py_2()
                                        .rounded(px(6.0))
                                        .text_sm()
                                        .font_weight(FontWeight::MEDIUM)
                                        .cursor_pointer();

                                    if btn.disabled || btn.loading {
                                        button = button.opacity(0.5).cursor_default();
                                    }

                                    match btn.style {
                                        DialogButtonStyle::Primary => {
                                            button = button
                                                .bg(accent)
                                                .text_color(gpui::white())
                                                .when(!btn.disabled && !btn.loading, |d| {
                                                    d.hover(|s| s.opacity(0.9))
                                                });
                                        }
                                        DialogButtonStyle::Secondary => {
                                            button = button
                                                .border_1()
                                                .border_color(border)
                                                .text_color(text)
                                                .when(!btn.disabled && !btn.loading, |d| {
                                                    d.hover(|s| s.bg(surface_hover))
                                                });
                                        }
                                        DialogButtonStyle::Destructive => {
                                            button = button
                                                .bg(danger)
                                                .text_color(gpui::white())
                                                .when(!btn.disabled && !btn.loading, |d| {
                                                    d.hover(|s| s.opacity(0.9))
                                                });
                                        }
                                        DialogButtonStyle::Ghost => {
                                            button = button
                                                .text_color(text_muted)
                                                .when(!btn.disabled && !btn.loading, |d| {
                                                    d.hover(|s| s.text_color(text))
                                                });
                                        }
                                    }

                                    button.child(if btn.loading {
                                        "Loading...".to_string()
                                    } else {
                                        btn.label
                                    })
                                })
                            )
                    )
                })
        )
    }
}
