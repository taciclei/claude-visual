//! Main Divider component

use std::sync::Arc;

use gpui::*;
use gpui::prelude::*;

use crate::app::state::AppState;
use super::types::*;

/// Divider component
pub struct Divider {
    app_state: Arc<AppState>,
    /// Orientation
    orientation: DividerOrientation,
    /// Style
    style: DividerStyle,
    /// Thickness
    thickness: DividerThickness,
    /// Optional label text (centered)
    label: Option<String>,
    /// Custom color
    color: Option<Hsla>,
    /// Margin/spacing around divider
    margin: f32,
}

impl Divider {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            orientation: DividerOrientation::default(),
            style: DividerStyle::default(),
            thickness: DividerThickness::default(),
            label: None,
            color: None,
            margin: 8.0,
        }
    }

    /// Create a horizontal divider
    pub fn horizontal(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        let mut divider = Self::new(app_state, cx);
        divider.orientation = DividerOrientation::Horizontal;
        divider
    }

    /// Create a vertical divider
    pub fn vertical(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        let mut divider = Self::new(app_state, cx);
        divider.orientation = DividerOrientation::Vertical;
        divider
    }

    /// Set orientation
    pub fn set_orientation(&mut self, orientation: DividerOrientation, cx: &mut Context<Self>) {
        self.orientation = orientation;
        cx.notify();
    }

    /// Set style
    pub fn set_style(&mut self, style: DividerStyle, cx: &mut Context<Self>) {
        self.style = style;
        cx.notify();
    }

    /// Set thickness
    pub fn set_thickness(&mut self, thickness: DividerThickness, cx: &mut Context<Self>) {
        self.thickness = thickness;
        cx.notify();
    }

    /// Set label text
    pub fn set_label(&mut self, label: Option<String>, cx: &mut Context<Self>) {
        self.label = label;
        cx.notify();
    }

    /// Set custom color
    pub fn set_color(&mut self, color: Option<Hsla>, cx: &mut Context<Self>) {
        self.color = color;
        cx.notify();
    }

    /// Set margin
    pub fn set_margin(&mut self, margin: f32, cx: &mut Context<Self>) {
        self.margin = margin;
        cx.notify();
    }
}

impl Render for Divider {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let thickness = self.thickness.pixels();
        let opacity = self.thickness.opacity();
        let color = self.color.unwrap_or(theme.colors.border).opacity(opacity);

        match self.orientation {
            DividerOrientation::Horizontal => {
                if let Some(label) = &self.label {
                    // Divider with label
                    div()
                        .id("divider-horizontal-labeled")
                        .w_full()
                        .flex()
                        .items_center()
                        .gap_3()
                        .my(px(self.margin))
                        // Left line
                        .child(
                            div()
                                .flex_1()
                                .h(px(thickness))
                                .bg(color)
                        )
                        // Label
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .flex_shrink_0()
                                .child(label.clone())
                        )
                        // Right line
                        .child(
                            div()
                                .flex_1()
                                .h(px(thickness))
                                .bg(color)
                        )
                } else {
                    // Simple horizontal line
                    div()
                        .id("divider-horizontal")
                        .w_full()
                        .h(px(thickness))
                        .bg(color)
                        .my(px(self.margin))
                }
            }
            DividerOrientation::Vertical => {
                div()
                    .id("divider-vertical")
                    .w(px(thickness))
                    .h_full()
                    .bg(color)
                    .mx(px(self.margin))
            }
        }
    }
}
