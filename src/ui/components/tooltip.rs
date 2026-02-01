//! Tooltip component

use gpui::prelude::*;
use gpui::*;

/// Tooltip position
#[derive(Debug, Clone, Copy, Default)]
pub enum TooltipPosition {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

/// Tooltip component
pub struct Tooltip {
    text: String,
    shortcut: Option<String>,
    #[allow(dead_code)]
    position: TooltipPosition,
}

impl Tooltip {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            shortcut: None,
            position: TooltipPosition::default(),
        }
    }

    /// Create a tooltip with a keyboard shortcut hint
    pub fn with_shortcut(text: impl Into<String>, shortcut: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            shortcut: Some(shortcut.into()),
            position: TooltipPosition::default(),
        }
    }

    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    /// Add a keyboard shortcut hint
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }
}

impl RenderOnce for Tooltip {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bg = hsla(220.0 / 360.0, 0.13, 0.18, 1.0);
        let border = hsla(220.0 / 360.0, 0.13, 0.25, 1.0);
        let text_color = hsla(0.0, 0.0, 0.93, 1.0);
        let shortcut_color = hsla(210.0 / 360.0, 0.6, 0.6, 1.0);

        div()
            .px_2()
            .py_1()
            .rounded_md()
            .bg(bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .gap_2()
            .text_xs()
            .text_color(text_color)
            .child(self.text.clone())
            .when_some(self.shortcut, |d, shortcut| {
                d.child(
                    div()
                        .px_1()
                        .py(px(1.0))
                        .rounded_sm()
                        .bg(hsla(220.0 / 360.0, 0.2, 0.25, 1.0))
                        .text_xs()
                        .text_color(shortcut_color)
                        .font_family("monospace")
                        .child(shortcut),
                )
            })
    }
}
