//! Main container rendering for side-by-side diff view

use gpui::*;
use gpui::prelude::*;

use crate::ui::diff::side_by_side::core::SideBySideDiffView;
use crate::ui::diff::hunk::{HunkStatus, ManagedHunk};

impl Render for SideBySideDiffView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let file_path = self.file_path.clone();
        let collapsed = self.collapsed;
        let total_additions = self.hunk_manager.total_additions();
        let total_deletions = self.hunk_manager.total_deletions();
        let applied_count = self.hunk_manager.count_by_status(HunkStatus::Applied);
        let rejected_count = self.hunk_manager.count_by_status(HunkStatus::Rejected);
        let pending_count = self.hunk_manager.count_by_status(HunkStatus::Pending);
        let total_hunks = self.hunk_manager.hunks.len();
        let comment_count = self.comments.total_comments();
        let display_mode = self.display_mode;

        // Collect hunk data for rendering
        let hunks: Vec<ManagedHunk> = self.hunk_manager.hunks.clone();

        // Copy theme colors for closures
        let border_color = theme.colors.border;
        let background_color = theme.colors.background;

        div()
            .w_full()
            .rounded_lg()
            .overflow_hidden()
            .border_1()
            .border_color(border_color)
            .bg(background_color)
            // Header
            .child(
                self.render_header(
                    &theme,
                    &file_path,
                    collapsed,
                    total_additions,
                    total_deletions,
                    applied_count,
                    rejected_count,
                    total_hunks,
                    comment_count,
                    display_mode,
                    cx,
                )
            )
            // Toolbar
            .when(!collapsed, |d| {
                d.child(
                    self.render_toolbar(
                        &theme,
                        display_mode,
                        pending_count,
                        cx,
                    )
                )
            })
            // Hunks content
            .when(!collapsed, |d| {
                d.child(
                    div()
                        .w_full()
                        .max_h(px(600.0))
                        .id("scroll-diff-hunks")
                        .overflow_y_scroll()
                        .overflow_x_scroll()
                        .p_2()
                        .children(hunks.iter().map(|hunk| {
                            self.render_hunk_side_by_side(hunk, &theme, cx)
                        })),
                )
            })
    }
}
