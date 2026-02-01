//! Calendar month view component

use super::types::DateValue;
use gpui::prelude::*;
use gpui::*;

/// Calendar month view (for dropdown)
#[derive(IntoElement)]
pub struct CalendarMonth {
    year: u16,
    month: u8,
    selected: Option<DateValue>,
    today: Option<DateValue>,
    min_date: Option<DateValue>,
    max_date: Option<DateValue>,
    show_week_numbers: bool,
    background: Option<gpui::Hsla>,
}

impl CalendarMonth {
    pub fn new(year: u16, month: u8) -> Self {
        Self {
            year,
            month,
            selected: None,
            today: None,
            min_date: None,
            max_date: None,
            show_week_numbers: false,
            background: None,
        }
    }

    pub fn selected(mut self, date: DateValue) -> Self {
        self.selected = Some(date);
        self
    }

    pub fn today(mut self, date: DateValue) -> Self {
        self.today = Some(date);
        self
    }

    pub fn min_date(mut self, date: DateValue) -> Self {
        self.min_date = Some(date);
        self
    }

    pub fn max_date(mut self, date: DateValue) -> Self {
        self.max_date = Some(date);
        self
    }

    pub fn show_week_numbers(mut self, show: bool) -> Self {
        self.show_week_numbers = show;
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = Some(color);
        self
    }

    fn days_in_month(&self) -> u8 {
        match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.year % 4 == 0 && (self.year % 100 != 0 || self.year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }

    fn month_name(&self) -> &'static str {
        match self.month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Unknown",
        }
    }
}

impl RenderOnce for CalendarMonth {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.12, 1.0));
        let days_in_month = self.days_in_month();

        div()
            .w(px(280.0))
            .p(px(12.0))
            .rounded(px(8.0))
            .bg(bg)
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(
                // Header with month/year and navigation
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded(px(4.0))
                            .cursor_pointer()
                            .hover(|style| style.bg(hsla(0.0, 0.0, 0.2, 1.0)))
                            .child("◀"),
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                            .child(format!("{} {}", self.month_name(), self.year)),
                    )
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded(px(4.0))
                            .cursor_pointer()
                            .hover(|style| style.bg(hsla(0.0, 0.0, 0.2, 1.0)))
                            .child("▶"),
                    ),
            )
            .child(
                // Day labels
                div()
                    .flex()
                    .children(["S", "M", "T", "W", "T", "F", "S"].into_iter().map(|day| {
                        div()
                            .w(px(36.0))
                            .h(px(28.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(12.0))
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                            .child(day)
                    })),
            )
            .child(
                // Days grid
                div()
                    .flex()
                    .flex_wrap()
                    .children((1..=days_in_month).map(|day| {
                        let date = DateValue::new(self.year, self.month, day);
                        let is_selected = self.selected == Some(date);
                        let is_today = self.today == Some(date);

                        let mut day_cell = div()
                            .w(px(36.0))
                            .h(px(36.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(18.0))
                            .cursor_pointer()
                            .text_size(px(14.0));

                        if is_selected {
                            day_cell = day_cell
                                .bg(hsla(0.6, 0.7, 0.5, 1.0))
                                .text_color(hsla(0.0, 0.0, 0.0, 1.0));
                        } else if is_today {
                            day_cell = day_cell
                                .border_1()
                                .border_color(hsla(0.6, 0.7, 0.5, 1.0))
                                .text_color(hsla(0.0, 0.0, 0.9, 1.0));
                        } else {
                            day_cell = day_cell
                                .text_color(hsla(0.0, 0.0, 0.8, 1.0))
                                .hover(|style| style.bg(hsla(0.0, 0.0, 0.2, 1.0)));
                        }

                        day_cell.child(day.to_string())
                    })),
            )
    }
}
