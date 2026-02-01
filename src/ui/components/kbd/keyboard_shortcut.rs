//! Keyboard shortcut component (combination of keys)

use gpui::*;
use gpui::prelude::*;
use super::types::*;
use super::kbd::Kbd;

/// Keyboard shortcut (combination of keys)
#[derive(Clone, IntoElement)]
pub struct KeyboardShortcut {
    pub(crate) keys: Vec<String>,
    pub(crate) size: KbdSize,
    pub(crate) style: KbdStyle,
    separator: String,
}

impl KeyboardShortcut {
    pub fn new(keys: Vec<impl Into<String>>) -> Self {
        Self {
            keys: keys.into_iter().map(|k| k.into()).collect(),
            size: KbdSize::default(),
            style: KbdStyle::default(),
            separator: "+".to_string(),
        }
    }

    /// Create from a shortcut string like "Cmd+Shift+P"
    pub fn parse(shortcut: &str) -> Self {
        let keys: Vec<String> = shortcut
            .split('+')
            .map(|k| {
                match k.trim().to_lowercase().as_str() {
                    "cmd" | "command" => "⌘".to_string(),
                    "opt" | "option" | "alt" => "⌥".to_string(),
                    "ctrl" | "control" => "⌃".to_string(),
                    "shift" => "⇧".to_string(),
                    "enter" | "return" => "↵".to_string(),
                    "tab" => "⇥".to_string(),
                    "esc" | "escape" => "Esc".to_string(),
                    "space" => "Space".to_string(),
                    "delete" | "backspace" => "⌫".to_string(),
                    "up" => "↑".to_string(),
                    "down" => "↓".to_string(),
                    "left" => "←".to_string(),
                    "right" => "→".to_string(),
                    other => other.to_uppercase(),
                }
            })
            .collect();

        Self::new(keys)
    }

    pub fn size(mut self, size: KbdSize) -> Self {
        self.size = size;
        self
    }

    pub fn style(mut self, style: KbdStyle) -> Self {
        self.style = style;
        self
    }

    pub fn separator(mut self, sep: impl Into<String>) -> Self {
        self.separator = sep.into();
        self
    }

    pub fn no_separator(mut self) -> Self {
        self.separator = String::new();
        self
    }
}

impl RenderOnce for KeyboardShortcut {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let show_separator = !self.separator.is_empty() && self.keys.len() > 1;

        let size = self.size;
        let style = self.style;
        let separator = self.separator.clone();

        div()
            .flex()
            .items_center()
            .gap(px(2.0))
            .children(
                self.keys.into_iter().enumerate().map(move |(idx, key)| {
                    let mut el = div().flex().items_center().gap(px(2.0));

                    if idx > 0 && show_separator {
                        el = el.child(
                            div()
                                .text_xs()
                                .text_color(text_muted)
                                .child(separator.clone())
                        );
                    }

                    el.child(Kbd::new(key).size(size).style(style))
                })
            )
    }
}
