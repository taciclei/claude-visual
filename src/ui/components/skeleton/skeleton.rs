//! Main skeleton component for loading placeholders

use std::sync::Arc;
use gpui::*;
use gpui::prelude::*;
use crate::app::state::AppState;
use super::types::*;

/// Skeleton component for loading placeholders
pub struct Skeleton {
    app_state: Arc<AppState>,
    /// Width (if None, fills parent)
    width: Option<f32>,
    /// Height
    height: f32,
    /// Shape
    shape: SkeletonShape,
    /// Whether to show animation
    animated: bool,
}

impl Skeleton {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            width: None,
            height: 16.0,
            shape: SkeletonShape::default(),
            animated: true,
        }
    }

    /// Create a text line skeleton
    pub fn text(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        let mut skeleton = Self::new(app_state, cx);
        skeleton.height = 12.0;
        skeleton.shape = SkeletonShape::Text;
        skeleton
    }

    /// Create a circular skeleton (avatar placeholder)
    pub fn circle(app_state: Arc<AppState>, size: f32, cx: &mut Context<Self>) -> Self {
        let mut skeleton = Self::new(app_state, cx);
        skeleton.width = Some(size);
        skeleton.height = size;
        skeleton.shape = SkeletonShape::Circle;
        skeleton
    }

    /// Create a rectangular skeleton
    pub fn rect(app_state: Arc<AppState>, width: f32, height: f32, cx: &mut Context<Self>) -> Self {
        let mut skeleton = Self::new(app_state, cx);
        skeleton.width = Some(width);
        skeleton.height = height;
        skeleton.shape = SkeletonShape::Rectangle;
        skeleton
    }

    /// Set width
    pub fn set_width(&mut self, width: Option<f32>, cx: &mut Context<Self>) {
        self.width = width;
        cx.notify();
    }

    /// Set height
    pub fn set_height(&mut self, height: f32, cx: &mut Context<Self>) {
        self.height = height;
        cx.notify();
    }

    /// Set shape
    pub fn set_shape(&mut self, shape: SkeletonShape, cx: &mut Context<Self>) {
        self.shape = shape;
        cx.notify();
    }

    /// Set animated
    pub fn set_animated(&mut self, animated: bool, cx: &mut Context<Self>) {
        self.animated = animated;
        cx.notify();
    }
}

impl Render for Skeleton {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        // Base skeleton color (semi-transparent)
        let base_color = theme.colors.surface_hover;

        let radius = match self.shape {
            SkeletonShape::Rectangle => 4.0,
            SkeletonShape::Circle => self.height / 2.0,
            SkeletonShape::Rounded => 8.0,
            SkeletonShape::Text => 2.0,
        };

        div()
            .id("skeleton")
            .h(px(self.height))
            .when_some(self.width, |d, w| d.w(px(w)))
            .when(self.width.is_none(), |d| d.w_full())
            .rounded(px(radius))
            .bg(base_color)
            // For a pulse effect, we could add a subtle gradient or opacity variation
            // GPUI doesn't have built-in animations, but we simulate loading state
    }
}
