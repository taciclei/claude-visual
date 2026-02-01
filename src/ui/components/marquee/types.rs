//! Shared types for marquee components

use gpui::*;

/// Marquee direction
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MarqueeDirection {
    #[default]
    Left,
    Right,
    Up,
    Down,
}

/// Marquee speed
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MarqueeSpeed {
    Slow,
    #[default]
    Normal,
    Fast,
}

/// A single ticker item
#[derive(Debug, Clone)]
pub struct TickerItem {
    pub symbol: SharedString,
    pub value: SharedString,
    pub change: Option<f32>,
}

impl TickerItem {
    pub fn new(symbol: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        Self {
            symbol: symbol.into(),
            value: value.into(),
            change: None,
        }
    }

    pub fn change(mut self, change: f32) -> Self {
        self.change = Some(change);
        self
    }
}
