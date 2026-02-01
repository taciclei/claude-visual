//! Completion dropdown rendering

use gpui::*;
use gpui::prelude::*;
use super::dropdown::CompletionDropdown;
use super::types::{default_colors, CompletionDropdownEvent};

impl Render for CompletionDropdown {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible || self.items.is_empty() {
            return div().into_any_element();
        }

        let colors = default_colors();

        // Copy colors for closures
        let surface = colors.surface;
        let border = colors.border;
        let hover = colors.hover;
        let selection = colors.selection;
        let text = colors.text;
        let text_muted = colors.text_muted;

        let visible_items: Vec<_> = self
            .items
            .iter()
            .enumerate()
            .skip(self.scroll_offset)
            .take(self.max_visible_items)
            .collect();

        div()
            .absolute()
            .left(self.position.x)
            .top(self.position.y)
            .w(px(350.0))
            .max_h(px(300.0))
            .bg(surface)
            .border_1()
            .border_color(border)
            .rounded_md()
            .shadow_lg()
            .overflow_hidden()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .children(visible_items.into_iter().map(|(index, item)| {
                        let is_selected = index == self.selected_index;
                        let kind = item.kind;
                        let label = item.label.clone();
                        let detail = item.detail.clone();
                        let kind_icon = Self::kind_icon(kind);
                        let kind_color = Self::kind_color(kind, &colors);

                        let on_mouse_down_handler = {
                            let item = item.clone();
                            cx.listener(move |this, _, _window, cx| {
                                cx.emit(CompletionDropdownEvent::Selected(item.clone()));
                                this.hide(cx);
                            })
                        };

                        div()
                            .id(ElementId::Name(format!("completion-{}", index).into()))
                            .px_2()
                            .py_1()
                            .cursor_pointer()
                            .when(is_selected, |this| {
                                this.bg(selection)
                            })
                            .hover(|style| style.bg(hover))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .w(px(20.0))
                                            .h(px(20.0))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .rounded_sm()
                                            .bg(kind_color.opacity(0.2))
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(kind_color)
                                                    .child(kind_icon),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .flex_1()
                                            .overflow_hidden()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(text)
                                                    .text_ellipsis()
                                                    .child(label),
                                            )
                                            .when(detail.is_some(), |this| {
                                                this.child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(text_muted)
                                                        .text_ellipsis()
                                                        .child(detail.unwrap_or_default()),
                                                )
                                            }),
                                    ),
                            )
                            .on_mouse_down(MouseButton::Left, on_mouse_down_handler)
                    })),
            )
            .when(self.items.len() > self.max_visible_items, |this| {
                this.child(
                    div()
                        .px_2()
                        .py_1()
                        .border_t_1()
                        .border_color(border)
                        .child(
                            div()
                                .text_xs()
                                .text_color(text_muted)
                                .child(format!(
                                    "{}/{} items",
                                    self.selected_index + 1,
                                    self.items.len()
                                )),
                        ),
                )
            })
            .into_any_element()
    }
}
