//! Inline change indicator component

use gpui::*;
use gpui::prelude::*;

/// Inline change indicator
#[derive(IntoElement)]
pub struct InlineChange {
    id: ElementId,
    old_text: SharedString,
    new_text: SharedString,
    show_old: bool,
    strikethrough: bool,
}

impl InlineChange {
    pub fn new(
        id: impl Into<ElementId>,
        old: impl Into<SharedString>,
        new: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            old_text: old.into(),
            new_text: new.into(),
            show_old: true,
            strikethrough: true,
        }
    }

    pub fn show_old(mut self, show: bool) -> Self {
        self.show_old = show;
        self
    }

    pub fn strikethrough(mut self, enable: bool) -> Self {
        self.strikethrough = enable;
        self
    }
}

impl RenderOnce for InlineChange {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap_1()
            .when(self.show_old, |d| {
                d.child(
                    div()
                        .px_1()
                        .rounded(px(2.0))
                        .bg(rgba(0xef44441a))
                        .text_color(rgb(0xef4444))
                        .when(self.strikethrough, |d| d.line_through())
                        .child(self.old_text.clone()),
                )
            })
            .child(
                div()
                    .px_1()
                    .rounded(px(2.0))
                    .bg(rgba(0x22c55e1a))
                    .text_color(rgb(0x22c55e))
                    .child(self.new_text.clone()),
            )
    }
}
