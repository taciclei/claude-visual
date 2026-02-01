//! Rendering implementation for lazy blocks

use super::config::LazyBlockConfig;
use super::core::LazyBlock;
use super::types::{default_colors, LazyState, SimpleColors};
use crate::ui::pct;
use gpui::prelude::*;
use gpui::*;

impl<T: IntoElement + Clone + 'static> Render for LazyBlock<T> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = default_colors();
        let id = self.id.clone();
        let height = self.config.estimated_height;
        let show_skeleton = self.config.show_skeleton;

        match self.state {
            LazyState::Pending | LazyState::Loading => {
                // Render placeholder
                div()
                    .id(id)
                    .w_full()
                    .h(px(height))
                    .rounded_md()
                    .bg(colors.surface)
                    .when(show_skeleton, |d| {
                        d.child(skeleton_placeholder(height, &colors))
                    })
            }
            LazyState::Loaded => {
                // Render actual content
                if let Some(ref content) = self.cached_content {
                    div().id(id).w_full().child(content.clone())
                } else {
                    // Fallback if content somehow missing
                    div()
                        .id(id)
                        .w_full()
                        .h(px(height))
                        .rounded_md()
                        .bg(colors.surface)
                }
            }
            LazyState::Error => {
                // Render error state
                let error_msg = self
                    .error
                    .as_deref()
                    .unwrap_or("Failed to load content")
                    .to_string();
                div()
                    .id(id)
                    .w_full()
                    .h(px(60.0))
                    .rounded_md()
                    .bg(colors.error.opacity(0.1))
                    .border_1()
                    .border_color(colors.error.opacity(0.3))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(div().text_sm().text_color(colors.error).child(error_msg))
            }
        }
    }
}

/// Render a skeleton placeholder
fn skeleton_placeholder(height: f32, colors: &SimpleColors) -> impl IntoElement {
    let line_count = (height / 20.0) as usize;

    div()
        .w_full()
        .h_full()
        .p_3()
        .flex()
        .flex_col()
        .gap_2()
        .children((0..line_count.min(10)).map(|i| {
            let width_pct = match i % 4 {
                0 => 85.0,
                1 => 70.0,
                2 => 90.0,
                _ => 60.0,
            };

            div()
                .h(px(14.0))
                .w(pct(width_pct))
                .rounded_sm()
                .bg(colors.border.opacity(0.5))
        }))
}

/// Helper to create a lazy code block
pub fn lazy_code_block(
    id: impl Into<ElementId>,
    code: String,
    language: Option<String>,
    line_count: usize,
) -> impl IntoElement {
    let config = LazyBlockConfig::for_code_block(line_count);
    let _code = code.clone();
    let _language = language.clone();

    // Return a placeholder div that represents where a lazy block would be
    // In a real implementation, this would be integrated with the virtual list
    div()
        .id(id.into())
        .w_full()
        .min_h(px(config.estimated_height))
        .rounded_md()
        .bg(hsla(0.0, 0.0, 0.1, 1.0))
        .child(
            div()
                .text_xs()
                .p_2()
                .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                .child(format!(
                    "{} ({} lines)",
                    language.unwrap_or_else(|| "text".to_string()),
                    line_count
                )),
        )
}
