//! Button group component

use gpui::prelude::*;
use gpui::*;

use super::types::*;

/// Button group - similar to segmented but for actions
#[derive(IntoElement)]
pub struct ButtonGroup {
    id: ElementId,
    buttons: Vec<(SharedString, SharedString)>, // (id, label)
    size: SegmentedSize,
    variant: SegmentedVariant,
    attached: bool,
}

impl ButtonGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            buttons: Vec::new(),
            size: SegmentedSize::default(),
            variant: SegmentedVariant::default(),
            attached: true,
        }
    }

    pub fn buttons(
        mut self,
        buttons: Vec<(impl Into<SharedString>, impl Into<SharedString>)>,
    ) -> Self {
        self.buttons = buttons
            .into_iter()
            .map(|(id, label)| (id.into(), label.into()))
            .collect();
        self
    }

    pub fn size(mut self, size: SegmentedSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: SegmentedVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn attached(mut self, attached: bool) -> Self {
        self.attached = attached;
        self
    }
}

impl RenderOnce for ButtonGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let height = self.size.height();
        let font_size = self.size.font_size();
        let padding = self.size.padding();

        let gap = if self.attached { 0.0 } else { 4.0 };

        div()
            .id(self.id)
            .flex()
            .gap(px(gap))
            .children(self.buttons.iter().enumerate().map(|(i, (_, label))| {
                let is_first = i == 0;
                let is_last = i == self.buttons.len() - 1;

                let border_radius = if self.attached {
                    if is_first {
                        (6.0, 0.0, 0.0, 6.0) // left rounded
                    } else if is_last {
                        (0.0, 6.0, 6.0, 0.0) // right rounded
                    } else {
                        (0.0, 0.0, 0.0, 0.0) // no rounding
                    }
                } else {
                    (6.0, 6.0, 6.0, 6.0) // all rounded
                };

                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .px(px(padding))
                    .h(px(height))
                    .bg(hsla(0.0, 0.0, 0.15, 1.0))
                    .border_1()
                    .border_color(hsla(0.0, 0.0, 0.25, 1.0))
                    .rounded_tl(px(border_radius.0))
                    .rounded_tr(px(border_radius.1))
                    .rounded_br(px(border_radius.2))
                    .rounded_bl(px(border_radius.3))
                    .when(self.attached && !is_last, |el| el.border_r_0())
                    .cursor_pointer()
                    .child(
                        div()
                            .text_size(px(font_size))
                            .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                            .child(label.clone()),
                    )
            }))
    }
}
