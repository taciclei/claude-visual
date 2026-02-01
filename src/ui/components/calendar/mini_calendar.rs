//! Mini calendar component for compact displays

use gpui::*;
use gpui::prelude::*;
use super::types::*;
use super::calendar_month::CalendarMonth;

/// Mini calendar for compact displays
#[derive(Clone)]
pub struct MiniCalendar {
    year: i32,
    month: u32,
    selected: Option<SimpleDate>,
}

impl MiniCalendar {
    pub fn new(year: i32, month: u32) -> Self {
        Self {
            year,
            month,
            selected: None,
        }
    }

    pub fn selected(mut self, date: SimpleDate) -> Self {
        self.selected = Some(date);
        self
    }
}

impl RenderOnce for MiniCalendar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        CalendarMonth::new(self.year, self.month)
            .size(CalendarSize::Small)
    }
}
