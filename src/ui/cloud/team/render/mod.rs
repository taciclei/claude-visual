//! Main render implementation and dialogs for Team panel

use gpui::prelude::*;
use gpui::*;

use super::{TeamPanel, TeamViewMode};

mod create_dialog;
mod invite_dialog;

impl Render for TeamPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.colors.background)
            .relative()
            .child(match self.view_mode {
                TeamViewMode::TeamList | TeamViewMode::Invitations => {
                    self.render_team_list(cx).into_any_element()
                }
                _ => {
                    if let Some(team) = self.selected_team().cloned() {
                        self.render_team_details(&team, cx).into_any_element()
                    } else {
                        self.render_team_list(cx).into_any_element()
                    }
                }
            })
            .when(self.create_dialog_open, |d| {
                d.child(self.render_create_dialog(cx))
            })
            .when(self.invite_dialog_open, |d| {
                d.child(self.render_invite_dialog(cx))
            })
    }
}
