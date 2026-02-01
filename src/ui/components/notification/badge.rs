//! Notification badge (count indicator) component

use gpui::*;
use gpui::prelude::*;

/// Notification badge (count indicator)
#[derive(Clone, IntoElement)]
pub struct NotificationBadge {
    count: usize,
    max_count: usize,
    show_zero: bool,
    pulsing: bool,
}

impl NotificationBadge {
    pub fn new(count: usize) -> Self {
        Self {
            count,
            max_count: 99,
            show_zero: false,
            pulsing: false,
        }
    }

    pub fn max_count(mut self, max: usize) -> Self {
        self.max_count = max;
        self
    }

    pub fn show_zero(mut self) -> Self {
        self.show_zero = true;
        self
    }

    pub fn pulsing(mut self) -> Self {
        self.pulsing = true;
        self
    }

    pub(crate) fn display_count(&self) -> String {
        if self.count > self.max_count {
            format!("{}+", self.max_count)
        } else {
            format!("{}", self.count)
        }
    }
}

impl RenderOnce for NotificationBadge {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let accent = hsla(0.0, 0.7, 0.5, 1.0); // Red for notifications

        if self.count == 0 && !self.show_zero {
            return div().into_any_element();
        }

        let display = self.display_count();
        let is_small = self.count < 10;

        div()
            .min_w(px(if is_small { 18.0 } else { 22.0 }))
            .h(px(18.0))
            .px(px(if is_small { 0.0 } else { 4.0 }))
            .rounded_full()
            .bg(accent)
            .flex()
            .items_center()
            .justify_center()
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(gpui::white())
            .child(display)
            .into_any_element()
    }
}
