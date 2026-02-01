//! Simple data table component

use gpui::*;
use gpui::prelude::*;

/// Simple data table for stateless rendering
#[derive(Clone)]
pub struct DataTable {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    striped: bool,
}

impl DataTable {
    pub fn new(headers: Vec<impl Into<String>>) -> Self {
        Self {
            headers: headers.into_iter().map(|h| h.into()).collect(),
            rows: Vec::new(),
            striped: false,
        }
    }

    pub fn row(mut self, cells: Vec<impl Into<String>>) -> Self {
        self.rows.push(cells.into_iter().map(|c| c.into()).collect());
        self
    }

    pub fn striped(mut self) -> Self {
        self.striped = true;
        self
    }
}

impl RenderOnce for DataTable {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.6, 1.0);
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.18, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        div()
            .w_full()
            .rounded(px(8.0))
            .border_1()
            .border_color(border)
            .overflow_hidden()
            // Header
            .child(
                div()
                    .h(px(40.0))
                    .w_full()
                    .flex()
                    .items_center()
                    .bg(surface_hover)
                    .border_b_1()
                    .border_color(border)
                    .children(self.headers.iter().map(|header| {
                        div()
                            .flex_1()
                            .px_3()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(text_muted)
                            .child(header.clone())
                    }))
            )
            // Rows
            .children(self.rows.into_iter().enumerate().map(|(idx, cells)| {
                let is_even = idx % 2 == 0;
                let bg = if self.striped && !is_even {
                    surface_hover.opacity(0.5)
                } else {
                    surface.opacity(0.0)
                };

                div()
                    .h(px(40.0))
                    .w_full()
                    .flex()
                    .items_center()
                    .bg(bg)
                    .when(idx > 0, |d| d.border_t_1().border_color(border))
                    .hover(|s| s.bg(surface_hover))
                    .cursor_pointer()
                    .children(cells.into_iter().map(|cell| {
                        div()
                            .flex_1()
                            .px_3()
                            .text_sm()
                            .text_color(text)
                            .overflow_hidden()
                            .text_ellipsis()
                            .child(cell)
                    }))
            }))
    }
}
