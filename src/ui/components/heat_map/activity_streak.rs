use gpui::*;
use gpui::prelude::*;

/// Activity streak display
#[derive(IntoElement)]
pub struct ActivityStreak {
    id: ElementId,
    current_streak: u32,
    longest_streak: u32,
    total_days: u32,
    show_flame: bool,
    flame_color: gpui::Hsla,
    text_color: gpui::Hsla,
    background: gpui::Hsla,
}

impl ActivityStreak {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            current_streak: 0,
            longest_streak: 0,
            total_days: 0,
            show_flame: true,
            flame_color: rgb(0xf97316).into(),
            text_color: rgba(0xffffffff).into(),
            background: rgba(0x00000000).into(),
        }
    }

    pub fn current_streak(mut self, days: u32) -> Self {
        self.current_streak = days;
        self
    }

    pub fn longest_streak(mut self, days: u32) -> Self {
        self.longest_streak = days;
        self
    }

    pub fn total_days(mut self, days: u32) -> Self {
        self.total_days = days;
        self
    }

    pub fn show_flame(mut self, show: bool) -> Self {
        self.show_flame = show;
        self
    }

    pub fn flame_color(mut self, color: gpui::Hsla) -> Self {
        self.flame_color = color;
        self
    }

    pub fn text_color(mut self, color: gpui::Hsla) -> Self {
        self.text_color = color;
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = color;
        self
    }
}

impl RenderOnce for ActivityStreak {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .gap_6()
            .p_4()
            .bg(self.background)
            .child(
                // Current streak
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .when(self.show_flame && self.current_streak > 0, |d| {
                                d.child(
                                    div()
                                        .text_2xl()
                                        .child("🔥"),
                                )
                            })
                            .child(
                                div()
                                    .text_3xl()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .text_color(self.text_color)
                                    .child(format!("{}", self.current_streak)),
                            ),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(self.text_color.opacity(0.7))
                            .child("Current streak"),
                    ),
            )
            .child(
                // Longest streak
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(self.text_color)
                            .child(format!("{}", self.longest_streak)),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(self.text_color.opacity(0.7))
                            .child("Longest streak"),
                    ),
            )
            .child(
                // Total days
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(self.text_color)
                            .child(format!("{}", self.total_days)),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(self.text_color.opacity(0.7))
                            .child("Total days"),
                    ),
            )
    }
}
