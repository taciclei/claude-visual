//! View switcher component

use gpui::prelude::*;
use gpui::*;

use super::types::*;

/// View switcher - horizontal tabs-like segmented control
#[derive(IntoElement)]
pub struct ViewSwitcher {
    id: ElementId,
    views: Vec<(SharedString, SharedString, Option<SharedString>)>, // (id, label, icon)
    selected: Option<SharedString>,
    size: SegmentedSize,
    show_indicator: bool,
}

impl ViewSwitcher {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            views: Vec::new(),
            selected: None,
            size: SegmentedSize::default(),
            show_indicator: true,
        }
    }

    pub fn views(
        mut self,
        views: Vec<(
            impl Into<SharedString>,
            impl Into<SharedString>,
            Option<impl Into<SharedString>>,
        )>,
    ) -> Self {
        self.views = views
            .into_iter()
            .map(|(id, label, icon)| (id.into(), label.into(), icon.map(|i| i.into())))
            .collect();
        self
    }

    pub fn selected(mut self, selected: impl Into<SharedString>) -> Self {
        self.selected = Some(selected.into());
        self
    }

    pub fn size(mut self, size: SegmentedSize) -> Self {
        self.size = size;
        self
    }

    pub fn show_indicator(mut self, show: bool) -> Self {
        self.show_indicator = show;
        self
    }
}

impl RenderOnce for ViewSwitcher {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let height = self.size.height() + 8.0;
        let font_size = self.size.font_size();

        div()
            .id(self.id)
            .flex()
            .h(px(height))
            .border_b_1()
            .border_color(hsla(0.0, 0.0, 0.2, 1.0))
            .children(self.views.iter().map(|(id, label, icon)| {
                let is_selected = self.selected.as_ref() == Some(id);

                let text_color = if is_selected {
                    hsla(0.6, 0.7, 0.5, 1.0)
                } else {
                    hsla(0.0, 0.0, 0.5, 1.0)
                };

                div()
                    .relative()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap(px(6.0))
                    .px(px(16.0))
                    .h_full()
                    .cursor_pointer()
                    .when_some(icon.clone(), |el, icon| {
                        el.child(
                            div()
                                .text_size(px(font_size + 2.0))
                                .text_color(text_color)
                                .child(icon),
                        )
                    })
                    .child(
                        div()
                            .text_size(px(font_size))
                            .font_weight(if is_selected {
                                FontWeight::MEDIUM
                            } else {
                                FontWeight::NORMAL
                            })
                            .text_color(text_color)
                            .child(label.clone()),
                    )
                    .when(is_selected && self.show_indicator, |el| {
                        el.child(
                            div()
                                .absolute()
                                .bottom_0()
                                .left_0()
                                .right_0()
                                .h(px(2.0))
                                .bg(hsla(0.6, 0.7, 0.5, 1.0)),
                        )
                    })
            }))
    }
}
