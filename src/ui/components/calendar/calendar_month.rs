//! Calendar month view component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Calendar month view
#[derive(Clone, IntoElement)]
pub struct CalendarMonth {
    /// Current displayed year
    pub(crate) year: i32,
    /// Current displayed month (1-12)
    pub(crate) month: u32,
    /// Selected date
    pub(crate) selected: Option<SimpleDate>,
    /// Today's date
    pub(crate) today: Option<SimpleDate>,
    /// Size
    pub(crate) size: CalendarSize,
    /// Show week numbers
    pub(crate) show_week_numbers: bool,
    /// Disabled dates
    pub(crate) disabled_dates: Vec<SimpleDate>,
    /// Min selectable date
    pub(crate) min_date: Option<SimpleDate>,
    /// Max selectable date
    pub(crate) max_date: Option<SimpleDate>,
}

impl CalendarMonth {
    pub fn new(year: i32, month: u32) -> Self {
        Self {
            year,
            month: month.clamp(1, 12),
            selected: None,
            today: None,
            size: CalendarSize::default(),
            show_week_numbers: false,
            disabled_dates: Vec::new(),
            min_date: None,
            max_date: None,
        }
    }

    pub fn selected(mut self, date: SimpleDate) -> Self {
        self.selected = Some(date);
        self
    }

    pub fn today(mut self, date: SimpleDate) -> Self {
        self.today = Some(date);
        self
    }

    pub fn size(mut self, size: CalendarSize) -> Self {
        self.size = size;
        self
    }

    pub fn show_week_numbers(mut self, show: bool) -> Self {
        self.show_week_numbers = show;
        self
    }

    pub fn min_date(mut self, date: SimpleDate) -> Self {
        self.min_date = Some(date);
        self
    }

    pub fn max_date(mut self, date: SimpleDate) -> Self {
        self.max_date = Some(date);
        self
    }

    fn is_date_disabled(&self, date: &SimpleDate) -> bool {
        if self.disabled_dates.contains(date) {
            return true;
        }

        if let Some(min) = &self.min_date {
            if date.year < min.year
                || (date.year == min.year && date.month < min.month)
                || (date.year == min.year && date.month == min.month && date.day < min.day)
            {
                return true;
            }
        }

        if let Some(max) = &self.max_date {
            if date.year > max.year
                || (date.year == max.year && date.month > max.month)
                || (date.year == max.year && date.month == max.month && date.day > max.day)
            {
                return true;
            }
        }

        false
    }
}

impl RenderOnce for CalendarMonth {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.12, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.18, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);

        let cell_size = self.size.cell_size();
        let font_size = self.size.font_size();

        let days_in_month = SimpleDate::days_in_month(self.year, self.month);
        let first_weekday = SimpleDate::first_weekday(self.year, self.month);

        let date = SimpleDate::new(self.year, self.month, 1);

        // Build calendar grid
        let mut days: Vec<Option<u32>> = Vec::new();

        // Empty cells before first day
        for _ in 0..first_weekday {
            days.push(None);
        }

        // Days of month
        for day in 1..=days_in_month {
            days.push(Some(day));
        }

        // Pad to complete last week
        while days.len() % 7 != 0 {
            days.push(None);
        }

        let selected = self.selected;
        let today = self.today;
        let year = self.year;
        let month = self.month;

        div()
            .bg(surface)
            .rounded(px(8.0))
            .border_1()
            .border_color(border)
            .p_3()
            .flex()
            .flex_col()
            .gap_2()
            // Header with month/year and navigation
            .child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_between()
                    .pb_2()
                    // Previous month
                    .child(
                        div()
                            .size(px(28.0))
                            .rounded(px(4.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(text_muted)
                            .cursor_pointer()
                            .hover(|s| s.bg(surface_hover).text_color(text))
                            .child("‹"),
                    )
                    // Month/Year
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(text)
                            .child(format!("{} {}", date.month_name(), self.year)),
                    )
                    // Next month
                    .child(
                        div()
                            .size(px(28.0))
                            .rounded(px(4.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(text_muted)
                            .cursor_pointer()
                            .hover(|s| s.bg(surface_hover).text_color(text))
                            .child("›"),
                    ),
            )
            // Weekday headers
            .child(
                div()
                    .w_full()
                    .flex()
                    .children(Weekday::all().into_iter().map(|day| {
                        let name = day.short_name().to_string();
                        div()
                            .size(px(cell_size))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_xs()
                            .text_color(text_muted)
                            .child(name)
                    })),
            )
            // Days grid
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_wrap()
                    .children(days.into_iter().map(move |day_opt| {
                        let mut cell = div()
                            .size(px(cell_size))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(4.0));

                        if let Some(day) = day_opt {
                            let current_date = SimpleDate::new(year, month, day);
                            let is_selected = selected == Some(current_date);
                            let is_today = today == Some(current_date);

                            cell = cell.text_color(text).cursor_pointer();

                            if is_selected {
                                cell = cell.bg(accent).text_color(gpui::white());
                            } else if is_today {
                                cell = cell.border_1().border_color(accent).text_color(accent);
                            } else {
                                cell = cell.hover(|s| s.bg(surface_hover));
                            }

                            cell.child(format!("{}", day))
                        } else {
                            cell
                        }
                    })),
            )
    }
}
