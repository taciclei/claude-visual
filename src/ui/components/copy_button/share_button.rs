//! Share button component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Share button with copy option
#[derive(IntoElement)]
pub struct ShareButton {
    pub(crate) id: ElementId,
    pub(crate) url: SharedString,
    pub(crate) title: SharedString,
    pub(crate) show_options: bool,
    pub(crate) state: CopyState,
}

impl ShareButton {
    pub fn new(id: impl Into<ElementId>, url: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            url: url.into(),
            title: "Share".into(),
            show_options: false,
            state: CopyState::default(),
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }

    pub fn show_options(mut self, show: bool) -> Self {
        self.show_options = show;
        self
    }

    pub fn state(mut self, state: CopyState) -> Self {
        self.state = state;
        self
    }
}

impl RenderOnce for ShareButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .px(px(16.0))
                    .py(px(8.0))
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .rounded(px(6.0))
                    .bg(hsla(0.6, 0.6, 0.5, 1.0))
                    .cursor_pointer()
                    .hover(|style| style.bg(hsla(0.6, 0.6, 0.45, 1.0)))
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                            .child("📤"),
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                            .child(self.title.clone()),
                    ),
            )
            .when(self.show_options, |el| {
                el.child(
                    div()
                        .mt(px(4.0))
                        .p(px(8.0))
                        .rounded(px(8.0))
                        .bg(hsla(0.0, 0.0, 0.12, 1.0))
                        .border_1()
                        .border_color(hsla(0.0, 0.0, 0.25, 1.0))
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .children(
                            [
                                ("📋", "Copy link"),
                                ("✉️", "Email"),
                                ("🐦", "Twitter"),
                                ("💼", "LinkedIn"),
                            ]
                            .into_iter()
                            .map(|(icon, label)| {
                                div()
                                    .px(px(12.0))
                                    .py(px(8.0))
                                    .flex()
                                    .items_center()
                                    .gap(px(8.0))
                                    .rounded(px(4.0))
                                    .cursor_pointer()
                                    .hover(|style| style.bg(hsla(0.0, 0.0, 0.2, 1.0)))
                                    .child(div().text_size(px(14.0)).child(icon))
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .text_color(hsla(0.0, 0.0, 0.8, 1.0))
                                            .child(label),
                                    )
                            }),
                        ),
                )
            })
    }
}
