//! Favorites panel render functions

use gpui::prelude::*;
use gpui::*;

use super::super::core::ChatView;

impl ChatView {
    pub fn render_favorites_panel(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let favorites = self.favorites_by_usage();
        let has_favorites = !favorites.is_empty();

        div()
            .id("favorites-panel-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle_favorites_panel(cx);
            }))
            .child(
                div()
                    .id("favorites-panel")
                    .w(px(500.0))
                    .max_h(px(450.0))
                    .bg(theme.colors.surface)
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.colors.border)
                    .shadow_lg()
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    .on_click(|_, _, _| {})
                    // Header
                    .child(
                        div()
                            .px_4()
                            .py_3()
                            .border_b_1()
                            .border_color(theme.colors.border)
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(div().text_base().child("⭐"))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.colors.text)
                                            .child("Favorite Prompts"),
                                    )
                                    .when(has_favorites, |d| {
                                        d.child(
                                            div()
                                                .px_2()
                                                .py_0p5()
                                                .rounded_full()
                                                .bg(theme.colors.accent.opacity(0.15))
                                                .text_xs()
                                                .text_color(theme.colors.accent)
                                                .child(format!("{}", favorites.len())),
                                        )
                                    }),
                            )
                            .child(
                                div()
                                    .id("close-favorites-panel")
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_sm()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_favorites_panel(cx);
                                    }))
                                    .child("×"),
                            ),
                    )
                    // Favorites list
                    .child(
                        div()
                            .id("favorites-list")
                            .flex_1()
                            .overflow_y_scroll()
                            .when(!has_favorites, |d| {
                                d.child(self.render_favorites_empty_state(theme, cx))
                            })
                            .when(has_favorites, |d| {
                                d.children(favorites.iter().map(|fav| {
                                    let fav_id = fav.id.clone();
                                    let fav_id_delete = fav.id.clone();
                                    div()
                                        .id(ElementId::Name(format!("fav-{}", fav.id).into()))
                                        .px_4()
                                        .py_3()
                                        .border_b_1()
                                        .border_color(theme.colors.border.opacity(0.5))
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.colors.surface_hover))
                                        .on_click(cx.listener(move |this, _, _window, cx| {
                                            this.use_favorite(&fav_id, cx);
                                        }))
                                        .child(
                                            div()
                                                .flex()
                                                .items_start()
                                                .justify_between()
                                                .gap_2()
                                                .child(
                                                    div()
                                                        .flex_1()
                                                        .child(
                                                            div()
                                                                .text_sm()
                                                                .font_weight(FontWeight::MEDIUM)
                                                                .text_color(theme.colors.text)
                                                                .mb_1()
                                                                .child(fav.label.clone()),
                                                        )
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(theme.colors.text_muted)
                                                                .max_w(px(350.0))
                                                                .overflow_hidden()
                                                                .child(if fav.text.len() > 100 {
                                                                    format!(
                                                                        "{}...",
                                                                        &fav.text[..100]
                                                                    )
                                                                } else {
                                                                    fav.text.clone()
                                                                }),
                                                        ),
                                                )
                                                .child(
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap_2()
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(theme.colors.text_muted)
                                                                .child(format!(
                                                                    "×{}",
                                                                    fav.usage_count
                                                                )),
                                                        )
                                                        .child(
                                                            div()
                                                                .id(ElementId::Name(
                                                                    format!("del-fav-{}", fav.id)
                                                                        .into(),
                                                                ))
                                                                .px_1()
                                                                .rounded_sm()
                                                                .cursor_pointer()
                                                                .text_xs()
                                                                .text_color(theme.colors.text_muted)
                                                                .hover(|s| {
                                                                    s.text_color(theme.colors.error)
                                                                })
                                                                .on_click(cx.listener(
                                                                    move |this, _, _window, cx| {
                                                                        this.remove_favorite(
                                                                            &fav_id_delete,
                                                                            cx,
                                                                        );
                                                                    },
                                                                ))
                                                                .child("×"),
                                                        ),
                                                ),
                                        )
                                }))
                            }),
                    ),
            )
    }

    /// Render empty state with skill suggestions
    fn render_favorites_empty_state(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        use super::super::types::ChatViewEvent;

        let accent = theme.colors.accent;
        let info = theme.colors.info;
        let success = theme.colors.success;

        div()
            .p_8()
            .flex()
            .flex_col()
            .items_center()
            .gap_3()
            .child(
                div()
                    .size(px(48.0))
                    .rounded_full()
                    .bg(theme.colors.accent.opacity(0.1))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_xl()
                    .child("⭐"),
            )
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.colors.text)
                    .child("No favorites yet"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .text_center()
                    .child("Save useful prompts with ⌘K → 'Save to Favorites'"),
            )
            // Suggested prompts to save
            .child(
                div()
                    .pt_4()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .w_full()
                    .max_w(px(350.0))
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child("Try these common skills:"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_2()
                            .justify_center()
                            // APEX
                            .child(
                                div()
                                    .id("fav-empty-apex")
                                    .px_3()
                                    .py_2()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(accent.opacity(0.15))
                                    .border_1()
                                    .border_color(accent.opacity(0.3))
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(accent)
                                    .hover(move |s| {
                                        s.bg(accent.opacity(0.25))
                                            .border_color(accent.opacity(0.5))
                                    })
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_favorites_panel(cx);
                                        cx.emit(ChatViewEvent::Submit("/apex".to_string()));
                                    }))
                                    .child("⚡ /apex"),
                            )
                            // Explore
                            .child(
                                div()
                                    .id("fav-empty-explore")
                                    .px_3()
                                    .py_2()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(info.opacity(0.15))
                                    .border_1()
                                    .border_color(info.opacity(0.3))
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(info)
                                    .hover(move |s| {
                                        s.bg(info.opacity(0.25))
                                            .border_color(info.opacity(0.5))
                                    })
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_favorites_panel(cx);
                                        cx.emit(ChatViewEvent::Submit("/explore".to_string()));
                                    }))
                                    .child("🔍 /explore"),
                            )
                            // Commit
                            .child(
                                div()
                                    .id("fav-empty-commit")
                                    .px_3()
                                    .py_2()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(success.opacity(0.15))
                                    .border_1()
                                    .border_color(success.opacity(0.3))
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(success)
                                    .hover(move |s| {
                                        s.bg(success.opacity(0.25))
                                            .border_color(success.opacity(0.5))
                                    })
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_favorites_panel(cx);
                                        cx.emit(ChatViewEvent::Submit("/commit".to_string()));
                                    }))
                                    .child("📦 /commit"),
                            ),
                    ),
            )
    }
}
