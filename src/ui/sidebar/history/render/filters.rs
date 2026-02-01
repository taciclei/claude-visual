//! Filter panel rendering for history sidebar

use gpui::prelude::*;
use gpui::*;

use super::super::core::HistorySidebar;
use crate::storage::models::DateRangeFilter;

impl HistorySidebar {
    pub(super) fn render_filter_panel(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let current_date_filter = self.search_filter.date_range;
        let current_project_id = self.search_filter.project_id.clone();
        let projects_for_render: Vec<_> = self
            .projects
            .iter()
            .map(|p| (p.id.clone(), p.name.clone()))
            .collect();
        let filter_active = self.search_filter.is_active();

        div()
            .mt_2()
            .p_2()
            .rounded_md()
            .bg(theme.colors.background)
            .border_1()
            .border_color(theme.colors.border)
            .flex()
            .flex_col()
            .gap_2()
            .child(self.render_date_filter(current_date_filter, cx))
            .child(self.render_project_filter(current_project_id, projects_for_render, cx))
            .when(filter_active, |d| {
                d.child(self.render_clear_filters_button(cx))
            })
    }

    fn render_date_filter(
        &mut self,
        current_date_filter: DateRangeFilter,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.colors.text_muted)
                    .child("Date Range"),
            )
            .child(div().flex().flex_wrap().gap_1().children(
                DateRangeFilter::all().into_iter().map(|filter| {
                    let is_selected = filter == current_date_filter;
                    div()
                        .id(ElementId::Name(format!("date-filter-{:?}", filter).into()))
                        .px_2()
                        .py_1()
                        .rounded_sm()
                        .text_xs()
                        .bg(if is_selected {
                            theme.colors.accent
                        } else {
                            theme.colors.surface
                        })
                        .text_color(if is_selected {
                            theme.colors.background
                        } else {
                            theme.colors.text_muted
                        })
                        .hover(|style| {
                            style.bg(if is_selected {
                                theme.colors.accent_hover
                            } else {
                                theme.colors.surface_hover
                            })
                        })
                        .cursor_pointer()
                        .on_click(cx.listener(move |this, _, _window, cx| {
                            this.set_date_filter(filter, cx);
                        }))
                        .child(filter.display_name())
                }),
            ))
    }

    fn render_project_filter(
        &mut self,
        current_project_id: Option<String>,
        projects_for_render: Vec<(String, String)>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.colors.text_muted)
                    .child("Project"),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_1()
                    .child({
                        let is_selected = current_project_id.is_none();
                        div()
                            .id("project-filter-all")
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .text_xs()
                            .bg(if is_selected {
                                theme.colors.accent
                            } else {
                                theme.colors.surface
                            })
                            .text_color(if is_selected {
                                theme.colors.background
                            } else {
                                theme.colors.text_muted
                            })
                            .hover(|style| {
                                style.bg(if is_selected {
                                    theme.colors.accent_hover
                                } else {
                                    theme.colors.surface_hover
                                })
                            })
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.set_project_filter(None, cx);
                            }))
                            .child("All")
                    })
                    .children(projects_for_render.into_iter().map(|(id, name)| {
                        let is_selected = current_project_id.as_ref() == Some(&id);
                        let id_for_click = id.clone();
                        div()
                            .id(ElementId::Name(format!("project-filter-{}", id).into()))
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .text_xs()
                            .max_w(px(80.0))
                            .overflow_hidden()
                            .text_ellipsis()
                            .bg(if is_selected {
                                theme.colors.accent
                            } else {
                                theme.colors.surface
                            })
                            .text_color(if is_selected {
                                theme.colors.background
                            } else {
                                theme.colors.text_muted
                            })
                            .hover(|style| {
                                style.bg(if is_selected {
                                    theme.colors.accent_hover
                                } else {
                                    theme.colors.surface_hover
                                })
                            })
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                this.set_project_filter(Some(id_for_click.clone()), cx);
                            }))
                            .child(name)
                    })),
            )
    }

    fn render_clear_filters_button(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .id("clear-filters")
            .mt_1()
            .px_2()
            .py_1()
            .rounded_sm()
            .text_xs()
            .text_center()
            .text_color(theme.colors.warning)
            .bg(theme.colors.surface)
            .hover(|style| style.bg(theme.colors.surface_hover))
            .cursor_pointer()
            .on_click(cx.listener(|this, _, _window, cx| {
                this.clear_filters(cx);
            }))
            .child("Clear Filters")
    }
}
