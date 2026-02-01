//! OTP Input component - for verification codes

use gpui::*;
use gpui::prelude::*;
use super::types::*;

#[derive(IntoElement)]
pub struct OtpInput {
    id: ElementId,
    length: usize,
    value: SharedString,
    masked: bool,
    size: OtpSize,
    variant: OtpVariant,
    state: OtpState,
    disabled: bool,
    auto_focus: bool,
    separator_after: Option<usize>,
    background: Option<gpui::Hsla>,
    border_color: Option<gpui::Hsla>,
    focus_color: Option<gpui::Hsla>,
}

impl OtpInput {
    pub fn new(id: impl Into<ElementId>, length: usize) -> Self {
        Self {
            id: id.into(),
            length,
            value: "".into(),
            masked: false,
            size: OtpSize::default(),
            variant: OtpVariant::default(),
            state: OtpState::default(),
            disabled: false,
            auto_focus: true,
            separator_after: None,
            background: None,
            border_color: None,
            focus_color: None,
        }
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    pub fn masked(mut self, masked: bool) -> Self {
        self.masked = masked;
        self
    }

    pub fn size(mut self, size: OtpSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: OtpVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn state(mut self, state: OtpState) -> Self {
        self.state = state;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn auto_focus(mut self, auto_focus: bool) -> Self {
        self.auto_focus = auto_focus;
        self
    }

    pub fn separator_after(mut self, position: usize) -> Self {
        self.separator_after = Some(position);
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn border_color(mut self, color: gpui::Hsla) -> Self {
        self.border_color = Some(color);
        self
    }

    pub fn focus_color(mut self, color: gpui::Hsla) -> Self {
        self.focus_color = Some(color);
        self
    }

    fn get_size_styles(&self) -> (f32, f32, f32) {
        match self.size {
            OtpSize::Small => (36.0, 32.0, 16.0),
            OtpSize::Medium => (48.0, 44.0, 20.0),
            OtpSize::Large => (56.0, 52.0, 24.0),
        }
    }

    fn get_state_color(&self) -> gpui::Hsla {
        match self.state {
            OtpState::Default => self.border_color.unwrap_or(hsla(0.0, 0.0, 0.3, 1.0)),
            OtpState::Focused => self.focus_color.unwrap_or(hsla(0.6, 0.7, 0.5, 1.0)),
            OtpState::Success => hsla(0.35, 0.7, 0.45, 1.0),
            OtpState::Error => hsla(0.0, 0.7, 0.5, 1.0),
        }
    }
}

impl RenderOnce for OtpInput {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (box_size, _, font_size) = self.get_size_styles();
        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.12, 1.0));
        let border_color = self.get_state_color();

        let chars: Vec<char> = self.value.chars().collect();

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(8.0))
            .when(self.disabled, |el| el.opacity(0.5))
            .children((0..self.length).map(|i| {
                let char_at = chars.get(i).cloned();
                let display_char = if self.masked && char_at.is_some() {
                    "•".to_string()
                } else {
                    char_at.map(|c| c.to_string()).unwrap_or_default()
                };

                let is_current = i == chars.len() && self.state == OtpState::Focused;

                let mut box_el = div()
                    .size(px(box_size))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(font_size))
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(hsla(0.0, 0.0, 0.95, 1.0));

                // Apply variant styles
                box_el = match self.variant {
                    OtpVariant::Boxes => {
                        box_el
                            .rounded(px(8.0))
                            .border_2()
                            .border_color(if is_current {
                                self.focus_color.unwrap_or(hsla(0.6, 0.7, 0.5, 1.0))
                            } else {
                                border_color
                            })
                            .bg(bg)
                    }
                    OtpVariant::Underline => {
                        box_el
                            .border_b_2()
                            .border_color(if is_current {
                                self.focus_color.unwrap_or(hsla(0.6, 0.7, 0.5, 1.0))
                            } else {
                                border_color
                            })
                    }
                    OtpVariant::Rounded => {
                        box_el
                            .rounded_full()
                            .border_2()
                            .border_color(if is_current {
                                self.focus_color.unwrap_or(hsla(0.6, 0.7, 0.5, 1.0))
                            } else {
                                border_color
                            })
                            .bg(bg)
                    }
                };

                // Add cursor for current position
                if is_current {
                    box_el = box_el.child(
                        div()
                            .w(px(2.0))
                            .h(px(font_size + 4.0))
                            .bg(self.focus_color.unwrap_or(hsla(0.6, 0.7, 0.5, 1.0)))
                    );
                } else if !display_char.is_empty() {
                    box_el = box_el.child(display_char);
                }

                // Wrap with optional separator
                let should_add_separator = self.separator_after
                    .map(|pos| i == pos - 1 && i < self.length - 1)
                    .unwrap_or(false);

                if should_add_separator {
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .child(box_el)
                        .child(
                            div()
                                .text_size(px(font_size))
                                .text_color(hsla(0.0, 0.0, 0.4, 1.0))
                                .child("-")
                        )
                } else {
                    div().child(box_el)
                }
            }))
    }
}
