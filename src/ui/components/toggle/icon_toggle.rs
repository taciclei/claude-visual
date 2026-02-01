//! Icon toggle component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Icon toggle - toggle with only an icon
#[derive(IntoElement)]
pub struct IconToggle {
    id: ElementId,
    icon_off: SharedString,
    icon_on: SharedString,
    pressed: bool,
    size: ToggleSize,
    disabled: bool,
    tooltip: Option<SharedString>,
}

impl IconToggle {
    pub fn new(
        id: impl Into<ElementId>,
        icon_off: impl Into<SharedString>,
        icon_on: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            icon_off: icon_off.into(),
            icon_on: icon_on.into(),
            pressed: false,
            size: ToggleSize::default(),
            disabled: false,
            tooltip: None,
        }
    }

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self
    }

    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }
}

impl RenderOnce for IconToggle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let size = match self.size {
            ToggleSize::Small => 28.0,
            ToggleSize::Medium => 36.0,
            ToggleSize::Large => 44.0,
        };

        let icon_size = match self.size {
            ToggleSize::Small => 14.0,
            ToggleSize::Medium => 16.0,
            ToggleSize::Large => 20.0,
        };

        let icon = if self.pressed {
            self.icon_on.clone()
        } else {
            self.icon_off.clone()
        };

        let color = if self.pressed {
            hsla(0.6, 0.7, 0.5, 1.0)
        } else {
            hsla(0.0, 0.0, 0.6, 1.0)
        };

        let mut button = div()
            .id(self.id)
            .size(px(size))
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(6.0))
            .cursor_pointer();

        if self.pressed {
            button = button.bg(hsla(0.6, 0.5, 0.5, 0.2));
        } else {
            button = button.hover(|style| style.bg(hsla(0.0, 0.0, 0.15, 1.0)));
        }

        if self.disabled {
            button = button.opacity(0.5).cursor_not_allowed();
        }

        button.child(div().text_size(px(icon_size)).text_color(color).child(icon))
    }
}
