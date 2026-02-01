use gpui::*;
use gpui::prelude::*;
use super::types::{WatermarkPosition, WatermarkVariant};

/// Text watermark component
#[derive(IntoElement)]
pub struct Watermark {
    id: ElementId,
    text: SharedString,
    position: WatermarkPosition,
    variant: WatermarkVariant,
    opacity: f32,
    rotation: f32,
    font_size: f32,
    color: gpui::Hsla,
    repeat_x: usize,
    repeat_y: usize,
    gap: f32,
    selectable: bool,
}

impl Watermark {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            position: WatermarkPosition::default(),
            variant: WatermarkVariant::default(),
            opacity: 0.1,
            rotation: -30.0,
            font_size: 48.0,
            color: rgba(0x000000ff).into(),
            repeat_x: 3,
            repeat_y: 3,
            gap: 100.0,
            selectable: false,
        }
    }

    pub fn position(mut self, position: WatermarkPosition) -> Self {
        self.position = position;
        self
    }

    pub fn variant(mut self, variant: WatermarkVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    pub fn rotation(mut self, degrees: f32) -> Self {
        self.rotation = degrees;
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn color(mut self, color: gpui::Hsla) -> Self {
        self.color = color;
        self
    }

    pub fn repeat(mut self, x: usize, y: usize) -> Self {
        self.repeat_x = x;
        self.repeat_y = y;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    fn render_single(&self) -> impl IntoElement {
        div()
            .text_size(px(self.font_size))
            .text_color(self.color.opacity(self.opacity))
            .font_weight(gpui::FontWeight::BOLD)
            .child(self.text.clone())
    }

    fn render_tiled(&self) -> impl IntoElement {
        div()
            .absolute()
            .inset_0()
            .overflow_hidden()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(self.gap))
            .children((0..self.repeat_y).map(|_row| {
                div()
                    .flex()
                    .gap(px(self.gap))
                    .children((0..self.repeat_x).map(|_col| {
                        div()
                            .text_size(px(self.font_size))
                            .text_color(self.color.opacity(self.opacity))
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(self.text.clone())
                    }))
            }))
    }
}

impl RenderOnce for Watermark {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let id = self.id.clone();
        let base = div()
            .id(id)
            .absolute()
            .inset_0()
            .overflow_hidden();

        if self.position == WatermarkPosition::Tiled {
            return base.child(self.render_tiled());
        }

        let positioned = match self.position {
            WatermarkPosition::Center => base.flex().items_center().justify_center(),
            WatermarkPosition::TopLeft => base.flex().items_start().justify_start().p_4(),
            WatermarkPosition::TopRight => base.flex().items_start().justify_end().p_4(),
            WatermarkPosition::BottomLeft => base.flex().items_end().justify_start().p_4(),
            WatermarkPosition::BottomRight => base.flex().items_end().justify_end().p_4(),
            WatermarkPosition::TopCenter => base.flex().items_start().justify_center().p_4(),
            WatermarkPosition::BottomCenter => base.flex().items_end().justify_center().p_4(),
            WatermarkPosition::LeftCenter => base.flex().items_center().justify_start().p_4(),
            WatermarkPosition::RightCenter => base.flex().items_center().justify_end().p_4(),
            WatermarkPosition::Tiled => base, // Handled above
        };

        positioned.child(self.render_single())
    }
}
