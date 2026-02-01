use super::types::HeatMapScale;
use gpui::prelude::*;
use gpui::*;

/// Data grid heat map (for tables)
#[derive(IntoElement)]
pub struct DataGridHeatMap {
    id: ElementId,
    rows: Vec<SharedString>,
    cols: Vec<SharedString>,
    values: Vec<Vec<f32>>,
    scale: HeatMapScale,
    show_values: bool,
    cell_width: f32,
    cell_height: f32,
    background: gpui::Hsla,
}

impl DataGridHeatMap {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            rows: Vec::new(),
            cols: Vec::new(),
            values: Vec::new(),
            scale: HeatMapScale::Blue,
            show_values: true,
            cell_width: 60.0,
            cell_height: 40.0,
            background: rgba(0x00000000).into(),
        }
    }

    pub fn rows(mut self, rows: Vec<impl Into<SharedString>>) -> Self {
        self.rows = rows.into_iter().map(|r| r.into()).collect();
        self
    }

    pub fn cols(mut self, cols: Vec<impl Into<SharedString>>) -> Self {
        self.cols = cols.into_iter().map(|c| c.into()).collect();
        self
    }

    pub fn values(mut self, values: Vec<Vec<f32>>) -> Self {
        self.values = values;
        self
    }

    pub fn scale(mut self, scale: HeatMapScale) -> Self {
        self.scale = scale;
        self
    }

    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    pub fn cell_size(mut self, width: f32, height: f32) -> Self {
        self.cell_width = width;
        self.cell_height = height;
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = color;
        self
    }

    pub(crate) fn get_max(&self) -> f32 {
        self.values
            .iter()
            .flat_map(|row| row.iter())
            .fold(0.0_f32, |a, &b| a.max(b))
    }
}

impl RenderOnce for DataGridHeatMap {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let max = self.get_max();

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .bg(self.background)
            .child(
                // Header row
                div()
                    .flex()
                    .child(div().w(px(80.0))) // Empty corner cell
                    .children(self.cols.iter().map(|col| {
                        div()
                            .w(px(self.cell_width))
                            .h(px(30.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_xs()
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .text_color(rgba(0xccccccff))
                            .child(col.clone())
                    })),
            )
            .children(self.values.iter().enumerate().map(|(row_idx, row)| {
                div()
                    .flex()
                    .child(
                        // Row label
                        div()
                            .w(px(80.0))
                            .h(px(self.cell_height))
                            .flex()
                            .items_center()
                            .text_xs()
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .text_color(rgba(0xccccccff))
                            .when(row_idx < self.rows.len(), |d| {
                                d.child(self.rows[row_idx].clone())
                            }),
                    )
                    .children(row.iter().map(|&value| {
                        let level = if max > 0.0 {
                            ((value / max) * 4.0).ceil() as usize
                        } else {
                            0
                        };
                        let color = self.scale.color_for_level(level.min(4));

                        div()
                            .w(px(self.cell_width))
                            .h(px(self.cell_height))
                            .flex()
                            .items_center()
                            .justify_center()
                            .bg(color)
                            .border_1()
                            .border_color(rgba(0xffffff1a))
                            .when(self.show_values, |d| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .text_color(rgba(0x333333ff))
                                        .child(format!("{:.1}", value)),
                                )
                            })
                    }))
            }))
    }
}
