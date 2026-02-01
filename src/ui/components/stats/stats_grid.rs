//! StatsGrid component

use gpui::prelude::*;
use gpui::*;

use super::simple_stat::SimpleStat;

/// Stats grid for multiple stats
#[derive(Clone)]
pub struct StatsGrid {
    pub(crate) stats: Vec<SimpleStat>,
    pub(crate) columns: usize,
}

impl StatsGrid {
    pub fn new() -> Self {
        Self {
            stats: Vec::new(),
            columns: 4,
        }
    }

    pub fn stat(mut self, stat: SimpleStat) -> Self {
        self.stats.push(stat);
        self
    }

    pub fn columns(mut self, cols: usize) -> Self {
        self.columns = cols.max(1);
        self
    }
}

impl Default for StatsGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for StatsGrid {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        div()
            .w_full()
            .rounded(px(8.0))
            .border_1()
            .border_color(border)
            .bg(surface)
            .flex()
            .flex_wrap()
            .children(self.stats.into_iter().enumerate().map(|(idx, stat)| {
                let is_first_in_row = idx % self.columns == 0;
                let is_first_row = idx < self.columns;

                div()
                    .flex_1()
                    .min_w(px(120.0))
                    .p_4()
                    .when(!is_first_in_row, |d| d.border_l_1().border_color(border))
                    .when(!is_first_row, |d| d.border_t_1().border_color(border))
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .when_some(stat.icon, |d, icon| d.child(div().text_base().child(icon)))
                            .child(div().text_xs().text_color(text_muted).child(stat.label)),
                    )
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(text)
                            .child(stat.value),
                    )
            }))
    }
}
