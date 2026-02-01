use super::types::PatternType;
use gpui::prelude::*;
use gpui::*;

/// Pattern overlay (stripes, dots, grid)
#[derive(IntoElement)]
pub struct PatternOverlay {
    id: ElementId,
    pattern: PatternType,
    color: gpui::Hsla,
    opacity: f32,
    size: f32,
    spacing: f32,
}

impl PatternOverlay {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            pattern: PatternType::default(),
            color: rgba(0x000000ff).into(),
            opacity: 0.05,
            size: 4.0,
            spacing: 20.0,
        }
    }

    pub fn pattern(mut self, pattern: PatternType) -> Self {
        self.pattern = pattern;
        self
    }

    pub fn color(mut self, color: gpui::Hsla) -> Self {
        self.color = color;
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl RenderOnce for PatternOverlay {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let pattern_color = self.color.opacity(self.opacity);

        div()
            .id(self.id)
            .absolute()
            .inset_0()
            .overflow_hidden()
            .child(match self.pattern {
                PatternType::Dots => {
                    // Grid of dots
                    div()
                        .size_full()
                        .flex()
                        .flex_wrap()
                        .gap(px(self.spacing))
                        .p(px(self.spacing / 2.0))
                        .children(
                            (0..100).map(|_| {
                                div().size(px(self.size)).rounded_full().bg(pattern_color)
                            }),
                        )
                }
                PatternType::Grid => {
                    // Grid lines
                    div()
                        .size_full()
                        .flex()
                        .flex_col()
                        .gap(px(self.spacing))
                        .children((0..20).map(|_| div().w_full().h(px(1.0)).bg(pattern_color)))
                }
                PatternType::HorizontalStripes => {
                    div()
                        .size_full()
                        .flex()
                        .flex_col()
                        .children((0..30).map(|i| {
                            div()
                                .w_full()
                                .h(px(self.size))
                                .when(i % 2 == 0, |d| d.bg(pattern_color))
                                .mb(px(self.spacing))
                        }))
                }
                PatternType::VerticalStripes => {
                    div().size_full().flex().children((0..30).map(|i| {
                        div()
                            .h_full()
                            .w(px(self.size))
                            .when(i % 2 == 0, |d| d.bg(pattern_color))
                            .mr(px(self.spacing))
                    }))
                }
                _ => {
                    // Default to dots for other patterns
                    div()
                        .size_full()
                        .flex()
                        .flex_wrap()
                        .gap(px(self.spacing))
                        .children(
                            (0..100).map(|_| {
                                div().size(px(self.size)).rounded_full().bg(pattern_color)
                            }),
                        )
                }
            })
    }
}
