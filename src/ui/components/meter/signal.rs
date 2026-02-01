//! Signal strength indicator

use super::types::MeterSize;
use gpui::prelude::*;
use gpui::*;

/// Signal strength indicator
#[derive(IntoElement)]
pub struct SignalStrength {
    id: ElementId,
    bars: u8,
    max_bars: u8,
    size: MeterSize,
    connected: bool,
}

impl SignalStrength {
    pub fn new(id: impl Into<ElementId>, bars: u8) -> Self {
        Self {
            id: id.into(),
            bars: bars.min(5),
            max_bars: 4,
            size: MeterSize::default(),
            connected: true,
        }
    }

    pub fn max_bars(mut self, max: u8) -> Self {
        self.max_bars = max.min(5);
        self
    }

    pub fn size(mut self, size: MeterSize) -> Self {
        self.size = size;
        self
    }

    pub fn connected(mut self, connected: bool) -> Self {
        self.connected = connected;
        self
    }
}

impl RenderOnce for SignalStrength {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (bar_width, base_height, gap) = match self.size {
            MeterSize::Small => (3.0, 4.0, 2.0),
            MeterSize::Medium => (4.0, 6.0, 2.0),
            MeterSize::Large => (6.0, 8.0, 3.0),
        };

        let max_height = base_height * self.max_bars as f32;
        let active_color = if self.connected {
            hsla(0.0, 0.0, 0.8, 1.0)
        } else {
            hsla(0.0, 0.7, 0.5, 1.0)
        };
        let inactive_color = hsla(0.0, 0.0, 0.3, 1.0);

        div()
            .id(self.id)
            .h(px(max_height))
            .flex()
            .items_end()
            .gap(px(gap))
            .children((0..self.max_bars).map(|i| {
                let height = base_height * (i as f32 + 1.0);
                let is_active = i < self.bars && self.connected;

                div()
                    .w(px(bar_width))
                    .h(px(height))
                    .rounded(px(1.0))
                    .bg(if is_active {
                        active_color
                    } else {
                        inactive_color
                    })
            }))
    }
}
