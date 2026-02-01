//! Projects list view rendering

use gpui::*;
use gpui::prelude::*;

use crate::cloud::team::Team;
use super::super::{TeamPanel, TeamPanelEvent};

impl TeamPanel {
    /// Render projects list
    pub(super) fn render_projects(&self, team: &Team, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex()
            .flex_col()
            .px_4()
            .py_2()
            .child(
                div()
                    .text_sm()
                    .text_color(theme.colors.text_muted)
                    .child(if team.project_ids.is_empty() {
                        "No shared projects yet. Share a project to collaborate with your team."
                    } else {
                        ""
                    }),
            )
            .children(team.project_ids.iter().enumerate().map(|(idx, project_id)| {
                let project_id = project_id.clone();
                let project_id_for_click = project_id.clone();

                div()
                    .id(ElementId::Name(format!("project-{}", idx).into()))
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .hover(|s| s.bg(theme.colors.surface_hover))
                    .cursor_pointer()
                    .on_click(cx.listener(move |_this, _, _window, cx| {
                        cx.emit(TeamPanelEvent::OpenProject(project_id_for_click.clone()));
                    }))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_lg()
                                    .child("📁"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.colors.text)
                                    .child(project_id.clone()),
                            ),
                    )
            }))
    }
}
