//! Sidebar rendering with tab bar and content

use super::super::core::Workspace;
use super::super::types::SidebarTab;
use crate::app::theme::Theme;
use gpui::prelude::*;
use gpui::*;

pub fn render_sidebar(
    workspace: &Workspace,
    theme: &Theme,
    cx: &mut Context<Workspace>,
) -> impl IntoElement {
    div()
        .id("sidebar")
        .track_focus(&workspace.sidebar_focus)
        .flex_shrink_0()
        .w(px(280.0))
        .h_full()
        .border_r_1()
        .border_color(theme.colors.border)
        .flex()
        .flex_col()
        .when(!workspace.show_sidebar, |this| this.hidden())
        .child(render_tab_bar(workspace, theme, cx))
        .child(render_tab_content(workspace))
}

fn render_tab_bar(
    workspace: &Workspace,
    theme: &Theme,
    cx: &mut Context<Workspace>,
) -> impl IntoElement {
    let tabs = [
        ("tab-projects", "Projects", SidebarTab::Projects),
        ("tab-files", "Files", SidebarTab::Files),
        ("tab-history", "History", SidebarTab::History),
        ("tab-git", "Git", SidebarTab::Git),
        ("tab-team", "Team", SidebarTab::Team),
    ];

    let mut bar = div()
        .id("navigation")
        .track_focus(&workspace.navigation_focus)
        .flex_shrink_0()
        // Add top padding for macOS traffic lights (window buttons)
        .pt(px(8.0))
        .h(px(44.0))
        .border_b_1()
        .border_color(theme.colors.border)
        .flex()
        .items_center()
        // Left padding for traffic lights
        .pl(px(76.0));

    for (id, label, tab) in tabs {
        let is_active = workspace.sidebar_tab == tab;
        let on_click = cx.listener(move |this, _, _window, cx| {
            this.switch_sidebar_tab(tab, cx);
        });

        let surface = theme.colors.surface;
        let surface_hover = theme.colors.surface_hover;
        let text = theme.colors.text;
        let text_muted = theme.colors.text_muted;
        let accent = theme.colors.accent;

        bar = bar.child(
            div()
                .id(id)
                .flex_1()
                .h_full()
                .flex()
                .items_center()
                .justify_center()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .cursor_pointer()
                .when(is_active, move |this| {
                    this.bg(surface)
                        .text_color(text)
                        .border_b_2()
                        .border_color(accent)
                })
                .when(!is_active, move |this| {
                    this.text_color(text_muted)
                        .hover(move |style| style.bg(surface_hover))
                })
                .on_click(on_click)
                .child(label),
        );
    }

    bar
}

fn render_tab_content(workspace: &Workspace) -> Div {
    div()
        .flex_1()
        .overflow_hidden()
        .when(workspace.sidebar_tab == SidebarTab::Projects, |this| {
            this.child(workspace.projects_sidebar.clone())
        })
        .when(workspace.sidebar_tab == SidebarTab::Files, |this| {
            this.child(workspace.file_tree.clone())
        })
        .when(workspace.sidebar_tab == SidebarTab::History, |this| {
            this.child(workspace.history_sidebar.clone())
        })
        .when(workspace.sidebar_tab == SidebarTab::Git, |this| {
            this.child(workspace.worktree_panel.clone())
        })
        .when(workspace.sidebar_tab == SidebarTab::Team, |this| {
            this.child(workspace.team_panel.clone())
        })
}
