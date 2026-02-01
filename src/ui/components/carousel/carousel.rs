//! Main carousel component

use gpui::*;
use gpui::prelude::*;

use super::types::{CarouselAnimation, CarouselNavigation, CarouselSlide};

/// Carousel component
#[derive(Clone)]
pub struct Carousel {
    pub(super) slides: Vec<CarouselSlide>,
    pub(super) current_index: usize,
    pub(super) navigation: CarouselNavigation,
    pub(super) animation: CarouselAnimation,
    pub(super) auto_play: bool,
    pub(super) auto_play_interval_ms: u64,
    pub(super) loop_slides: bool,
    pub(super) show_progress: bool,
}

impl Carousel {
    pub fn new() -> Self {
        Self {
            slides: Vec::new(),
            current_index: 0,
            navigation: CarouselNavigation::default(),
            animation: CarouselAnimation::default(),
            auto_play: false,
            auto_play_interval_ms: 5000,
            loop_slides: true,
            show_progress: false,
        }
    }

    pub fn slides(mut self, slides: Vec<CarouselSlide>) -> Self {
        self.slides = slides;
        self
    }

    pub fn slide(mut self, slide: CarouselSlide) -> Self {
        self.slides.push(slide);
        self
    }

    pub fn current(mut self, index: usize) -> Self {
        self.current_index = index;
        self
    }

    pub fn navigation(mut self, nav: CarouselNavigation) -> Self {
        self.navigation = nav;
        self
    }

    pub fn animation(mut self, anim: CarouselAnimation) -> Self {
        self.animation = anim;
        self
    }

    pub fn auto_play(mut self, interval_ms: u64) -> Self {
        self.auto_play = true;
        self.auto_play_interval_ms = interval_ms;
        self
    }

    pub fn no_loop(mut self) -> Self {
        self.loop_slides = false;
        self
    }

    pub fn show_progress(mut self) -> Self {
        self.show_progress = true;
        self
    }
}

impl Default for Carousel {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Carousel {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.12, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.18, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);

        let total_slides = self.slides.len();
        let current = self.current_index.min(total_slides.saturating_sub(1));
        let show_arrows = matches!(self.navigation, CarouselNavigation::Arrows | CarouselNavigation::Both);
        let show_dots = matches!(self.navigation, CarouselNavigation::Dots | CarouselNavigation::Both);
        let can_go_prev = self.loop_slides || current > 0;
        let can_go_next = self.loop_slides || current < total_slides.saturating_sub(1);

        let current_slide = self.slides.get(current).cloned();

        div()
            .w_full()
            .relative()
            .flex()
            .flex_col()
            // Main slide area
            .child(
                div()
                    .w_full()
                    .h(px(300.0))
                    .relative()
                    .overflow_hidden()
                    .rounded(px(8.0))
                    .bg(surface)
                    // Current slide
                    .when_some(current_slide, |d, slide| {
                        d.child(
                            div()
                                .absolute()
                                .inset_0()
                                .bg(slide.background.unwrap_or(surface))
                                .flex()
                                .flex_col()
                                .items_center()
                                .justify_center()
                                .gap_4()
                                .p_6()
                                // Image/icon
                                .when_some(slide.image, |d, img| {
                                    d.child(
                                        div()
                                            .text_3xl()
                                            .child(img)
                                    )
                                })
                                // Title
                                .when_some(slide.title, |d, title| {
                                    d.child(
                                        div()
                                            .text_xl()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(text)
                                            .text_center()
                                            .child(title)
                                    )
                                })
                                // Description
                                .when_some(slide.description, |d, desc| {
                                    d.child(
                                        div()
                                            .text_sm()
                                            .text_color(text_muted)
                                            .text_center()
                                            .max_w(px(400.0))
                                            .child(desc)
                                    )
                                })
                        )
                    })
                    // Navigation arrows
                    .when(show_arrows && total_slides > 1, |d| {
                        d
                            // Previous arrow
                            .child(
                                div()
                                    .absolute()
                                    .left_2()
                                    .top_1_2()

                                    .size(px(40.0))
                                    .rounded_full()
                                    .bg(hsla(0.0, 0.0, 0.0, 0.5))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(if can_go_prev { text } else { text_muted })
                                    .cursor(if can_go_prev { CursorStyle::PointingHand } else { CursorStyle::default() })
                                    .when(can_go_prev, |d| d.hover(|s| s.bg(hsla(0.0, 0.0, 0.0, 0.7))))
                                    .child("‹")
                            )
                            // Next arrow
                            .child(
                                div()
                                    .absolute()
                                    .right_2()
                                    .top_1_2()

                                    .size(px(40.0))
                                    .rounded_full()
                                    .bg(hsla(0.0, 0.0, 0.0, 0.5))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(if can_go_next { text } else { text_muted })
                                    .cursor(if can_go_next { CursorStyle::PointingHand } else { CursorStyle::default() })
                                    .when(can_go_next, |d| d.hover(|s| s.bg(hsla(0.0, 0.0, 0.0, 0.7))))
                                    .child("›")
                            )
                    })
                    // Progress bar
                    .when(self.show_progress && total_slides > 1, |d| {
                        d.child(
                            div()
                                .absolute()
                                .bottom_0()
                                .left_0()
                                .right_0()
                                .h(px(3.0))
                                .bg(hsla(0.0, 0.0, 0.0, 0.3))
                                .child(
                                    div()
                                        .h_full()
                                        .w(relative((current + 1) as f32 / total_slides as f32))
                                        .bg(accent)
                                )
                        )
                    })
            )
            // Dots navigation
            .when(show_dots && total_slides > 1, |d| {
                d.child(
                    div()
                        .w_full()
                        .pt_3()
                        .flex()
                        .items_center()
                        .justify_center()
                        .gap_2()
                        .children(
                            (0..total_slides).map(move |idx| {
                                let is_current = idx == current;
                                div()
                                    .size(px(if is_current { 10.0 } else { 8.0 }))
                                    .rounded_full()
                                    .bg(if is_current { accent } else { text_muted.opacity(0.3) })
                                    .cursor_pointer()
                                    .hover(|s| s.bg(if is_current { accent } else { text_muted.opacity(0.5) }))
                            })
                        )
                )
            })
    }
}
