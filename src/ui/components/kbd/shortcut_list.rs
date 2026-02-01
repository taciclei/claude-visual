//! Shortcut list component for help dialogs

use gpui::*;
use gpui::prelude::*;
use super::types::KbdSize;
use super::shortcut_hint::ShortcutHint;

/// Shortcut list (for help dialogs)
#[derive(Clone, IntoElement)]
pub struct ShortcutList {
    title: Option<String>,
    pub(crate) shortcuts: Vec<(String, String)>,
}

impl ShortcutList {
    pub fn new() -> Self {
        Self {
            title: None,
            shortcuts: Vec::new(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn shortcut(mut self, label: impl Into<String>, keys: impl Into<String>) -> Self {
        self.shortcuts.push((label.into(), keys.into()));
        self
    }

    pub fn shortcuts(mut self, list: Vec<(impl Into<String>, impl Into<String>)>) -> Self {
        for (label, keys) in list {
            self.shortcuts.push((label.into(), keys.into()));
        }
        self
    }
}

impl Default for ShortcutList {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ShortcutList {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        div()
            .flex()
            .flex_col()
            .gap_2()
            .when_some(self.title, |d, title| {
                d.child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(text)
                        .pb_2()
                        .border_b_1()
                        .border_color(border)
                        .child(title)
                )
            })
            .children(
                self.shortcuts.into_iter().map(|(label, keys)| {
                    ShortcutHint::new(label, &keys)
                        .size(KbdSize::Small)
                })
            )
    }
}
