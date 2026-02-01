//! Separator with text in the middle

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Separator with text in the middle
#[derive(IntoElement)]
pub struct LabeledSeparator {
    label: SharedString,
    position: LabelPosition,
    orientation: SeparatorOrientation,
    color: Option<Hsla>,
    label_color: Option<Hsla>,
    label_background: Option<Hsla>,
    thickness: SeparatorThickness,
}

impl LabeledSeparator {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            position: LabelPosition::Center,
            orientation: SeparatorOrientation::Horizontal,
            color: None,
            label_color: None,
            label_background: None,
            thickness: SeparatorThickness::Default,
        }
    }

    pub fn position(mut self, position: LabelPosition) -> Self {
        self.position = position;
        self
    }

    pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn label_color(mut self, color: Hsla) -> Self {
        self.label_color = Some(color);
        self
    }

    pub fn label_background(mut self, color: Hsla) -> Self {
        self.label_background = Some(color);
        self
    }

    pub fn thickness(mut self, thickness: SeparatorThickness) -> Self {
        self.thickness = thickness;
        self
    }
}

impl RenderOnce for LabeledSeparator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let line_color = self.color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.25,
            a: 1.0,
        });
        let label_color = self.label_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.5,
            a: 1.0,
        });
        let thickness = self.thickness.pixels();

        let line = || div().h(px(thickness)).flex_1().bg(line_color);

        let label_el = div()
            .px_3()
            .text_size(px(12.0))
            .text_color(label_color)
            .when_some(self.label_background, |d, bg| d.bg(bg))
            .child(self.label);

        let mut container = div().flex().items_center().w_full().gap_2();

        match self.position {
            LabelPosition::Start => {
                container = container.child(label_el).child(line());
            }
            LabelPosition::Center => {
                container = container.child(line()).child(label_el).child(line());
            }
            LabelPosition::End => {
                container = container.child(line()).child(label_el);
            }
        }

        container
    }
}
