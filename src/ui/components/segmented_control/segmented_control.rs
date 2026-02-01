//! Main segmented control component

use gpui::prelude::*;
use gpui::*;

use super::types::*;

/// Segmented control component
#[derive(IntoElement)]
pub struct SegmentedControl {
    id: ElementId,
    segments: Vec<Segment>,
    selected: Option<SharedString>,
    size: SegmentedSize,
    variant: SegmentedVariant,
    full_width: bool,
    disabled: bool,
}

impl SegmentedControl {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            segments: Vec::new(),
            selected: None,
            size: SegmentedSize::default(),
            variant: SegmentedVariant::default(),
            full_width: false,
            disabled: false,
        }
    }

    pub fn segments(mut self, segments: Vec<Segment>) -> Self {
        self.segments = segments;
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

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for SegmentedControl {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let height = self.size.height();
        let font_size = self.size.font_size();
        let padding = self.size.padding();

        let (bg_color, border_color) = match self.variant {
            SegmentedVariant::Filled => (hsla(0.0, 0.0, 0.1, 1.0), hsla(0.0, 0.0, 0.2, 1.0)),
            SegmentedVariant::Outline => (hsla(0.0, 0.0, 0.0, 0.0), hsla(0.0, 0.0, 0.3, 1.0)),
            SegmentedVariant::Ghost => (hsla(0.0, 0.0, 0.0, 0.0), hsla(0.0, 0.0, 0.0, 0.0)),
            SegmentedVariant::Pills => (hsla(0.0, 0.0, 0.1, 1.0), hsla(0.0, 0.0, 0.0, 0.0)),
        };

        let gap = if self.variant == SegmentedVariant::Pills {
            4.0
        } else {
            0.0
        };
        let container_padding = if self.variant == SegmentedVariant::Pills {
            4.0
        } else {
            2.0
        };

        div()
            .id(self.id)
            .flex()
            .gap(px(gap))
            .p(px(container_padding))
            .h(px(height))
            .bg(bg_color)
            .border_1()
            .border_color(border_color)
            .rounded(px(6.0))
            .when(self.full_width, |el| el.w_full())
            .children(self.segments.iter().map(|segment| {
                let is_selected = self.selected.as_ref() == Some(&segment.id);
                let is_disabled = self.disabled || segment.disabled;

                let (seg_bg, seg_text) = if is_selected {
                    match self.variant {
                        SegmentedVariant::Filled => {
                            (hsla(0.0, 0.0, 0.25, 1.0), hsla(0.0, 0.0, 0.95, 1.0))
                        }
                        SegmentedVariant::Outline => {
                            (hsla(0.6, 0.7, 0.5, 0.15), hsla(0.6, 0.7, 0.5, 1.0))
                        }
                        SegmentedVariant::Ghost => {
                            (hsla(0.6, 0.7, 0.5, 0.15), hsla(0.6, 0.7, 0.5, 1.0))
                        }
                        SegmentedVariant::Pills => {
                            (hsla(0.6, 0.7, 0.5, 1.0), hsla(0.0, 0.0, 1.0, 1.0))
                        }
                    }
                } else {
                    (hsla(0.0, 0.0, 0.0, 0.0), hsla(0.0, 0.0, 0.6, 1.0))
                };

                let opacity = if is_disabled { 0.5 } else { 1.0 };

                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap(px(4.0))
                    .px(px(padding))
                    .flex_1()
                    .h_full()
                    .bg(seg_bg)
                    .rounded(px(4.0))
                    .opacity(opacity)
                    .when(!is_disabled, |el| el.cursor_pointer())
                    .when_some(segment.icon.clone(), |el, icon| {
                        el.child(
                            div()
                                .text_size(px(font_size))
                                .text_color(seg_text)
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
                            .text_color(seg_text)
                            .child(segment.label.clone()),
                    )
                    .when_some(segment.badge.clone(), |el, badge| {
                        el.child(
                            div()
                                .px(px(4.0))
                                .py(px(1.0))
                                .bg(hsla(0.0, 0.7, 0.5, 1.0))
                                .rounded(px(8.0))
                                .text_size(px(10.0))
                                .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                                .child(badge),
                        )
                    })
            }))
    }
}
