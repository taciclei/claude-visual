//! Theme preview card component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Theme preview card
#[derive(IntoElement)]
pub struct ThemePreview {
    id: ElementId,
    mode: ThemeMode,
    is_selected: bool,
    preview_content: Option<SharedString>,
}

impl ThemePreview {
    pub fn new(id: impl Into<ElementId>, mode: ThemeMode) -> Self {
        Self {
            id: id.into(),
            mode,
            is_selected: false,
            preview_content: None,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
        self
    }

    pub fn preview_content(mut self, content: impl Into<SharedString>) -> Self {
        self.preview_content = Some(content.into());
        self
    }
}

impl RenderOnce for ThemePreview {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (bg_color, text_color, accent_color) = match self.mode {
            ThemeMode::Light => (
                hsla(0.0, 0.0, 0.98, 1.0),
                hsla(0.0, 0.0, 0.1, 1.0),
                hsla(0.6, 0.7, 0.5, 1.0),
            ),
            ThemeMode::Dark => (
                hsla(0.0, 0.0, 0.08, 1.0),
                hsla(0.0, 0.0, 0.95, 1.0),
                hsla(0.6, 0.7, 0.5, 1.0),
            ),
            ThemeMode::System => (
                hsla(0.0, 0.0, 0.15, 1.0),
                hsla(0.0, 0.0, 0.85, 1.0),
                hsla(0.6, 0.7, 0.5, 1.0),
            ),
        };

        let border_color = if self.is_selected {
            accent_color
        } else {
            hsla(0.0, 0.0, 0.25, 1.0)
        };

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap(px(8.0))
            .cursor_pointer()
            // Preview card
            .child(
                div()
                    .w(px(120.0))
                    .h(px(80.0))
                    .bg(bg_color)
                    .border_2()
                    .border_color(border_color)
                    .rounded(px(8.0))
                    .overflow_hidden()
                    .p(px(8.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .w(px(60.0))
                                    .h(px(8.0))
                                    .bg(accent_color)
                                    .rounded(px(2.0)),
                            )
                            .child(
                                div()
                                    .w(px(80.0))
                                    .h(px(6.0))
                                    .bg(text_color)
                                    .opacity(0.3)
                                    .rounded(px(2.0)),
                            )
                            .child(
                                div()
                                    .w(px(50.0))
                                    .h(px(6.0))
                                    .bg(text_color)
                                    .opacity(0.2)
                                    .rounded(px(2.0)),
                            ),
                    ),
            )
            // Label
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap(px(4.0))
                    .child(div().text_size(px(14.0)).child(self.mode.icon()))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(if self.is_selected {
                                FontWeight::MEDIUM
                            } else {
                                FontWeight::NORMAL
                            })
                            .text_color(if self.is_selected {
                                accent_color
                            } else {
                                hsla(0.0, 0.0, 0.7, 1.0)
                            })
                            .child(self.mode.label()),
                    ),
            )
    }
}
