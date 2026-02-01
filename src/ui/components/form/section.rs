//! Form section - groups related fields with a title

use gpui::prelude::*;
use gpui::*;

#[derive(IntoElement)]
pub struct FormSection {
    title: Option<SharedString>,
    description: Option<SharedString>,
    children: Vec<gpui::AnyElement>,
    collapsible: bool,
    collapsed: bool,
}

impl FormSection {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            children: Vec::new(),
            collapsible: false,
            collapsed: false,
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
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

    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }
}

impl Default for FormSection {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for FormSection {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .when(self.title.is_some(), |el| {
                el.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .pb(px(8.0))
                        .border_b_1()
                        .border_color(hsla(0.0, 0.0, 0.2, 1.0))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .when(self.collapsible, |el| {
                                    el.cursor_pointer().child(
                                        div()
                                            .text_size(px(10.0))
                                            .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                            .child(if self.collapsed { "▶" } else { "▼" }),
                                    )
                                })
                                .child(
                                    div()
                                        .text_size(px(16.0))
                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                        .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                                        .child(self.title.unwrap_or_default()),
                                ),
                        )
                        .when(self.description.is_some(), |el| {
                            el.child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                    .child(self.description.unwrap_or_default()),
                            )
                        }),
                )
            })
            .when(!self.collapsed, |el| {
                el.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(16.0))
                        .children(self.children),
                )
            })
    }
}
