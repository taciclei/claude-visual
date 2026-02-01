//! Speed dial component - FAB with expandable actions

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Speed dial component - FAB with expandable actions
#[derive(IntoElement)]
pub struct SpeedDial {
    id: ElementId,
    icon: SharedString,
    items: Vec<SpeedDialItem>,
    expanded: bool,
    direction: SpeedDialDirection,
    variant: FabVariant,
}

impl SpeedDial {
    pub fn new(id: impl Into<ElementId>, icon: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            icon: icon.into(),
            items: Vec::new(),
            expanded: false,
            direction: SpeedDialDirection::default(),
            variant: FabVariant::default(),
        }
    }

    pub fn items(mut self, items: Vec<SpeedDialItem>) -> Self {
        self.items = items;
        self
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    pub fn direction(mut self, direction: SpeedDialDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn variant(mut self, variant: FabVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl RenderOnce for SpeedDial {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (bg_color, text_color) = self.variant.colors();

        let is_vertical = matches!(self.direction, SpeedDialDirection::Up | SpeedDialDirection::Down);
        let is_reverse = matches!(self.direction, SpeedDialDirection::Up | SpeedDialDirection::Left);

        div()
            .id(self.id)
            .flex()
            .when(is_vertical, |el| el.flex_col())
            .when(is_reverse, |el| el.flex_row_reverse())
            .items_center()
            .gap(px(12.0))
            // Main FAB
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(56.0))
                    .h(px(56.0))
                    .bg(bg_color)
                    .rounded_full()
                    .shadow_lg()
                    .cursor_pointer()
                    .child(
                        div()
                            .text_size(px(24.0))
                            .text_color(text_color)
                            .child(self.icon.clone())
                    )
            )
            // Speed dial items
            .when(self.expanded, |el| {
                el.children(self.items.iter().map(|item| {
                    let opacity = if item.disabled { 0.5 } else { 1.0 };

                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .when(is_reverse, |el| el.flex_row_reverse())
                        .when_some(item.label.clone(), |el, label| {
                            el.child(
                                div()
                                    .px(px(8.0))
                                    .py(px(4.0))
                                    .bg(hsla(0.0, 0.0, 0.15, 0.9))
                                    .rounded(px(4.0))
                                    .text_size(px(12.0))
                                    .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                                    .child(label)
                            )
                        })
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .w(px(40.0))
                                .h(px(40.0))
                                .bg(hsla(0.0, 0.0, 0.2, 1.0))
                                .rounded_full()
                                .shadow_md()
                                .opacity(opacity)
                                .when(!item.disabled, |el| el.cursor_pointer())
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                                        .child(item.icon.clone())
                                )
                        )
                }))
            })
    }
}
