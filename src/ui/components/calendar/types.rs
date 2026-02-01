//! Shared types for calendar components

use gpui::prelude::*;

/// Day of the week
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl Weekday {
    pub(crate) fn short_name(&self) -> &str {
        match self {
            Weekday::Sunday => "Su",
            Weekday::Monday => "Mo",
            Weekday::Tuesday => "Tu",
            Weekday::Wednesday => "We",
            Weekday::Thursday => "Th",
            Weekday::Friday => "Fr",
            Weekday::Saturday => "Sa",
        }
    }

    pub(crate) fn all() -> [Weekday; 7] {
        [
            Weekday::Sunday,
            Weekday::Monday,
            Weekday::Tuesday,
            Weekday::Wednesday,
            Weekday::Thursday,
            Weekday::Friday,
            Weekday::Saturday,
        ]
    }
}

/// Calendar size
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CalendarSize {
    /// Small (compact)
    Small,
    /// Medium (default)
    #[default]
    Medium,
    /// Large
    Large,
}

impl CalendarSize {
    pub(crate) fn cell_size(&self) -> f32 {
        match self {
            CalendarSize::Small => 28.0,
            CalendarSize::Medium => 36.0,
            CalendarSize::Large => 44.0,
        }
    }

    pub(crate) fn font_size(&self) -> f32 {
        match self {
            CalendarSize::Small => 11.0,
            CalendarSize::Medium => 13.0,
            CalendarSize::Large => 15.0,
        }
    }
}

/// Simple date structure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimpleDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl SimpleDate {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    /// Get days in month (simplified, doesn't handle leap years perfectly)
    pub fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }

    /// Get day of week for first of month (Zeller's congruence simplified)
    pub fn first_weekday(year: i32, month: u32) -> u32 {
        // Simplified calculation
        let y = if month < 3 { year - 1 } else { year };
        let m = if month < 3 { month as i32 + 12 } else { month as i32 };

        let q = 1i32; // First day
        let k = y % 100;
        let j = y / 100;

        let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;

        // Convert to 0 = Sunday
        ((h + 6) % 7) as u32
    }

    pub(crate) fn month_name(&self) -> &str {
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

/// Events emitted by Calendar
#[derive(Debug, Clone)]
pub enum CalendarEvent {
    /// Date selected
    DateSelected(SimpleDate),
    /// Month changed
    MonthChanged { year: i32, month: u32 },
}
