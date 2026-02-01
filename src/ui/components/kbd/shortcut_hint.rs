//! Shortcut hint component with label

use gpui::*;
use gpui::prelude::*;
use super::types::KbdSize;
use super::keyboard_shortcut::KeyboardShortcut;

/// Shortcut hint with label
#[derive(Clone, IntoElement)]
pub struct ShortcutHint {
    label: String,
    pub(crate) shortcut: KeyboardShortcut,
    align_right: bool,
}

impl ShortcutHint {
    pub fn new(label: impl Into<String>, shortcut: &str) -> Self {
        Self {
            label: label.into(),
            shortcut: KeyboardShortcut::parse(shortcut),
            align_right: true,
        }
    }

    pub fn align_left(mut self) -> Self {
        self.align_right = false;
        self
    }

    pub fn size(mut self, size: KbdSize) -> Self {
        self.shortcut = self.shortcut.size(size);
        self
    }
}

impl RenderOnce for ShortcutHint {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let _text_muted = hsla(0.0, 0.0, 0.5, 1.0);

        let mut container = div()
            .flex()
            .items_center()
            .justify_between()
            .gap_4();

        if self.align_right {
            container = container
                .child(
                    div()
                        .text_sm()
                        .text_color(text)
                        .child(self.label)
                )
                .child(self.shortcut);
        } else {
            container = container
                .child(self.shortcut)
                .child(
                    div()
                        .text_sm()
                        .text_color(text)
                        .child(self.label)
                );
        }

        container
    }
}
