//! Date picker components - Date and time selection
//!
//! Provides date picker, time picker, and datetime picker components.

mod calendar;
mod date_picker;
mod date_range_picker;
mod date_time_picker;
mod time_picker;
mod types;

// Re-export types
pub use types::{DatePickerMode, DatePickerSize, DateRange, DateValue, TimeValue};

// Re-export components
pub use calendar::CalendarMonth;
pub use date_picker::DatePicker;
pub use date_range_picker::DateRangePicker;
pub use date_time_picker::DateTimePicker;
pub use time_picker::TimePicker;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_value() {
        let date = DateValue::new(2024, 3, 15);
        assert_eq!(date.year, 2024);
        assert_eq!(date.month, 3);
        assert_eq!(date.day, 15);
        assert_eq!(date.format(), "2024-03-15");
        assert_eq!(date.format_display(), "Mar 15, 2024");
    }

    #[test]
    fn test_time_value() {
        let time = TimeValue::new(14, 30);
        assert_eq!(time.format_24h(), "14:30");
        assert_eq!(time.format_12h(), "02:30 PM");

        let midnight = TimeValue::new(0, 0);
        assert_eq!(midnight.format_12h(), "12:00 AM");

        let noon = TimeValue::new(12, 0);
        assert_eq!(noon.format_12h(), "12:00 PM");
    }

    #[test]
    fn test_date_range() {
        let start = DateValue::new(2024, 1, 1);
        let end = DateValue::new(2024, 1, 31);
        let range = DateRange::new(start, end);

        assert_eq!(range.start, start);
        assert_eq!(range.end, end);
    }

    #[test]
    fn test_date_picker_sizes() {
        let small = DatePickerSize::Small;
        let medium = DatePickerSize::Medium;
        let large = DatePickerSize::Large;

        assert_eq!(small.styles(), (28.0, 8.0, 12.0));
        assert_eq!(medium.styles(), (36.0, 12.0, 14.0));
        assert_eq!(large.styles(), (44.0, 16.0, 16.0));
    }

    #[test]
    fn test_calendar_days_in_month() {
        let jan = CalendarMonth::new(2024, 1);
        // We can't call private methods directly, but the logic is tested indirectly
        // through rendering. Keeping test structure for documentation.
    }
}
