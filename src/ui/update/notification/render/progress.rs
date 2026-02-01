//! Progress rendering for update installation

use gpui::prelude::*;
use gpui::*;

use super::super::core::UpdateNotification;
use super::super::types::SimpleColors;
use crate::update::InstallProgress;

impl UpdateNotification {
    /// Render the download progress
    pub(crate) fn render_progress(
        &self,
        progress: &InstallProgress,
        theme: &SimpleColors,
    ) -> impl IntoElement {
        let (message, progress_value): (String, Option<f32>) = match progress {
            InstallProgress::Starting => ("Starting download...".to_string(), None),
            InstallProgress::Downloading(p) => ("Downloading update...".to_string(), Some(*p)),
            InstallProgress::Verifying => ("Verifying download...".to_string(), None),
            InstallProgress::Installing => ("Installing update...".to_string(), None),
            InstallProgress::Complete => ("Update complete! Restart to apply.".to_string(), None),
            InstallProgress::Failed(err) => (err.clone(), None),
        };

        let is_error = matches!(progress, InstallProgress::Failed(_));
        let is_complete = matches!(progress, InstallProgress::Complete);

        // Copy theme colors for closures
        let error = theme.error;
        let success = theme.success;
        let surface = theme.surface;
        let border = theme.border;
        let background = theme.background;
        let text = theme.text;
        let accent = theme.accent;
        let surface_hover = theme.surface_hover;

        div()
            .w_full()
            .bg(if is_error {
                error.opacity(0.1)
            } else if is_complete {
                success.opacity(0.1)
            } else {
                surface
            })
            .border_1()
            .border_color(if is_error {
                error.opacity(0.3)
            } else if is_complete {
                success.opacity(0.3)
            } else {
                border
            })
            .rounded_md()
            .p_3()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        // Progress indicator
                        if is_complete {
                            div()
                                .w_6()
                                .h_6()
                                .rounded_full()
                                .bg(success)
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(div().text_color(background).text_xs().child("✓"))
                        } else if is_error {
                            div()
                                .w_6()
                                .h_6()
                                .rounded_full()
                                .bg(error)
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(div().text_color(background).text_xs().child("!"))
                        } else {
                            // Spinner placeholder
                            div()
                                .w_6()
                                .h_6()
                                .rounded_full()
                                .border_2()
                                .border_color(accent.opacity(0.3))
                                .border_color(accent)
                        },
                    )
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(div().text_color(text).text_sm().child(message))
                            .when(progress_value.is_some(), |this| {
                                let pv = progress_value.unwrap();
                                this.child(
                                    div().w_full().h_1().bg(surface_hover).rounded_full().child(
                                        div()
                                            .h_full()
                                            .bg(accent)
                                            .rounded_full()
                                            .w(relative(pv as f32 / 100.0)),
                                    ),
                                )
                            }),
                    )
                    .when(is_complete, |this| {
                        this.child(
                            div()
                                .px_4()
                                .py_1()
                                .rounded_md()
                                .cursor_pointer()
                                .bg(success)
                                .text_color(background)
                                .text_sm()
                                .font_weight(FontWeight::MEDIUM)
                                .hover(|s| s.opacity(0.9))
                                .child("Restart"),
                        )
                    }),
            )
    }
}
