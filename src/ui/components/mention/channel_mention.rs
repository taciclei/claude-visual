//! Channel mention component

use gpui::prelude::*;
use gpui::*;

/// Channel mention component
#[derive(IntoElement)]
pub struct ChannelMention {
    id: ElementId,
    name: SharedString,
    description: Option<SharedString>,
    pub(crate) member_count: Option<u32>,
    pub(crate) is_private: bool,
}

impl ChannelMention {
    pub fn new(id: impl Into<ElementId>, name: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            member_count: None,
            is_private: false,
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn member_count(mut self, count: u32) -> Self {
        self.member_count = Some(count);
        self
    }

    pub fn is_private(mut self, is_private: bool) -> Self {
        self.is_private = is_private;
        self
    }
}

impl RenderOnce for ChannelMention {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let prefix = if self.is_private { "🔒" } else { "#" };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(4.0))
            .px(px(6.0))
            .py(px(2.0))
            .bg(hsla(0.55, 0.5, 0.4, 0.2))
            .rounded(px(4.0))
            .cursor_pointer()
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(hsla(0.55, 0.7, 0.6, 1.0))
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .child(format!("{}{}", prefix, self.name)),
            )
            .when_some(self.member_count, |el, count| {
                el.child(
                    div()
                        .text_size(px(11.0))
                        .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                        .child(format!("({} members)", count)),
                )
            })
    }
}
