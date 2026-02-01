//! Fieldset - group of related fields with border

use gpui::*;
use gpui::prelude::*;

#[derive(IntoElement)]
pub struct Fieldset {
    legend: Option<SharedString>,
    children: Vec<gpui::AnyElement>,
    disabled: bool,
}

impl Fieldset {
    pub fn new() -> Self {
        Self {
            legend: None,
            children: Vec::new(),
            disabled: false,
        }
    }

    pub fn legend(mut self, legend: impl Into<SharedString>) -> Self {
        self.legend = Some(legend.into());
        self
    }

    pub fn children(mut self, children: Vec<impl IntoElement>) -> Self {
        self.children = children.into_iter().map(|c| c.into_any_element()).collect();
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Default for Fieldset {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Fieldset {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .p(px(16.0))
            .rounded(px(8.0))
            .border_1()
            .border_color(hsla(0.0, 0.0, 0.25, 1.0))
            .flex()
            .flex_col()
            .gap(px(16.0))
            .when(self.disabled, |el| el.opacity(0.6))
            .when(self.legend.is_some(), |el| {
                el.child(
                    div()
                        .mt(px(-28.0))
                        .px(px(8.0))
                        .bg(hsla(0.0, 0.0, 0.1, 1.0))
                        .text_size(px(14.0))
                        .font_weight(gpui::FontWeight::MEDIUM)
                        .text_color(hsla(0.0, 0.0, 0.7, 1.0))
                        .child(self.legend.unwrap_or_default())
                )
            })
            .children(self.children)
    }
}
