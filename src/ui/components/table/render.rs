//! Table render implementation

use gpui::*;
use gpui::prelude::*;

use super::table_impl::Table;
use super::types::*;

impl Render for Table {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let row_height = self.size.row_height();
        let cell_padding = self.size.cell_padding();

        let is_striped = matches!(self.style, TableStyle::Striped);
        let is_bordered = matches!(self.style, TableStyle::Bordered);
        let show_borders = !matches!(self.style, TableStyle::Minimal);

        let border_color = theme.colors.border;
        let surface_color = theme.colors.surface;
        let surface_hover = theme.colors.surface_hover;
        let text_muted = theme.colors.text_muted;
        let accent_color = theme.colors.accent;
        let text_color = theme.colors.text;

        div()
            .id("table")
            .w_full()
            .rounded(px(8.0))
            .when(show_borders, |d| {
                d.border_1()
                    .border_color(border_color)
            })
            .overflow_hidden()
            // Header
            .when(self.show_header, |d| {
                let border_color = border_color;
                let surface_hover = surface_hover;
                let text_muted = text_muted;
                let accent_color = accent_color;

                d.child(
                    div()
                        .id("table-header")
                        .h(px(row_height))
                        .w_full()
                        .flex()
                        .items_center()
                        .bg(surface_hover)
                        .when(show_borders, |d| {
                            d.border_b_1().border_color(border_color)
                        })
                        .children(self.columns.iter().map(|col| {
                            let sort_icon = match &self.sort {
                                Some(sort) if sort.column == col.key => {
                                    match sort.direction {
                                        SortDirection::Ascending => Some("↑"),
                                        SortDirection::Descending => Some("↓"),
                                    }
                                }
                                _ => None,
                            };

                            let text_align = match col.align {
                                ColumnAlign::Left => TextAlign::Left,
                                ColumnAlign::Center => TextAlign::Center,
                                ColumnAlign::Right => TextAlign::Right,
                            };

                            div()
                                .px(px(cell_padding))
                                .h_full()
                                .flex()
                                .items_center()
                                .when_some(col.width, |d, w| d.w(px(w)))
                                .when(col.width.is_none(), |d| d.flex_1())
                                .when_some(col.min_width, |d, w| d.min_w(px(w)))
                                .when_some(col.max_width, |d, w| d.max_w(px(w)))
                                .when(is_bordered, |d| {
                                    d.border_r_1().border_color(border_color)
                                })
                                .when(col.sortable, |d| {
                                    d.cursor_pointer()
                                        .hover(|s| s.bg(surface_color))
                                })
                                .child(
                                    div()
                                        .flex_1()
                                        .text_xs()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(text_muted)
                                        .text_align(text_align)
                                        .child(col.header.clone())
                                )
                                .when_some(sort_icon, |d, icon| {
                                    d.child(
                                        div()
                                            .ml_1()
                                            .text_xs()
                                            .text_color(accent_color)
                                            .child(icon)
                                    )
                                })
                        }))
                )
            })
            // Body
            .child(
                div()
                    .id("table-body")
                    .w_full()
                    .flex()
                    .flex_col()
                    .children(self.rows.iter().enumerate().map(|(row_idx, row)| {
                        let is_selected = self.selected.contains(&row_idx);
                        let is_hovered = self.hovered_row == Some(row_idx);
                        let is_even = row_idx % 2 == 0;
                        let opacity = if row.disabled { 0.5 } else { 1.0 };

                        let bg_color = if is_selected {
                            accent_color.opacity(0.15)
                        } else if is_striped && !is_even {
                            surface_hover.opacity(0.5)
                        } else {
                            surface_color.opacity(0.0)
                        };

                        let on_row_click = cx.listener(move |this, _, _window, cx| {
                            this.handle_row_click(row_idx, cx);
                        });

                        div()
                            .id(SharedString::from(format!("table-row-{}", row_idx)))
                            .h(px(row_height))
                            .w_full()
                            .flex()
                            .items_center()
                            .bg(bg_color)
                            .opacity(opacity)
                            .when(show_borders && row_idx > 0, |d| {
                                d.border_t_1().border_color(border_color)
                            })
                            .when(self.hoverable && !row.disabled, |d| {
                                d.hover(|s| s.bg(surface_hover))
                            })
                            .when(!row.disabled, |d| {
                                d.cursor_pointer()
                                    .on_click(on_row_click)
                            })
                            .children(row.cells.iter().enumerate().map(|(col_idx, cell)| {
                                let col = self.columns.get(col_idx);
                                let text_align = col.map(|c| match c.align {
                                    ColumnAlign::Left => TextAlign::Left,
                                    ColumnAlign::Center => TextAlign::Center,
                                    ColumnAlign::Right => TextAlign::Right,
                                }).unwrap_or(TextAlign::Left);

                                div()
                                    .px(px(cell_padding))
                                    .h_full()
                                    .flex()
                                    .items_center()
                                    .when_some(col.and_then(|c| c.width), |d, w| d.w(px(w)))
                                    .when(col.map(|c| c.width.is_none()).unwrap_or(true), |d| d.flex_1())
                                    .when(is_bordered, |d| {
                                        d.border_r_1().border_color(border_color)
                                    })
                                    .child(
                                        div()
                                            .flex_1()
                                            .text_sm()
                                            .text_color(text_color)
                                            .text_align(text_align)
                                            .overflow_hidden()
                                            .text_ellipsis()
                                            .child(cell.clone())
                                    )
                            }))
                    }))
            )
    }
}
