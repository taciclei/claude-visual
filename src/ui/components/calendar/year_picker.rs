//! Year picker component

use gpui::prelude::*;
use gpui::*;

/// Year picker
#[derive(Clone)]
pub struct YearPicker {
    current_year: i32,
    selected_year: Option<i32>,
    start_year: i32,
}

impl YearPicker {
    pub fn new(current_year: i32) -> Self {
        Self {
            current_year,
            selected_year: Some(current_year),
            start_year: current_year - 4,
        }
    }

    pub fn selected(mut self, year: i32) -> Self {
        self.selected_year = Some(year);
        self
    }
}

impl RenderOnce for YearPicker {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.12, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.18, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);

        let years: Vec<i32> = (self.start_year..self.start_year + 12).collect();
        let selected = self.selected_year;

        div()
            .bg(surface)
            .rounded(px(8.0))
            .border_1()
            .border_color(border)
            .p_3()
            .flex()
            .flex_col()
            .gap_2()
            // Header
            .child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_between()
                    .pb_2()
                    .child(
                        div()
                            .size(px(28.0))
                            .rounded(px(4.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(text_muted)
                            .cursor_pointer()
                            .hover(|s| s.bg(surface_hover))
                            .child("‹"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(text)
                            .child(format!("{} - {}", self.start_year, self.start_year + 11)),
                    )
                    .child(
                        div()
                            .size(px(28.0))
                            .rounded(px(4.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(text_muted)
                            .cursor_pointer()
                            .hover(|s| s.bg(surface_hover))
                            .child("›"),
                    ),
            )
            // Years grid (3 columns x 4 rows)
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_wrap()
                    .gap_1()
                    .children(years.into_iter().map(move |year| {
                        let is_selected = selected == Some(year);
                        let is_current = year == self.current_year;

                        div()
                            .w(px(70.0))
                            .py_2()
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(4.0))
                            .text_sm()
                            .cursor_pointer()
                            .when(is_selected, |d| d.bg(accent).text_color(gpui::white()))
                            .when(!is_selected && is_current, |d| {
                                d.border_1().border_color(accent).text_color(accent)
                            })
                            .when(!is_selected && !is_current, |d| {
                                d.text_color(text).hover(|s| s.bg(surface_hover))
                            })
                            .child(format!("{}", year))
                    })),
            )
    }
}
