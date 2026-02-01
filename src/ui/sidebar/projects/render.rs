//! Rendering implementation for project sidebar

use gpui::*;
use gpui::prelude::*;

use super::types::{ProjectsSidebar, ProjectsSidebarEvent};

impl ProjectsSidebar {
    fn render_quick_actions(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let accent = theme.colors.accent;
        let info = theme.colors.info;
        let success = theme.colors.success;
        let warning = theme.colors.warning;

        div()
            .flex()
            .flex_wrap()
            .gap_1()
            .px_3()
            .py_2()
            .border_b_1()
            .border_color(theme.colors.border.opacity(0.5))
            // Explore codebase
            .child(
                div()
                    .id("project-action-explore")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .bg(accent.opacity(0.15))
                    .border_1()
                    .border_color(accent.opacity(0.3))
                    .text_xs()
                    .text_color(accent)
                    .hover(move |s| s.bg(accent.opacity(0.25)).border_color(accent.opacity(0.5)))
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(ProjectsSidebarEvent::SendSkillCommand("/explore".to_string()));
                    }))
                    .child("Explore")
            )
            // APEX workflow
            .child(
                div()
                    .id("project-action-apex")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .bg(info.opacity(0.15))
                    .border_1()
                    .border_color(info.opacity(0.3))
                    .text_xs()
                    .text_color(info)
                    .hover(move |s| s.bg(info.opacity(0.25)).border_color(info.opacity(0.5)))
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(ProjectsSidebarEvent::SendSkillCommand("/apex".to_string()));
                    }))
                    .child("APEX")
            )
            // Documentation
            .child(
                div()
                    .id("project-action-docs")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .bg(success.opacity(0.15))
                    .border_1()
                    .border_color(success.opacity(0.3))
                    .text_xs()
                    .text_color(success)
                    .hover(move |s| s.bg(success.opacity(0.25)).border_color(success.opacity(0.5)))
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(ProjectsSidebarEvent::SendSkillCommand("/docs".to_string()));
                    }))
                    .child("Docs")
            )
            // Search
            .child(
                div()
                    .id("project-action-search")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .bg(warning.opacity(0.15))
                    .border_1()
                    .border_color(warning.opacity(0.3))
                    .text_xs()
                    .text_color(warning)
                    .hover(move |s| s.bg(warning.opacity(0.25)).border_color(warning.opacity(0.5)))
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(ProjectsSidebarEvent::SendSkillCommand("/search".to_string()));
                    }))
                    .child("Search")
            )
    }
}

impl Render for ProjectsSidebar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let filtered = self.filtered_projects();
        let filter_text = self.filter_text.clone();
        let is_search_focused = self.search_focus_handle.is_focused(_window);
        let is_drag_over = self.is_drag_over;

        // Copy theme colors for move closures
        let border_color = theme.colors.border;
        let surface_color = theme.colors.surface;
        let background_color = theme.colors.background;
        let accent_color = theme.colors.accent;
        let accent_hover_color = theme.colors.accent_hover;
        let text_color = theme.colors.text;
        let text_muted_color = theme.colors.text_muted;
        let surface_hover_color = theme.colors.surface_hover;

        // Pre-compute selection state for each filtered project
        let project_items: Vec<_> = filtered
            .iter()
            .enumerate()
            .map(|(idx, project)| {
                let is_selected = self
                    .selected_project
                    .map(|sel_idx| self.projects.get(sel_idx).map(|p| p.path == project.path).unwrap_or(false))
                    .unwrap_or(false);
                (idx, project.name.clone(), project.path.display().to_string(), is_selected)
            })
            .collect();

        // Extract listeners before div chain
        let on_drop_listener = cx.listener(|this, paths: &ExternalPaths, window, cx| {
            this.handle_file_drop(paths, window, cx);
        });

        let on_drag_move_listener = cx.listener(|this, _, _window, cx| {
            this.set_drag_over(true, cx);
        });

        let search_click_listener = cx.listener(|this, _, window, cx| {
            this.search_focus_handle.focus(window);
            cx.notify();
        });

        let search_key_listener = cx.listener(|this, event: &KeyDownEvent, window, cx| {
            this.handle_search_key(event, window, cx);
        });

        let add_project_listener = cx.listener(|_this, _, _window, cx| {
            cx.emit(super::types::ProjectsSidebarEvent::AddProjectRequested);
        });

        div()
            .id("projects-sidebar")
            .size_full()
            .flex()
            .flex_col()
            .bg(surface_color)
            .relative()
            // Drag and drop handlers
            .on_drop(on_drop_listener)
            .drag_over::<ExternalPaths>(|style, _, _window, _cx| {
                style.bg(hsla(210.0 / 360.0, 0.80, 0.55, 0.1))
            })
            .on_drag_move::<ExternalPaths>(on_drag_move_listener)
            // Header
            .child(
                div()
                    .flex_shrink_0()
                    .px_4()
                    .py_3()
                    .border_b_1()
                    .border_color(border_color)
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(text_muted_color)
                            .child("PROJECTS"),
                    ),
            )
            // Quick action buttons for project exploration
            .child(self.render_quick_actions(cx))
            // Search input
            .child(
                div()
                    .flex_shrink_0()
                    .px_3()
                    .py_2()
                    .child(
                        div()
                            .id("project-search")
                            .track_focus(&self.search_focus_handle)
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .bg(background_color)
                            .border_1()
                            .border_color(if is_search_focused {
                                accent_color
                            } else {
                                border_color
                            })
                            .cursor_text()
                            .text_sm()
                            .text_color(if filter_text.is_empty() {
                                text_muted_color
                            } else {
                                text_color
                            })
                            .on_click(search_click_listener)
                            .on_key_down(search_key_listener)
                            .child(if filter_text.is_empty() {
                                "Search projects...".to_string()
                            } else {
                                filter_text
                            }),
                    ),
            )
            // Project list
            .child(
                div()
                    .flex_1()
                    .id("scroll-projects")
                    .overflow_y_scroll()
                    .px_2()
                    .py_1()
                    .children(project_items.into_iter().map(|(idx, name, path, is_selected)| {
                        let bg_color = if is_selected {
                            accent_color
                        } else {
                            surface_color
                        };
                        let hover_color = if is_selected {
                            accent_hover_color
                        } else {
                            surface_hover_color
                        };

                        let project_click_listener = cx.listener(move |this, _, _window, cx| {
                            this.select_project(idx, cx);
                        });

                        div()
                            .id(ElementId::Name(format!("project-{}", idx).into()))
                            .px_3()
                            .py_2()
                            .rounded_md()
                            .bg(bg_color)
                            .hover(|style| style.bg(hover_color))
                            .cursor_pointer()
                            .on_click(project_click_listener)
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .child(name),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(text_muted_color)
                                    .child(path),
                            )
                    })),
            )
            // Add project button
            .child(
                div()
                    .flex_shrink_0()
                    .px_3()
                    .py_3()
                    .border_t_1()
                    .border_color(border_color)
                    .child(
                        div()
                            .id("add-project-button")
                            .w_full()
                            .px_3()
                            .py_2()
                            .rounded_md()
                            .bg(accent_color)
                            .hover(|style| style.bg(accent_hover_color))
                            .cursor_pointer()
                            .flex()
                            .justify_center()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .on_click(add_project_listener)
                            .child("+ Add Project"),
                    ),
            )
            // Drop zone overlay (shown when dragging files over)
            .when(is_drag_over, |d| {
                d.child(
                    div()
                        .id("drop-zone-overlay")
                        .absolute()
                        .inset_0()
                        .bg(accent_color.opacity(0.15))
                        .border_2()
                        .border_color(accent_color)
                        .rounded_lg()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .gap_2()
                                .child(
                                    div()
                                        .text_2xl()
                                        .child("📁"),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(accent_color)
                                        .child("Drop folder to add project"),
                                ),
                        ),
                )
            })
    }
}
