//! Button component

use gpui::prelude::*;
use gpui::prelude::*;
use gpui::prelude::*;
use gpui::*;

/// Button variant styles
#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Danger,
}

/// Button size
#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Reusable button component
pub struct Button {
    label: SharedString,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    icon: Option<SharedString>,
}

impl Button {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::default(),
            size: ButtonSize::default(),
            disabled: false,
            icon: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // Get colors based on variant
        let (bg, bg_hover, text) = match self.variant {
            ButtonVariant::Primary => (
                hsla(210.0 / 360.0, 0.80, 0.55, 1.0),
                hsla(210.0 / 360.0, 0.80, 0.60, 1.0),
                hsla(0.0, 0.0, 1.0, 1.0),
            ),
            ButtonVariant::Secondary => (
                hsla(220.0 / 360.0, 0.13, 0.15, 1.0),
                hsla(220.0 / 360.0, 0.13, 0.20, 1.0),
                hsla(0.0, 0.0, 0.93, 1.0),
            ),
            ButtonVariant::Ghost => (
                hsla(0.0, 0.0, 0.0, 0.0),
                hsla(220.0 / 360.0, 0.13, 0.15, 1.0),
                hsla(0.0, 0.0, 0.93, 1.0),
            ),
            ButtonVariant::Danger => (
                hsla(0.0, 0.84, 0.60, 1.0),
                hsla(0.0, 0.84, 0.65, 1.0),
                hsla(0.0, 0.0, 1.0, 1.0),
            ),
        };

        // Get padding based on size
        let (px_val, py_val, font_size) = match self.size {
            ButtonSize::Small => (px(8.0), px(4.0), px(12.0)),
            ButtonSize::Medium => (px(12.0), px(6.0), px(14.0)),
            ButtonSize::Large => (px(16.0), px(8.0), px(16.0)),
        };

        let mut element = div()
            .px(px_val)
            .py(py_val)
            .rounded_md()
            .text_size(font_size)
            .font_weight(FontWeight::MEDIUM)
            .bg(bg)
            .text_color(text)
            .flex()
            .items_center()
            .justify_center()
            .gap_2();

        if !self.disabled {
            element = element.hover(|style| style.bg(bg_hover)).cursor_pointer();
        } else {
            element = element.opacity(0.5);
        }

        if let Some(icon) = self.icon {
            element = element.child(div().child(icon));
        }

        element.child(self.label)
    }
}
