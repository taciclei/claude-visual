//! Diff statistics and file change badge components

use gpui::*;
use gpui::prelude::*;
use crate::ui::pct;
use super::types::*;

/// Diff stat summary
#[derive(IntoElement)]
pub struct DiffStat {
    id: ElementId,
    files_changed: usize,
    additions: usize,
    deletions: usize,
    show_bar: bool,
    compact: bool,
}

impl DiffStat {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            files_changed: 0,
            additions: 0,
            deletions: 0,
            show_bar: true,
            compact: false,
        }
    }

    pub fn files_changed(mut self, count: usize) -> Self {
        self.files_changed = count;
        self
    }

    pub fn additions(mut self, count: usize) -> Self {
        self.additions = count;
        self
    }

    pub fn deletions(mut self, count: usize) -> Self {
        self.deletions = count;
        self
    }

    pub fn show_bar(mut self, show: bool) -> Self {
        self.show_bar = show;
        self
    }

    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    fn total(&self) -> usize {
        self.additions + self.deletions
    }
}

impl RenderOnce for DiffStat {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let total = self.total();
        let add_ratio = if total > 0 {
            self.additions as f32 / total as f32
        } else {
            0.0
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap_3()
            .text_sm()
            .when(!self.compact, |d| {
                d.child(
                    div()
                        .text_color(rgba(0xccccccff))
                        .child(format!(
                            "{} file{} changed",
                            self.files_changed,
                            if self.files_changed == 1 { "" } else { "s" }
                        )),
                )
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .text_color(rgb(0x22c55e))
                    .child(format!("+{}", self.additions)),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .text_color(rgb(0xef4444))
                    .child(format!("-{}", self.deletions)),
            )
            .when(self.show_bar && total > 0, |d| {
                d.child(
                    div()
                        .flex()
                        .w(px(50.0))
                        .h(px(8.0))
                        .rounded_full()
                        .overflow_hidden()
                        .bg(rgb(0xef4444))
                        .child(
                            div()
                                .h_full()
                                .w(pct(add_ratio * 100.0))
                                .bg(rgb(0x22c55e)),
                        ),
                )
            })
    }
}

/// File change badge
#[derive(IntoElement)]
pub struct FileChangeBadge {
    id: ElementId,
    change_type: FileChangeType,
    file_name: SharedString,
    show_icon: bool,
}

impl FileChangeBadge {
    pub fn new(id: impl Into<ElementId>, file_name: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            change_type: FileChangeType::default(),
            file_name: file_name.into(),
            show_icon: true,
        }
    }

    pub fn change_type(mut self, change_type: FileChangeType) -> Self {
        self.change_type = change_type;
        self
    }

    pub fn show_icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }
}

impl RenderOnce for FileChangeBadge {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.change_type.color();
        let label = self.change_type.label().to_string();

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap_2()
            .when(self.show_icon, |d| {
                d.child(
                    div()
                        .size(px(18.0))
                        .rounded(px(3.0))
                        .bg(color.opacity(0.2))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_xs()
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_color(color)
                        .child(label),
                )
            })
            .child(
                div()
                    .text_sm()
                    .font_family("monospace")
                    .text_color(rgba(0xccccccff))
                    .child(self.file_name.clone()),
            )
    }
}
