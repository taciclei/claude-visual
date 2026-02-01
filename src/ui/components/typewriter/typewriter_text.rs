//! TypewriterText component - character-by-character reveal

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Typewriter text with character-by-character reveal
#[derive(IntoElement)]
pub struct TypewriterText {
    id: ElementId,
    text: SharedString,
    visible_chars: usize,
    style: TypewriterStyle,
    cursor_style: TypewriterCursor,
    show_cursor: bool,
    cursor_blink: bool,
    typing_speed: u32,
    delete_speed: u32,
    delay_before_delete: u32,
    direction: AnimationDirection,
    font_size: f32,
    text_color: gpui::Hsla,
    cursor_color: gpui::Hsla,
}

impl TypewriterText {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            visible_chars: 0,
            style: TypewriterStyle::default(),
            cursor_style: TypewriterCursor::default(),
            show_cursor: true,
            cursor_blink: true,
            typing_speed: 50,
            delete_speed: 30,
            delay_before_delete: 2000,
            direction: AnimationDirection::default(),
            font_size: 16.0,
            text_color: rgba(0xffffffff).into(),
            cursor_color: rgba(0xffffffff).into(),
        }
    }

    pub fn visible_chars(mut self, count: usize) -> Self {
        self.visible_chars = count;
        self
    }

    pub fn style(mut self, style: TypewriterStyle) -> Self {
        self.style = style;
        self
    }

    pub fn cursor_style(mut self, style: TypewriterCursor) -> Self {
        self.cursor_style = style;
        self
    }

    pub fn show_cursor(mut self, show: bool) -> Self {
        self.show_cursor = show;
        self
    }

    pub fn cursor_blink(mut self, blink: bool) -> Self {
        self.cursor_blink = blink;
        self
    }

    pub fn typing_speed(mut self, ms: u32) -> Self {
        self.typing_speed = ms;
        self
    }

    pub fn delete_speed(mut self, ms: u32) -> Self {
        self.delete_speed = ms;
        self
    }

    pub fn delay_before_delete(mut self, ms: u32) -> Self {
        self.delay_before_delete = ms;
        self
    }

    pub fn direction(mut self, direction: AnimationDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn text_color(mut self, color: gpui::Hsla) -> Self {
        self.text_color = color;
        self
    }

    pub fn cursor_color(mut self, color: gpui::Hsla) -> Self {
        self.cursor_color = color;
        self
    }

    fn render_cursor(&self) -> Div {
        if !self.show_cursor {
            return div();
        }

        let cursor = match self.cursor_style {
            TypewriterCursor::Line => div()
                .w(px(2.0))
                .h(px(self.font_size * 1.2))
                .bg(self.cursor_color),
            TypewriterCursor::Block => div()
                .w(px(self.font_size * 0.6))
                .h(px(self.font_size * 1.2))
                .bg(self.cursor_color.opacity(0.7)),
            TypewriterCursor::Underscore => div()
                .w(px(self.font_size * 0.6))
                .h(px(2.0))
                .bg(self.cursor_color),
            TypewriterCursor::None => div(),
        };

        cursor.when(self.cursor_blink, |d| d.opacity(0.8))
    }

    /// Get the visible portion of the text
    pub fn visible_text(&self) -> String {
        self.text.chars().take(self.visible_chars).collect()
    }

    /// Check if typing is complete
    pub fn is_complete(&self) -> bool {
        self.visible_chars >= self.text.len()
    }
}

impl RenderOnce for TypewriterText {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let visible = self.visible_text();
        let font_family = match self.style {
            TypewriterStyle::Classic => "serif",
            TypewriterStyle::Modern => "sans-serif",
            TypewriterStyle::Terminal | TypewriterStyle::Glitch => "monospace",
        };
        let id = self.id.clone();

        div()
            .id(id)
            .flex()
            .items_center()
            .text_size(px(self.font_size))
            .text_color(self.text_color)
            .font_family(font_family)
            .when(self.style == TypewriterStyle::Glitch, |d| {
                d.text_decoration_1() // Glitch effect styling
            })
            .child(visible)
            .child(self.render_cursor())
    }
}
