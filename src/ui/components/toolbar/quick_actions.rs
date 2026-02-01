//! Quick actions bar component

use gpui::prelude::*;
use gpui::*;

/// Quick actions bar - simplified toolbar for common actions
#[derive(IntoElement)]
pub struct QuickActions {
    items: Vec<(SharedString, SharedString)>, // (icon, label)
    selected: Option<usize>,
    background: Option<gpui::Hsla>,
}

impl QuickActions {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected: None,
            background: None,
        }
    }

    pub fn items(mut self, items: Vec<(SharedString, SharedString)>) -> Self {
        self.items = items;
        self
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = Some(index);
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = Some(color);
        self
    }
}

impl Default for QuickActions {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for QuickActions {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.12, 1.0));

        div()
            .flex()
            .items_center()
            .gap(px(4.0))
            .px(px(8.0))
            .py(px(4.0))
            .rounded(px(8.0))
            .bg(bg)
            .children(
                self.items
                    .into_iter()
                    .enumerate()
                    .map(|(i, (icon, label))| {
                        let is_selected = self.selected == Some(i);

                        div()
                            .px(px(12.0))
                            .py(px(6.0))
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .rounded(px(6.0))
                            .cursor_pointer()
                            .when(is_selected, |el| el.bg(hsla(0.6, 0.5, 0.4, 0.3)))
                            .when(!is_selected, |el| {
                                el.hover(|style| style.bg(hsla(0.0, 0.0, 0.2, 1.0)))
                            })
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(if is_selected {
                                        hsla(0.6, 0.7, 0.6, 1.0)
                                    } else {
                                        hsla(0.0, 0.0, 0.6, 1.0)
                                    })
                                    .child(icon),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(if is_selected {
                                        hsla(0.0, 0.0, 0.95, 1.0)
                                    } else {
                                        hsla(0.0, 0.0, 0.7, 1.0)
                                    })
                                    .child(label),
                            )
                    }),
            )
    }
}
