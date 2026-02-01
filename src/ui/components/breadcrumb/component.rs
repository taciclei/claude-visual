//! Breadcrumb navigation component implementation

use std::sync::Arc;

use gpui::*;
use gpui::prelude::*;

use crate::app::state::AppState;
use super::types::*;

impl EventEmitter<BreadcrumbEvent> for Breadcrumb {}

/// Breadcrumb navigation component
pub struct Breadcrumb {
    app_state: Arc<AppState>,
    items: Vec<BreadcrumbItem>,
    /// Separator character
    separator: String,
    /// Maximum items to show before truncating
    max_visible: usize,
    /// Whether to show home icon
    show_home: bool,
}

impl Breadcrumb {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            items: Vec::new(),
            separator: "/".to_string(),
            max_visible: 5,
            show_home: true,
        }
    }

    /// Set the breadcrumb items
    pub fn set_items(&mut self, items: Vec<BreadcrumbItem>, cx: &mut Context<Self>) {
        self.items = items;
        cx.notify();
    }

    /// Add an item to the end
    pub fn push(&mut self, item: BreadcrumbItem, cx: &mut Context<Self>) {
        self.items.push(item);
        cx.notify();
    }

    /// Remove the last item
    pub fn pop(&mut self, cx: &mut Context<Self>) -> Option<BreadcrumbItem> {
        let item = self.items.pop();
        cx.notify();
        item
    }

    /// Clear all items
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.items.clear();
        cx.notify();
    }

    /// Set the separator
    pub fn set_separator(&mut self, separator: impl Into<String>, cx: &mut Context<Self>) {
        self.separator = separator.into();
        cx.notify();
    }

    /// Set maximum visible items
    pub fn set_max_visible(&mut self, max: usize, cx: &mut Context<Self>) {
        self.max_visible = max;
        cx.notify();
    }

    /// Set whether to show home icon
    pub fn set_show_home(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_home = show;
        cx.notify();
    }

    /// Build breadcrumb from a file path
    pub fn from_path(&mut self, path: &str, cx: &mut Context<Self>) {
        self.items.clear();

        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        for (i, part) in parts.iter().enumerate() {
            let id = parts[..=i].join("/");
            let mut item = BreadcrumbItem::new(id, *part);

            // Add folder icon for directories (all except last)
            if i < parts.len() - 1 {
                item = item.with_icon("📁");
            } else {
                // Determine icon based on extension
                let icon = if part.ends_with(".rs") {
                    "🦀"
                } else if part.ends_with(".js") || part.ends_with(".ts") {
                    "📜"
                } else if part.ends_with(".py") {
                    "🐍"
                } else if part.ends_with(".md") {
                    "📝"
                } else if part.ends_with(".json") || part.ends_with(".toml") {
                    "⚙️"
                } else {
                    "📄"
                };
                item = item.with_icon(icon);
            }

            self.items.push(item);
        }

        cx.notify();
    }

    /// Get the visible items (handles truncation)
    fn visible_items(&self) -> Vec<(usize, &BreadcrumbItem, bool)> {
        let total = self.items.len();

        if total <= self.max_visible {
            return self.items.iter()
                .enumerate()
                .map(|(i, item)| (i, item, i == total - 1))
                .collect();
        }

        // Show first item, ellipsis, then last (max_visible - 2) items
        let mut result = Vec::new();

        // First item
        if let Some(first) = self.items.first() {
            result.push((0, first, false));
        }

        // Last items (excluding first)
        let start = total.saturating_sub(self.max_visible - 1);
        for i in start..total {
            result.push((i, &self.items[i], i == total - 1));
        }

        result
    }
}

impl Render for Breadcrumb {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let separator = self.separator.clone();
        let visible = self.visible_items();
        let total_items = self.items.len();
        let show_ellipsis = total_items > self.max_visible;

        div()
            .id("breadcrumb")
            .flex()
            .items_center()
            .gap_1()
            .text_sm()
            // Home icon
            .when(self.show_home, |this| {
                this.child(
                    div()
                        .id("breadcrumb-home")
                        .flex()
                        .items_center()
                        .justify_center()
                        .size(px(20.0))
                        .rounded_sm()
                        .text_color(theme.colors.text_muted)
                        .hover(|s| s.bg(theme.colors.surface_hover).text_color(theme.colors.text))
                        .cursor_pointer()
                        .on_click(cx.listener(|_this, _, _window, cx| {
                            cx.emit(BreadcrumbEvent::ItemClicked("home".to_string()));
                        }))
                        .child("🏠")
                )
                .child(
                    div()
                        .text_color(theme.colors.text_muted)
                        .child(separator.clone())
                )
            })
            // Breadcrumb items
            .children(visible.iter().enumerate().map(|(display_idx, (original_idx, item, is_last))| {
                let item_id = item.id.clone();
                let show_separator = !*is_last;
                let is_clickable = item.clickable;

                // Show ellipsis after first item if truncated
                let show_ellipsis_here = show_ellipsis && display_idx == 0 && *original_idx == 0;

                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    // Item itself
                    .child(
                        div()
                            .id(SharedString::from(format!("breadcrumb-{}", original_idx)))
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_1p5()
                            .py_0p5()
                            .rounded_sm()
                            .when(*is_last, |d| d.text_color(theme.colors.text).font_weight(FontWeight::MEDIUM))
                            .when(!*is_last, |d| d.text_color(theme.colors.text_muted))
                            .when(is_clickable && !*is_last, |d| {
                                d.hover(|s| s.bg(theme.colors.surface_hover).text_color(theme.colors.text))
                                    .cursor_pointer()
                            })
                            .when(is_clickable, |d| {
                                d.on_click(cx.listener(move |_this, _, _window, cx| {
                                    cx.emit(BreadcrumbEvent::ItemClicked(item_id.clone()));
                                }))
                            })
                            .when_some(item.icon.clone(), |d, icon| {
                                d.child(div().text_xs().child(icon))
                            })
                            .child(item.label.clone())
                    )
                    // Ellipsis if needed
                    .when(show_ellipsis_here, |d| {
                        d.child(
                            div()
                                .text_color(theme.colors.text_muted)
                                .child(separator.clone())
                        )
                        .child(
                            div()
                                .px_1()
                                .text_color(theme.colors.text_muted)
                                .child("...")
                        )
                    })
                    // Separator
                    .when(show_separator, |d| {
                        d.child(
                            div()
                                .text_color(theme.colors.text_muted)
                                .child(separator.clone())
                        )
                    })
            }))
    }
}
