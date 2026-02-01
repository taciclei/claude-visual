//! Countdown type definitions

use gpui::*;
use gpui::prelude::*;

/// Countdown size variants
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CountdownSize {
    Small,
    #[default]
    Medium,
    Large,
    XLarge,
}

/// Countdown variant styles
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CountdownVariant {
    #[default]
    Default,
    Boxed,
    Circular,
    Minimal,
}

/// Time value for countdowns
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TimeRemaining {
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

impl TimeRemaining {
    pub fn new(days: u32, hours: u32, minutes: u32, seconds: u32) -> Self {
        Self { days, hours, minutes, seconds }
    }

    pub fn from_seconds(total_seconds: u64) -> Self {
        let days = (total_seconds / 86400) as u32;
        let hours = ((total_seconds % 86400) / 3600) as u32;
        let minutes = ((total_seconds % 3600) / 60) as u32;
        let seconds = (total_seconds % 60) as u32;
        Self { days, hours, minutes, seconds }
    }

    pub fn total_seconds(&self) -> u64 {
        self.days as u64 * 86400
            + self.hours as u64 * 3600
            + self.minutes as u64 * 60
            + self.seconds as u64
    }

    pub fn is_zero(&self) -> bool {
        self.days == 0 && self.hours == 0 && self.minutes == 0 && self.seconds == 0
    }
}
