//! Skeleton loading placeholder

use gpui::*;
use gpui::prelude::*;

/// Skeleton loading placeholder
#[derive(Clone)]
pub struct SkeletonLoader {
    lines: u32,
    show_avatar: bool,
    show_title: bool,
    animated: bool,
}

impl SkeletonLoader {
    pub fn new() -> Self {
        Self {
            lines: 3,
            show_avatar: false,
            show_title: true,
            animated: true,
        }
    }

    pub fn lines(mut self, count: u32) -> Self {
        self.lines = count;
        self
    }

    pub fn with_avatar(mut self) -> Self {
        self.show_avatar = true;
        self
    }

    pub fn with_title(mut self, show: bool) -> Self {
        self.show_title = show;
        self
    }

    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }
}

impl Default for SkeletonLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for SkeletonLoader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let skeleton_color = hsla(0.0, 0.0, 0.2, 1.0);
        let skeleton_highlight = hsla(0.0, 0.0, 0.25, 1.0);

        let line_widths = vec![100, 85, 70, 90, 60];

        div()
            .flex()
            .gap_3()
            // Avatar
            .when(self.show_avatar, |d| {
                d.child(
                    div()
                        .flex_shrink_0()
                        .size(px(40.0))
                        .rounded_full()
                        .bg(skeleton_color)
                )
            })
            // Content
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_2()
                    // Title
                    .when(self.show_title, |d| {
                        d.child(
                            div()
                                .h(px(16.0))
                                .w(px(120.0))
                                .rounded(px(4.0))
                                .bg(skeleton_color)
                        )
                    })
                    // Lines
                    .children(
                        (0..self.lines).map(|i| {
                            let width_pct = line_widths.get(i as usize).copied().unwrap_or(80);
                            div()
                                .h(px(12.0))
                                .w(relative(width_pct as f32 / 100.0))
                                .rounded(px(4.0))
                                .bg(skeleton_color)
                        })
                    )
            )
    }
}
