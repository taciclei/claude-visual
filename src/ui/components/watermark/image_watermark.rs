use super::types::WatermarkPosition;
use gpui::prelude::*;
use gpui::*;

/// Image watermark overlay
#[derive(IntoElement)]
pub struct ImageWatermark {
    id: ElementId,
    src: Option<SharedString>,
    position: WatermarkPosition,
    opacity: f32,
    width: Option<f32>,
    height: Option<f32>,
    grayscale: bool,
}

impl ImageWatermark {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            src: None,
            position: WatermarkPosition::BottomRight,
            opacity: 0.3,
            width: None,
            height: None,
            grayscale: false,
        }
    }

    pub fn src(mut self, src: impl Into<SharedString>) -> Self {
        self.src = Some(src.into());
        self
    }

    pub fn position(mut self, position: WatermarkPosition) -> Self {
        self.position = position;
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn grayscale(mut self, grayscale: bool) -> Self {
        self.grayscale = grayscale;
        self
    }
}

impl RenderOnce for ImageWatermark {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let base = div().id(self.id).absolute().inset_0().overflow_hidden();

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
            WatermarkPosition::Tiled => base.flex().items_center().justify_center(),
        };

        positioned.child(
            div()
                .when_some(self.width, |d, w| d.w(px(w)))
                .when_some(self.height, |d, h| d.h(px(h)))
                .opacity(self.opacity)
                .when(self.grayscale, |d| d.opacity(self.opacity * 0.5))
                .bg(rgba(0x8888881a)) // Placeholder for image
                .rounded_md(),
        )
    }
}
