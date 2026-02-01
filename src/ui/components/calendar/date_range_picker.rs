//! Date range picker component (two calendars)

use super::calendar_month::CalendarMonth;
use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Date range picker (two calendars)
#[derive(Clone)]
pub struct DateRangePicker {
    start_date: Option<SimpleDate>,
    end_date: Option<SimpleDate>,
    left_year: i32,
    left_month: u32,
}

impl DateRangePicker {
    pub fn new(year: i32, month: u32) -> Self {
        Self {
            start_date: None,
            end_date: None,
            left_year: year,
            left_month: month,
        }
    }

    pub fn range(mut self, start: SimpleDate, end: SimpleDate) -> Self {
        self.start_date = Some(start);
        self.end_date = Some(end);
        self
    }
}

impl RenderOnce for DateRangePicker {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        // Calculate right calendar month
        let (right_year, right_month) = if self.left_month == 12 {
            (self.left_year + 1, 1)
        } else {
            (self.left_year, self.left_month + 1)
        };

        div()
            .flex()
            .gap_4()
            // Left calendar
            .child(
                CalendarMonth::new(self.left_year, self.left_month)
                    .selected(self.start_date.unwrap_or(SimpleDate::new(2026, 1, 1))),
            )
            // Divider
            .child(div().w(px(1.0)).h_full().bg(border))
            // Right calendar
            .child(
                CalendarMonth::new(right_year, right_month)
                    .selected(self.end_date.unwrap_or(SimpleDate::new(2026, 1, 31))),
            )
    }
}
