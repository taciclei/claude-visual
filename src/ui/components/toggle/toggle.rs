//! Basic toggle button component

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Basic toggle button component
#[derive(IntoElement)]
pub struct Toggle {
    id: ElementId,
    pub(crate) pressed: bool,
    label: Option<SharedString>,
    icon: Option<SharedString>,
    pub(crate) size: ToggleSize,
    pub(crate) variant: ToggleVariant,
    disabled: bool,
    aria_label: Option<SharedString>,
}

impl Toggle {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            pressed: false,
            label: None,
            icon: None,
            size: ToggleSize::default(),
            variant: ToggleVariant::default(),
            disabled: false,
            aria_label: None,
        }
    }

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    fn get_size_styles(&self) -> (f32, f32, f32) {
        match self.size {
            ToggleSize::Small => (28.0, 8.0, 12.0),
            ToggleSize::Medium => (36.0, 12.0, 14.0),
            ToggleSize::Large => (44.0, 16.0, 16.0),
        }
    }
}

impl RenderOnce for Toggle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (height, padding, font_size) = self.get_size_styles();
        let active_color = hsla(0.6, 0.7, 0.5, 1.0);

        let mut button = div()
            .id(self.id)
            .h(px(height))
            .flex()
            .items_center()
            .justify_center()
            .gap(px(6.0))
            .rounded(px(6.0))
            .cursor_pointer();

        // Apply width based on content
        if self.label.is_some() {
            button = button.px(px(padding));
        } else {
            button = button.w(px(height));
        }

        // Apply variant and pressed state styles
        button = match self.variant {
            ToggleVariant::Default => {
                if self.pressed {
                    button.bg(active_color).text_color(hsla(0.0, 0.0, 0.0, 1.0))
                } else {
                    button
                        .bg(hsla(0.0, 0.0, 0.15, 1.0))
                        .hover(|style| style.bg(hsla(0.0, 0.0, 0.2, 1.0)))
                }
            }
            ToggleVariant::Outline => {
                button = button.border_1();
                if self.pressed {
                    button
                        .border_color(active_color)
                        .bg(hsla(0.6, 0.5, 0.5, 0.2))
                        .text_color(active_color)
                } else {
                    button
                        .border_color(hsla(0.0, 0.0, 0.3, 1.0))
                        .hover(|style| style.bg(hsla(0.0, 0.0, 0.1, 1.0)))
                }
            }
            ToggleVariant::Ghost => {
                if self.pressed {
                    button
                        .bg(hsla(0.6, 0.5, 0.5, 0.2))
                        .text_color(active_color)
                } else {
                    button.hover(|style| style.bg(hsla(0.0, 0.0, 0.15, 1.0)))
                }
            }
            ToggleVariant::Subtle => {
                if self.pressed {
                    button
                        .bg(hsla(0.0, 0.0, 0.2, 1.0))
                        .text_color(hsla(0.0, 0.0, 0.95, 1.0))
                } else {
                    button
                        .bg(hsla(0.0, 0.0, 0.1, 1.0))
                        .hover(|style| style.bg(hsla(0.0, 0.0, 0.15, 1.0)))
                }
            }
        };

        if self.disabled {
            button = button.opacity(0.5).cursor_not_allowed();
        }

        // Icon
        if let Some(icon) = self.icon {
            button = button.child(
                div()
                    .text_size(px(font_size))
                    .child(icon)
            );
        }

        // Label
        if let Some(label) = self.label {
            button = button.child(
                div()
                    .text_size(px(font_size))
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .child(label)
            );
        }

        button
    }
}
