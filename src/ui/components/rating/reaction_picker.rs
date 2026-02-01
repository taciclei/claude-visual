//! Emoji reaction picker component

use gpui::*;
use gpui::prelude::*;

/// Emoji reaction picker
#[derive(Clone)]
pub struct ReactionPicker {
    pub(crate) reactions: Vec<(String, u32)>, // (emoji, count)
    pub(crate) selected: Vec<String>,
}

impl ReactionPicker {
    pub fn new() -> Self {
        Self {
            reactions: Vec::new(),
            selected: Vec::new(),
        }
    }

    pub fn reaction(mut self, emoji: impl Into<String>, count: u32) -> Self {
        self.reactions.push((emoji.into(), count));
        self
    }

    pub fn selected(mut self, emojis: Vec<impl Into<String>>) -> Self {
        self.selected = emojis.into_iter().map(|e| e.into()).collect();
        self
    }
}

impl Default for ReactionPicker {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ReactionPicker {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let muted = hsla(0.0, 0.0, 0.5, 1.0);
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.2, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        div()
            .flex()
            .items_center()
            .gap_1()
            .children(self.reactions.into_iter().map(|(emoji, count)| {
                let is_selected = self.selected.contains(&emoji);

                div()
                    .h(px(28.0))
                    .px_2()
                    .rounded_full()
                    .border_1()
                    .when(is_selected, |d| {
                        d.border_color(accent)
                            .bg(accent.opacity(0.15))
                    })
                    .when(!is_selected, |d| {
                        d.border_color(border)
                            .bg(surface)
                    })
                    .flex()
                    .items_center()
                    .gap_1()
                    .cursor_pointer()
                    .hover(|s| s.bg(surface_hover))
                    .child(
                        div()
                            .text_sm()
                            .child(emoji)
                    )
                    .when(count > 0, |d| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(if is_selected { accent } else { muted })
                                .child(count.to_string())
                        )
                    })
            }))
            // Add reaction button
            .child(
                div()
                    .size(px(28.0))
                    .rounded_full()
                    .border_1()
                    .border_color(border)
                    .bg(surface)
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .hover(|s| s.bg(surface_hover))
                    .child(
                        div()
                            .text_sm()
                            .text_color(muted)
                            .child("+")
                    )
            )
    }
}
