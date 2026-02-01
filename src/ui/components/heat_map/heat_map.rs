use gpui::*;
use gpui::prelude::*;
use super::types::{HeatMapScale, HeatMapCell};

/// GitHub-style contribution heat map
#[derive(IntoElement)]
pub struct HeatMap {
    id: ElementId,
    data: Vec<Vec<HeatMapCell>>,
    scale: HeatMapScale,
    custom_colors: Option<Vec<gpui::Hsla>>,
    cell_size: f32,
    cell_gap: f32,
    show_legend: bool,
    show_labels: bool,
    row_labels: Vec<SharedString>,
    col_labels: Vec<SharedString>,
    max_value: Option<f32>,
    levels: usize,
    background: gpui::Hsla,
}

impl HeatMap {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            data: Vec::new(),
            scale: HeatMapScale::default(),
            custom_colors: None,
            cell_size: 12.0,
            cell_gap: 3.0,
            show_legend: true,
            show_labels: false,
            row_labels: Vec::new(),
            col_labels: Vec::new(),
            max_value: None,
            levels: 5,
            background: rgba(0x00000000).into(),
        }
    }

    pub fn data(mut self, data: Vec<Vec<HeatMapCell>>) -> Self {
        self.data = data;
        self
    }

    pub fn scale(mut self, scale: HeatMapScale) -> Self {
        self.scale = scale;
        self
    }

    pub fn custom_colors(mut self, colors: Vec<gpui::Hsla>) -> Self {
        self.custom_colors = Some(colors);
        self.scale = HeatMapScale::Custom;
        self
    }

    pub fn cell_size(mut self, size: f32) -> Self {
        self.cell_size = size;
        self
    }

    pub fn cell_gap(mut self, gap: f32) -> Self {
        self.cell_gap = gap;
        self
    }

    pub fn show_legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    pub fn row_labels(mut self, labels: Vec<impl Into<SharedString>>) -> Self {
        self.row_labels = labels.into_iter().map(|l| l.into()).collect();
        self
    }

    pub fn col_labels(mut self, labels: Vec<impl Into<SharedString>>) -> Self {
        self.col_labels = labels.into_iter().map(|l| l.into()).collect();
        self
    }

    pub fn max_value(mut self, max: f32) -> Self {
        self.max_value = Some(max);
        self
    }

    pub fn levels(mut self, levels: usize) -> Self {
        self.levels = levels.max(2);
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = color;
        self
    }

    fn get_max(&self) -> f32 {
        self.max_value.unwrap_or_else(|| {
            self.data
                .iter()
                .flat_map(|row| row.iter())
                .map(|cell| cell.value)
                .fold(0.0_f32, |a, b| a.max(b))
        })
    }

    fn value_to_level(&self, value: f32, max: f32) -> usize {
        if max == 0.0 || value == 0.0 {
            return 0;
        }
        let normalized = value / max;
        let level = (normalized * (self.levels - 1) as f32).ceil() as usize;
        level.min(self.levels - 1)
    }

    fn get_color(&self, level: usize) -> gpui::Hsla {
        if let Some(ref colors) = self.custom_colors {
            return colors.get(level.min(colors.len() - 1)).copied().unwrap_or(colors[0]);
        }
        self.scale.color_for_level(level)
    }
}

impl RenderOnce for HeatMap {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let max = self.get_max();
        let cell_total = self.cell_size + self.cell_gap;
        let id = self.id.clone();

        div()
            .id(id)
            .flex()
            .flex_col()
            .gap_2()
            .bg(self.background)
            .child(
                // Grid container
                div()
                    .flex()
                    .flex_col()
                    .gap(px(self.cell_gap))
                    .children(self.data.iter().enumerate().map(|(row_idx, row)| {
                        div()
                            .flex()
                            .gap(px(self.cell_gap))
                            .when(self.show_labels && row_idx < self.row_labels.len(), |d| {
                                d.child(
                                    div()
                                        .w(px(30.0))
                                        .text_xs()
                                        .text_color(rgba(0x888888ff))
                                        .child(self.row_labels[row_idx].clone()),
                                )
                            })
                            .children(row.iter().map(|cell| {
                                let level = self.value_to_level(cell.value, max);
                                let color = self.get_color(level);

                                div()
                                    .size(px(self.cell_size))
                                    .rounded(px(2.0))
                                    .bg(color)
                                    .cursor_pointer()
                            }))
                    })),
            )
            .when(self.show_legend, |d| {
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .mt_2()
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgba(0x888888ff))
                                .child("Less"),
                        )
                        .children((0..self.levels).map(|level| {
                            div()
                                .size(px(self.cell_size))
                                .rounded(px(2.0))
                                .bg(self.get_color(level))
                        }))
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgba(0x888888ff))
                                .child("More"),
                        ),
                )
            })
    }
}
