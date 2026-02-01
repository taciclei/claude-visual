//! Simple stateless tab bar component

use gpui::prelude::*;
use gpui::*;

/// Simple stateless tab bar
#[derive(Clone)]
pub struct TabBar {
    tabs: Vec<(String, String)>, // (id, label)
    active: String,
}

impl TabBar {
    pub fn new(
        tabs: Vec<(impl Into<String>, impl Into<String>)>,
        active: impl Into<String>,
    ) -> Self {
        Self {
            tabs: tabs
                .into_iter()
                .map(|(id, label)| (id.into(), label.into()))
                .collect(),
            active: active.into(),
        }
    }
}

impl RenderOnce for TabBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let border_color = hsla(0.0, 0.0, 0.3, 1.0);
        let accent = hsla(210.0 / 360.0, 0.8, 0.5, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.6, 1.0);

        div()
            .flex()
            .items_center()
            .border_b_1()
            .border_color(border_color)
            .children(self.tabs.iter().map(|(id, label)| {
                let is_active = &self.active == id;

                div()
                    .h(px(40.0))
                    .px_4()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_sm()
                    .when(is_active, |d| {
                        d.text_color(accent)
                            .border_b_2()
                            .border_color(accent)
                            .mb(px(-1.0))
                    })
                    .when(!is_active, |d| {
                        d.text_color(text_muted)
                            .border_b_2()
                            .border_color(gpui::transparent_black())
                    })
                    .child(label.clone())
            }))
    }
}
