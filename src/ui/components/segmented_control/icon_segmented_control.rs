//! Icon-only segmented control component

use gpui::prelude::*;
use gpui::*;

use super::types::*;

/// Icon-only segmented control
#[derive(IntoElement)]
pub struct IconSegmentedControl {
    id: ElementId,
    icons: Vec<(SharedString, SharedString)>, // (id, icon)
    selected: Option<SharedString>,
    size: SegmentedSize,
    variant: SegmentedVariant,
}

impl IconSegmentedControl {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            icons: Vec::new(),
            selected: None,
            size: SegmentedSize::default(),
            variant: SegmentedVariant::default(),
        }
    }

    pub fn icons(mut self, icons: Vec<(impl Into<SharedString>, impl Into<SharedString>)>) -> Self {
        self.icons = icons
            .into_iter()
            .map(|(id, icon)| (id.into(), icon.into()))
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

    pub fn variant(mut self, variant: SegmentedVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl RenderOnce for IconSegmentedControl {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let height = self.size.height();
        let icon_size = self.size.font_size() + 4.0;

        let bg_color = match self.variant {
            SegmentedVariant::Filled => hsla(0.0, 0.0, 0.1, 1.0),
            SegmentedVariant::Outline | SegmentedVariant::Ghost => hsla(0.0, 0.0, 0.0, 0.0),
            SegmentedVariant::Pills => hsla(0.0, 0.0, 0.1, 1.0),
        };

        div()
            .id(self.id)
            .flex()
            .gap(px(2.0))
            .p(px(2.0))
            .h(px(height))
            .bg(bg_color)
            .rounded(px(6.0))
            .children(self.icons.iter().map(|(id, icon)| {
                let is_selected = self.selected.as_ref() == Some(id);

                let (seg_bg, seg_text) = if is_selected {
                    (hsla(0.6, 0.7, 0.5, 0.2), hsla(0.6, 0.7, 0.5, 1.0))
                } else {
                    (hsla(0.0, 0.0, 0.0, 0.0), hsla(0.0, 0.0, 0.5, 1.0))
                };

                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(height - 4.0))
                    .h_full()
                    .bg(seg_bg)
                    .rounded(px(4.0))
                    .cursor_pointer()
                    .child(
                        div()
                            .text_size(px(icon_size))
                            .text_color(seg_text)
                            .child(icon.clone()),
                    )
            }))
    }
}
