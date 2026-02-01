//! Testimonial carousel component

use gpui::*;
use gpui::prelude::*;

use super::types::Testimonial;

/// Testimonial carousel
#[derive(Clone)]
pub struct TestimonialCarousel {
    testimonials: Vec<Testimonial>,
    current_index: usize,
}

impl TestimonialCarousel {
    pub fn new() -> Self {
        Self {
            testimonials: Vec::new(),
            current_index: 0,
        }
    }

    pub fn testimonials(mut self, testimonials: Vec<Testimonial>) -> Self {
        self.testimonials = testimonials;
        self
    }

    pub fn current(mut self, index: usize) -> Self {
        self.current_index = index;
        self
    }
}

impl Default for TestimonialCarousel {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for TestimonialCarousel {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.12, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);
        let star_color = hsla(0.12, 0.9, 0.5, 1.0);

        let total = self.testimonials.len();
        let current = self.current_index.min(total.saturating_sub(1));
        let current_testimonial = self.testimonials.get(current).cloned();

        div()
            .w_full()
            .max_w(px(600.0))
            .bg(surface)
            .rounded(px(12.0))
            .p_6()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .when_some(current_testimonial, |d, t| {
                d
                    // Quote mark
                    .child(
                        div()
                            .text_3xl()
                            .text_color(accent.opacity(0.3))
                            .child("\u{201C}")
                    )
                    // Quote text
                    .child(
                        div()
                            .text_base()
                            .text_color(text)
                            .text_center()
                            .italic()
                            .child(t.quote)
                    )
                    // Rating
                    .when_some(t.rating, |d, rating| {
                        d.child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .children(
                                    (0..5).map(move |i| {
                                        div()
                                            .text_sm()
                                            .text_color(if i < rating { star_color } else { text_muted.opacity(0.3) })
                                            .child("★")
                                    })
                                )
                        )
                    })
                    // Author
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .pt_2()
                            // Avatar
                            .when_some(t.avatar, |d, avatar| {
                                d.child(
                                    div()
                                        .size(px(48.0))
                                        .rounded_full()
                                        .bg(accent.opacity(0.2))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_lg()
                                        .child(avatar)
                                )
                            })
                            // Name and role
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(text)
                                            .child(t.author)
                                    )
                                    .when_some(t.role, |d, role| {
                                        d.child(
                                            div()
                                                .text_xs()
                                                .text_color(text_muted)
                                                .child(role)
                                        )
                                    })
                            )
                    )
            })
            // Dots
            .when(total > 1, |d| {
                d.child(
                    div()
                        .pt_4()
                        .flex()
                        .items_center()
                        .gap_2()
                        .children(
                            (0..total).map(move |idx| {
                                let is_current = idx == current;
                                div()
                                    .size(px(8.0))
                                    .rounded_full()
                                    .bg(if is_current { accent } else { text_muted.opacity(0.3) })
                                    .cursor_pointer()
                            })
                        )
                )
            })
    }
}
