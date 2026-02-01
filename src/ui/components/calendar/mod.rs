//! Calendar component for date selection

mod calendar_month;
mod date_range_picker;
mod mini_calendar;
mod types;
mod year_picker;

pub use calendar_month::CalendarMonth;
pub use date_range_picker::DateRangePicker;
pub use mini_calendar::MiniCalendar;
pub use types::{CalendarEvent, CalendarSize, SimpleDate, Weekday};
pub use year_picker::YearPicker;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_date() {
        let date = SimpleDate::new(2026, 1, 26);
        assert_eq!(date.year, 2026);
        assert_eq!(date.month, 1);
        assert_eq!(date.day, 26);
    }

    #[test]
    fn test_days_in_month() {
        assert_eq!(SimpleDate::days_in_month(2026, 1), 31);
        assert_eq!(SimpleDate::days_in_month(2026, 2), 28);
        assert_eq!(SimpleDate::days_in_month(2024, 2), 29); // Leap year
        assert_eq!(SimpleDate::days_in_month(2026, 4), 30);
    }

    #[test]
    fn test_calendar_month() {
        let cal = CalendarMonth::new(2026, 1)
            .selected(SimpleDate::new(2026, 1, 15))
            .today(SimpleDate::new(2026, 1, 26));

        assert_eq!(cal.year, 2026);
        assert_eq!(cal.month, 1);
        assert!(cal.selected.is_some());
    }

    #[test]
    fn test_year_picker() {
        let picker = YearPicker::new(2026).selected(2026);
        assert_eq!(picker.selected_year, Some(2026));
        assert_eq!(picker.start_year, 2022);
    }
}
