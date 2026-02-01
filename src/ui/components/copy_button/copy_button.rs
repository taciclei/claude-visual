//! Main copy button component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Copy button component
#[derive(IntoElement)]
pub struct CopyButton {
    pub(crate) id: ElementId,
    pub(crate) content: SharedString,
    pub(crate) size: CopyButtonSize,
    pub(crate) variant: CopyButtonVariant,
    pub(crate) state: CopyState,
    pub(crate) show_label: bool,
    pub(crate) label: SharedString,
    pub(crate) copied_label: SharedString,
    pub(crate) disabled: bool,
    pub(crate) timeout_ms: u32,
}

impl CopyButton {
    pub fn new(id: impl Into<ElementId>, content: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
            size: CopyButtonSize::default(),
            variant: CopyButtonVariant::default(),
            state: CopyState::default(),
            show_label: false,
            label: "Copy".into(),
            copied_label: "Copied!".into(),
            disabled: false,
            timeout_ms: 2000,
        }
    }

    pub fn size(mut self, size: CopyButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: CopyButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn state(mut self, state: CopyState) -> Self {
        self.state = state;
        self
    }

    pub fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    pub fn copied_label(mut self, label: impl Into<SharedString>) -> Self {
        self.copied_label = label.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn timeout_ms(mut self, ms: u32) -> Self {
        self.timeout_ms = ms;
        self
    }

    fn get_size_styles(&self) -> (f32, f32, f32) {
        match self.size {
            CopyButtonSize::Small => (24.0, 6.0, 12.0),
            CopyButtonSize::Medium => (32.0, 8.0, 14.0),
            CopyButtonSize::Large => (40.0, 10.0, 16.0),
        }
    }
}

impl RenderOnce for CopyButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (size, padding, icon_size) = self.get_size_styles();

        let (icon, color) = match self.state {
            CopyState::Idle => ("📋", hsla(0.0, 0.0, 0.6, 1.0)),
            CopyState::Copying => ("⏳", hsla(0.0, 0.0, 0.5, 1.0)),
            CopyState::Copied => ("✓", hsla(0.35, 0.7, 0.45, 1.0)),
            CopyState::Error => ("✗", hsla(0.0, 0.7, 0.5, 1.0)),
        };

        let label_text = match self.state {
            CopyState::Copied => self.copied_label.clone(),
            _ => self.label.clone(),
        };

        let mut button = div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .gap(px(4.0))
            .cursor_pointer()
            .rounded(px(6.0));

        // Apply size
        if self.show_label {
            button = button.h(px(size)).px(px(padding * 1.5));
        } else {
            button = button.size(px(size));
        }

        // Apply variant styles
        button = match self.variant {
            CopyButtonVariant::Default => button
                .bg(hsla(0.0, 0.0, 0.15, 1.0))
                .hover(|style| style.bg(hsla(0.0, 0.0, 0.2, 1.0))),
            CopyButtonVariant::Ghost => button.hover(|style| style.bg(hsla(0.0, 0.0, 0.15, 1.0))),
            CopyButtonVariant::Outline => button
                .border_1()
                .border_color(hsla(0.0, 0.0, 0.3, 1.0))
                .hover(|style| style.bg(hsla(0.0, 0.0, 0.1, 1.0))),
            CopyButtonVariant::Subtle => button
                .bg(hsla(0.0, 0.0, 0.1, 1.0))
                .hover(|style| style.bg(hsla(0.0, 0.0, 0.15, 1.0))),
        };

        // Apply state
        if self.state == CopyState::Copied {
            button = button.bg(hsla(0.35, 0.4, 0.2, 0.3));
        }

        if self.disabled {
            button = button.opacity(0.5).cursor_not_allowed();
        }

        button
            .child(div().text_size(px(icon_size)).text_color(color).child(icon))
            .when(self.show_label, |el| {
                el.child(
                    div()
                        .text_size(px(icon_size - 2.0))
                        .text_color(color)
                        .child(label_text),
                )
            })
    }
}
