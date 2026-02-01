use super::types::HeatMapScale;
use gpui::prelude::*;
use gpui::*;

/// Contribution calendar (GitHub style)
#[derive(IntoElement)]
pub struct ContributionCalendar {
    id: ElementId,
    contributions: Vec<(String, u32)>, // (date, count)
    scale: HeatMapScale,
    weeks_to_show: usize,
    show_month_labels: bool,
    show_day_labels: bool,
    show_total: bool,
    cell_size: f32,
    background: gpui::Hsla,
}

impl ContributionCalendar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            contributions: Vec::new(),
            scale: HeatMapScale::Green,
            weeks_to_show: 52,
            show_month_labels: true,
            show_day_labels: true,
            show_total: true,
            cell_size: 10.0,
            background: rgba(0x00000000).into(),
        }
    }

    pub fn contributions(mut self, data: Vec<(String, u32)>) -> Self {
        self.contributions = data;
        self
    }

    pub fn scale(mut self, scale: HeatMapScale) -> Self {
        self.scale = scale;
        self
    }

    pub fn weeks_to_show(mut self, weeks: usize) -> Self {
        self.weeks_to_show = weeks;
        self
    }

    pub fn show_month_labels(mut self, show: bool) -> Self {
        self.show_month_labels = show;
        self
    }

    pub fn show_day_labels(mut self, show: bool) -> Self {
        self.show_day_labels = show;
        self
    }

    pub fn show_total(mut self, show: bool) -> Self {
        self.show_total = show;
        self
    }

    pub fn cell_size(mut self, size: f32) -> Self {
        self.cell_size = size;
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = color;
        self
    }

    fn total_contributions(&self) -> u32 {
        self.contributions.iter().map(|(_, c)| c).sum()
    }
}

impl RenderOnce for ContributionCalendar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let total = self.total_contributions();
        let day_labels = ["Mon", "Wed", "Fri"];

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap_2()
            .bg(self.background)
            .when(self.show_total, |d| {
                d.child(
                    div()
                        .text_sm()
                        .text_color(rgba(0xccccccff))
                        .child(format!("{} contributions in the last year", total)),
                )
            })
            .child(
                div()
                    .flex()
                    .gap_1()
                    .when(self.show_day_labels, |d| {
                        d.child(div().flex().flex_col().gap_1().w(px(24.0)).children(
                            day_labels.iter().map(|label| {
                                div()
                                    .h(px(self.cell_size * 2.0 + 2.0))
                                    .text_xs()
                                    .text_color(rgba(0x888888ff))
                                    .flex()
                                    .items_center()
                                    .child(*label)
                            }),
                        ))
                    })
                    .child(
                        // Weeks grid placeholder
                        div()
                            .flex()
                            .gap_1()
                            .children((0..self.weeks_to_show.min(52)).map(|_week| {
                                div().flex().flex_col().gap_1().children((0..7).map(|_day| {
                                    div()
                                        .size(px(self.cell_size))
                                        .rounded(px(2.0))
                                        .bg(self.scale.color_for_level(0))
                                }))
                            })),
                    ),
            )
    }
}
