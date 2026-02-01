use gpui::*;
use gpui::prelude::*;

/// Breadcrumb-style page indicator
#[derive(IntoElement)]
pub struct PageIndicator {
    current: usize,
    total: usize,
    show_numbers: bool,
    active_color: Option<gpui::Hsla>,
}

impl PageIndicator {
    pub fn new(current: usize, total: usize) -> Self {
        Self {
            current,
            total,
            show_numbers: false,
            active_color: None,
        }
    }

    pub fn show_numbers(mut self, show: bool) -> Self {
        self.show_numbers = show;
        self
    }

    pub fn active_color(mut self, color: gpui::Hsla) -> Self {
        self.active_color = Some(color);
        self
    }
}

impl RenderOnce for PageIndicator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let active_color = self.active_color.unwrap_or(hsla(0.6, 0.7, 0.5, 1.0));

        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .when(self.show_numbers, |el| {
                el.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(hsla(0.0, 0.0, 0.6, 1.0))
                        .child(format!("{} / {}", self.current + 1, self.total))
                )
            })
            .when(!self.show_numbers, |el| {
                el.children((0..self.total).map(|i| {
                    let is_current = i == self.current;
                    div()
                        .size(px(8.0))
                        .rounded_full()
                        .bg(if is_current {
                            active_color
                        } else {
                            hsla(0.0, 0.0, 0.3, 1.0)
                        })
                }))
            })
    }
}
