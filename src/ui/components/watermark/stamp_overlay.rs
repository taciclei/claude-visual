use super::types::{StampSize, StampType, WatermarkPosition};
use gpui::prelude::*;
use gpui::*;

/// Draft/confidential stamp overlay
#[derive(IntoElement)]
pub struct StampOverlay {
    id: ElementId,
    text: SharedString,
    stamp_type: StampType,
    position: WatermarkPosition,
    rotation: f32,
    opacity: f32,
    size: StampSize,
}

impl StampOverlay {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            text: "DRAFT".into(),
            stamp_type: StampType::default(),
            position: WatermarkPosition::Center,
            rotation: -15.0,
            opacity: 0.3,
            size: StampSize::default(),
        }
    }

    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = text.into();
        self
    }

    pub fn stamp_type(mut self, stamp_type: StampType) -> Self {
        let default_text = stamp_type.default_text().to_string();
        self.stamp_type = stamp_type;
        if self.text.is_empty() || self.text == "DRAFT" {
            self.text = default_text.into();
        }
        self
    }

    pub fn position(mut self, position: WatermarkPosition) -> Self {
        self.position = position;
        self
    }

    pub fn rotation(mut self, degrees: f32) -> Self {
        self.rotation = degrees;
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    pub fn size(mut self, size: StampSize) -> Self {
        self.size = size;
        self
    }
}

impl RenderOnce for StampOverlay {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.stamp_type.color();
        let font_size = self.size.font_size();
        let padding = self.size.padding();

        let base = div().id(self.id).absolute().inset_0().overflow_hidden();

        let positioned = match self.position {
            WatermarkPosition::Center => base.flex().items_center().justify_center(),
            WatermarkPosition::TopLeft => base.flex().items_start().justify_start().p_8(),
            WatermarkPosition::TopRight => base.flex().items_start().justify_end().p_8(),
            WatermarkPosition::BottomLeft => base.flex().items_end().justify_start().p_8(),
            WatermarkPosition::BottomRight => base.flex().items_end().justify_end().p_8(),
            _ => base.flex().items_center().justify_center(),
        };

        positioned.child(
            div()
                .px(px(padding * 2.0))
                .py(px(padding))
                .border_4()
                .border_color(color.opacity(self.opacity))
                .rounded_md()
                .text_size(px(font_size))
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(color.opacity(self.opacity))
                .child(self.text.clone()),
        )
    }
}
