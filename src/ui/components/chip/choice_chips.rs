//! Choice chips component (single select)

use gpui::*;
use gpui::prelude::*;

/// Choice chips (single select)
#[derive(Clone)]
pub struct ChoiceChips {
    pub(crate) options: Vec<String>,
    pub(crate) selected: Option<usize>,
}

impl ChoiceChips {
    pub fn new(options: Vec<impl Into<String>>) -> Self {
        Self {
            options: options.into_iter().map(|o| o.into()).collect(),
            selected: None,
        }
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = Some(index);
        self
    }
}

impl RenderOnce for ChoiceChips {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.2, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);

        div()
            .flex()
            .flex_wrap()
            .gap_2()
            .children(self.options.into_iter().enumerate().map(|(idx, option)| {
                let is_selected = self.selected == Some(idx);

                let (bg, border_col, text_col) = if is_selected {
                    (accent, accent, white())
                } else {
                    (surface, border, text)
                };

                div()
                    .h(px(36.0))
                    .px_4()
                    .rounded_full()
                    .bg(bg)
                    .border_1()
                    .border_color(border_col)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_sm()
                    .text_color(text_col)
                    .cursor_pointer()
                    .hover(|s| {
                        if is_selected {
                            s.opacity(0.9)
                        } else {
                            s.bg(surface_hover)
                        }
                    })
                    .child(option)
            }))
    }
}
