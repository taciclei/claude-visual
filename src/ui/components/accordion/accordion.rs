//! Main accordion component

use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;

use super::types::*;
use crate::app::state::AppState;

/// Accordion component
pub struct Accordion {
    app_state: Arc<AppState>,
    /// Items in the accordion
    items: Vec<AccordionItem>,
    /// Currently expanded item indices
    expanded: Vec<usize>,
    /// Expansion mode
    mode: AccordionMode,
    /// Style variant
    style: AccordionStyle,
    /// Default expanded items
    default_expanded: Vec<usize>,
}

impl Accordion {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            items: Vec::new(),
            expanded: Vec::new(),
            mode: AccordionMode::default(),
            style: AccordionStyle::default(),
            default_expanded: Vec::new(),
        }
    }

    /// Set items
    pub fn set_items(&mut self, items: Vec<AccordionItem>, cx: &mut Context<Self>) {
        self.items = items;
        // Apply default expansion
        self.expanded = self.default_expanded.clone();
        cx.notify();
    }

    /// Add an item
    pub fn add_item(&mut self, item: AccordionItem, cx: &mut Context<Self>) {
        self.items.push(item);
        cx.notify();
    }

    /// Set expansion mode
    pub fn set_mode(&mut self, mode: AccordionMode, cx: &mut Context<Self>) {
        self.mode = mode;
        // If switching to single mode and multiple expanded, keep first
        if mode == AccordionMode::Single && self.expanded.len() > 1 {
            self.expanded = vec![self.expanded[0]];
        }
        cx.notify();
    }

    /// Set style
    pub fn set_style(&mut self, style: AccordionStyle, cx: &mut Context<Self>) {
        self.style = style;
        cx.notify();
    }

    /// Set default expanded items
    pub fn set_default_expanded(&mut self, indices: Vec<usize>, cx: &mut Context<Self>) {
        self.default_expanded = indices.clone();
        self.expanded = indices;
        cx.notify();
    }

    /// Toggle item expansion
    pub fn toggle(&mut self, index: usize, cx: &mut Context<Self>) {
        if index >= self.items.len() || self.items[index].disabled {
            return;
        }

        if self.expanded.contains(&index) {
            self.expanded.retain(|&i| i != index);
            cx.emit(AccordionEvent::Collapsed(index));
        } else {
            match self.mode {
                AccordionMode::Single => {
                    self.expanded = vec![index];
                }
                AccordionMode::Multiple => {
                    self.expanded.push(index);
                }
            }
            cx.emit(AccordionEvent::Expanded(index));
        }
        cx.notify();
    }

    /// Expand an item
    pub fn expand(&mut self, index: usize, cx: &mut Context<Self>) {
        if !self.expanded.contains(&index) {
            self.toggle(index, cx);
        }
    }

    /// Collapse an item
    pub fn collapse(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.expanded.contains(&index) {
            self.toggle(index, cx);
        }
    }

    /// Expand all (only in Multiple mode)
    pub fn expand_all(&mut self, cx: &mut Context<Self>) {
        if self.mode == AccordionMode::Multiple {
            self.expanded = (0..self.items.len())
                .filter(|&i| !self.items[i].disabled)
                .collect();
            cx.notify();
        }
    }

    /// Collapse all
    pub fn collapse_all(&mut self, cx: &mut Context<Self>) {
        self.expanded.clear();
        cx.notify();
    }

    /// Check if item is expanded
    pub fn is_expanded(&self, index: usize) -> bool {
        self.expanded.contains(&index)
    }
}

impl EventEmitter<AccordionEvent> for Accordion {}

impl Render for Accordion {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        let show_outer_border = matches!(
            self.style,
            AccordionStyle::Default | AccordionStyle::Separated
        );
        let show_dividers = matches!(self.style, AccordionStyle::Default | AccordionStyle::Flush);
        let separated = matches!(self.style, AccordionStyle::Separated);

        div()
            .id("accordion")
            .w_full()
            .flex()
            .flex_col()
            .when(separated, |d| d.gap_2())
            .when(show_outer_border && !separated, |d| {
                d.rounded(px(8.0))
                    .border_1()
                    .border_color(theme.colors.border)
                    .overflow_hidden()
            })
            .children(self.items.iter().enumerate().map(|(index, item)| {
                let is_expanded = self.expanded.contains(&index);
                let is_first = index == 0;
                let is_last = index == self.items.len() - 1;
                let opacity = if item.disabled { 0.5 } else { 1.0 };

                let chevron = if is_expanded { "▼" } else { "▶" };

                div()
                    .id(SharedString::from(format!("accordion-item-{}", index)))
                    .w_full()
                    .opacity(opacity)
                    .when(separated, |d| {
                        d.rounded(px(8.0))
                            .border_1()
                            .border_color(theme.colors.border)
                            .overflow_hidden()
                    })
                    .when(!separated && show_dividers && !is_first, |d| {
                        d.border_t_1().border_color(theme.colors.border)
                    })
                    // Header
                    .child(
                        div()
                            .id(SharedString::from(format!("accordion-header-{}", index)))
                            .w_full()
                            .h(px(48.0))
                            .px_4()
                            .flex()
                            .items_center()
                            .gap_3()
                            .bg(theme.colors.surface)
                            .when(!item.disabled, |d| {
                                d.cursor_pointer()
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .on_click(cx.listener(move |this, _, _window, cx| {
                                        this.toggle(index, cx);
                                    }))
                            })
                            // Icon
                            .when_some(item.icon.clone(), |d, icon| {
                                d.child(div().text_base().child(icon))
                            })
                            // Title and subtitle
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .flex_col()
                                    .justify_center()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(theme.colors.text)
                                            .child(item.title.clone()),
                                    )
                                    .when_some(item.subtitle.clone(), |d, subtitle| {
                                        d.child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.text_muted)
                                                .child(subtitle),
                                        )
                                    }),
                            )
                            // Chevron
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child(chevron),
                            ),
                    )
                    // Content (when expanded)
                    .when(is_expanded, |d| {
                        d.child(
                            div()
                                .id(SharedString::from(format!("accordion-content-{}", index)))
                                .w_full()
                                .px_4()
                                .py_3()
                                .border_t_1()
                                .border_color(theme.colors.border)
                                .text_sm()
                                .text_color(theme.colors.text_muted)
                                .child("Accordion content placeholder"),
                        )
                    })
            }))
    }
}
