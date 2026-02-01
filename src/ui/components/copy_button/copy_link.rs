//! Copy link component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Copy link component - copies a URL
#[derive(IntoElement)]
pub struct CopyLink {
    pub(crate) id: ElementId,
    pub(crate) url: SharedString,
    pub(crate) label: SharedString,
    pub(crate) state: CopyState,
    pub(crate) show_url: bool,
}

impl CopyLink {
    pub fn new(id: impl Into<ElementId>, url: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            url: url.into(),
            label: "Copy link".into(),
            state: CopyState::default(),
            show_url: false,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    pub fn state(mut self, state: CopyState) -> Self {
        self.state = state;
        self
    }

    pub fn show_url(mut self, show: bool) -> Self {
        self.show_url = show;
        self
    }
}

impl RenderOnce for CopyLink {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (icon, color) = match self.state {
            CopyState::Idle => ("🔗", hsla(0.6, 0.7, 0.5, 1.0)),
            CopyState::Copying => ("⏳", hsla(0.0, 0.0, 0.5, 1.0)),
            CopyState::Copied => ("✓", hsla(0.35, 0.7, 0.45, 1.0)),
            CopyState::Error => ("✗", hsla(0.0, 0.7, 0.5, 1.0)),
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(8.0))
            .px(px(12.0))
            .py(px(8.0))
            .rounded(px(6.0))
            .border_1()
            .border_color(hsla(0.0, 0.0, 0.25, 1.0))
            .bg(hsla(0.0, 0.0, 0.1, 1.0))
            .cursor_pointer()
            .hover(|style| style.bg(hsla(0.0, 0.0, 0.15, 1.0)))
            .child(div().text_size(px(14.0)).text_color(color).child(icon))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                            .child(if self.state == CopyState::Copied {
                                SharedString::from("Link copied!")
                            } else {
                                self.label.clone()
                            }),
                    )
                    .when(self.show_url, |el| {
                        el.child(
                            div()
                                .text_size(px(11.0))
                                .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                .overflow_hidden()
                                .text_ellipsis()
                                .max_w(px(200.0))
                                .child(self.url.clone()),
                        )
                    }),
            )
    }
}
