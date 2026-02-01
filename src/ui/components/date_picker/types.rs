//! Date picker types and value representations

use gpui::SharedString;

/// Date picker mode
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DatePickerMode {
    #[default]
    Single,
    Range,
    Multiple,
}

/// Date picker size variants
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DatePickerSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl DatePickerSize {
    /// Get size styles (height, padding_x, font_size)
    pub fn styles(&self) -> (f32, f32, f32) {
        match self {
            DatePickerSize::Small => (28.0, 8.0, 12.0),
            DatePickerSize::Medium => (36.0, 12.0, 14.0),
            DatePickerSize::Large => (44.0, 16.0, 16.0),
        }
    }
}

/// Date value representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateValue {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl DateValue {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    pub fn format(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    pub fn format_display(&self) -> String {
        let month_name = match self.month {
            1 => "Jan", 2 => "Feb", 3 => "Mar", 4 => "Apr",
            5 => "May", 6 => "Jun", 7 => "Jul", 8 => "Aug",
            9 => "Sep", 10 => "Oct", 11 => "Nov", 12 => "Dec",
            _ => "???",
        };
        format!("{} {}, {}", month_name, self.day, self.year)
    }
}

/// Date range
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateRange {
    pub start: DateValue,
    pub end: DateValue,
}

impl DateRange {
    pub fn new(start: DateValue, end: DateValue) -> Self {
        Self { start, end }
    }
}

/// Time value representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeValue {
    pub hour: u8,
    pub minute: u8,
    pub second: Option<u8>,
}

impl TimeValue {
    pub fn new(hour: u8, minute: u8) -> Self {
        Self { hour, minute, second: None }
    }

    pub fn with_seconds(hour: u8, minute: u8, second: u8) -> Self {
        Self { hour, minute, second: Some(second) }
    }

    pub fn format_24h(&self) -> String {
        if let Some(sec) = self.second {
            format!("{:02}:{:02}:{:02}", self.hour, self.minute, sec)
        } else {
            format!("{:02}:{:02}", self.hour, self.minute)
        }
    }

    pub fn format_12h(&self) -> String {
        let (hour, period) = if self.hour == 0 {
            (12, "AM")
        } else if self.hour < 12 {
            (self.hour, "AM")
        } else if self.hour == 12 {
            (12, "PM")
        } else {
            (self.hour - 12, "PM")
        };
        format!("{:02}:{:02} {}", hour, self.minute, period)
    }
}
