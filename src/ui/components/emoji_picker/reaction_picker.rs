//! Reaction picker component - quick emoji reactions

use gpui::prelude::*;
use gpui::*;

/// Reaction picker - quick emoji reactions
#[derive(IntoElement)]
pub struct ReactionPicker {
    id: ElementId,
    pub(crate) quick_reactions: Vec<SharedString>,
    pub(crate) show_more: bool,
}

impl ReactionPicker {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            quick_reactions: vec![
                "👍".into(),
                "❤️".into(),
                "😂".into(),
                "😮".into(),
                "😢".into(),
                "👏".into(),
            ],
            show_more: true,
        }
    }

    pub fn quick_reactions(mut self, reactions: Vec<impl Into<SharedString>>) -> Self {
        self.quick_reactions = reactions.into_iter().map(|r| r.into()).collect();
        self
    }

    pub fn show_more(mut self, show: bool) -> Self {
        self.show_more = show;
        self
    }
}

impl RenderOnce for ReactionPicker {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(4.0))
            .px(px(8.0))
            .py(px(6.0))
            .bg(hsla(0.0, 0.0, 0.15, 1.0))
            .border_1()
            .border_color(hsla(0.0, 0.0, 0.25, 1.0))
            .rounded(px(20.0))
            .shadow_md()
            .children(self.quick_reactions.iter().map(|emoji| {
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(28.0))
                    .h(px(28.0))
                    .rounded(px(4.0))
                    .cursor_pointer()
                    .child(
                        div()
                            .text_size(px(18.0))
                            .child(emoji.clone())
                    )
            }))
            .when(self.show_more, |el| {
                el.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(px(28.0))
                        .h(px(28.0))
                        .rounded(px(4.0))
                        .cursor_pointer()
                        .child(
                            div()
                                .text_size(px(14.0))
                                .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                .child("+")
                        )
                )
            })
    }
}
