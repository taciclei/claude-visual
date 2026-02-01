//! Main SearchInput component

use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;

use super::types::*;
use crate::app::state::AppState;

/// Search input component
pub struct SearchInput {
    app_state: Arc<AppState>,
    /// Current query
    query: String,
    /// Placeholder text
    placeholder: String,
    /// Size variant
    size: SearchInputSize,
    /// Whether input is disabled
    disabled: bool,
    /// Whether to show clear button
    show_clear: bool,
    /// Whether input is focused
    focused: bool,
    /// Focus handle
    focus_handle: FocusHandle,
    /// Optional keyboard shortcut hint
    shortcut_hint: Option<String>,
    /// Whether search is loading
    loading: bool,
    /// Result count (for display)
    result_count: Option<(usize, usize)>, // (current, total)
}

impl SearchInput {
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            query: String::new(),
            placeholder: "Search...".to_string(),
            size: SearchInputSize::default(),
            disabled: false,
            show_clear: true,
            focused: false,
            focus_handle: cx.focus_handle(),
            shortcut_hint: None,
            loading: false,
            result_count: None,
        }
    }

    /// Create with placeholder
    pub fn with_placeholder(
        app_state: Arc<AppState>,
        placeholder: impl Into<String>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut input = Self::new(app_state, cx);
        input.placeholder = placeholder.into();
        input
    }

    /// Set query
    pub fn set_query(&mut self, query: impl Into<String>, cx: &mut Context<Self>) {
        self.query = query.into();
        cx.notify();
    }

    /// Get query
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Clear query
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        if !self.query.is_empty() {
            self.query.clear();
            cx.emit(SearchInputEvent::Cleared);
            cx.notify();
        }
    }

    /// Set placeholder
    pub fn set_placeholder(&mut self, placeholder: impl Into<String>, cx: &mut Context<Self>) {
        self.placeholder = placeholder.into();
        cx.notify();
    }

    /// Set size
    pub fn set_size(&mut self, size: SearchInputSize, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }

    /// Set disabled
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        cx.notify();
    }

    /// Set show clear button
    pub fn set_show_clear(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_clear = show;
        cx.notify();
    }

    /// Set shortcut hint
    pub fn set_shortcut_hint(&mut self, hint: Option<String>, cx: &mut Context<Self>) {
        self.shortcut_hint = hint;
        cx.notify();
    }

    /// Set loading state
    pub fn set_loading(&mut self, loading: bool, cx: &mut Context<Self>) {
        self.loading = loading;
        cx.notify();
    }

    /// Set result count
    pub fn set_result_count(&mut self, current: usize, total: usize, cx: &mut Context<Self>) {
        self.result_count = if total > 0 {
            Some((current, total))
        } else {
            None
        };
        cx.notify();
    }

    /// Clear result count
    pub fn clear_result_count(&mut self, cx: &mut Context<Self>) {
        self.result_count = None;
        cx.notify();
    }

    /// Focus the input
    pub fn focus(&mut self, cx: &mut Context<Self>) {
        self.focused = true;
        cx.emit(SearchInputEvent::Focus);
        cx.notify();
    }

    /// Handle key down
    fn handle_key_down(
        &mut self,
        event: &KeyDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }

        let key = &event.keystroke.key;

        match key.as_str() {
            "enter" => {
                cx.emit(SearchInputEvent::Submit(self.query.clone()));
            }
            "escape" => {
                if !self.query.is_empty() {
                    self.clear(cx);
                }
            }
            "backspace" => {
                if !self.query.is_empty() {
                    self.query.pop();
                    cx.emit(SearchInputEvent::Changed(self.query.clone()));
                }
            }
            _ => {}
        }

        cx.notify();
    }

    /// Handle text input
    fn handle_input(&mut self, text: &str, _window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        self.query.push_str(text);
        cx.emit(SearchInputEvent::Changed(self.query.clone()));
        cx.notify();
    }
}

impl EventEmitter<SearchInputEvent> for SearchInput {}

impl Focusable for SearchInput {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SearchInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let height = self.size.height();
        let font_size = self.size.font_size();
        let icon_size = self.size.icon_size();

        let is_focused = self.focus_handle.is_focused(_window);
        let has_query = !self.query.is_empty();

        let border_color = if is_focused {
            theme.colors.accent
        } else {
            theme.colors.border
        };

        let opacity = if self.disabled { 0.5 } else { 1.0 };

        div().id("search-input").w_full().opacity(opacity).child(
            div()
                .id("search-input-container")
                .track_focus(&self.focus_handle)
                .h(px(height))
                .w_full()
                .px_3()
                .rounded(px(6.0))
                .border_1()
                .border_color(border_color)
                .bg(theme.colors.surface)
                .flex()
                .items_center()
                .gap_2()
                .when(!self.disabled, |d| {
                    d.hover(|s| s.border_color(theme.colors.accent.opacity(0.5)))
                })
                .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                    this.handle_key_down(event, window, cx);
                }))
                // Search icon or loading spinner
                .child(
                    div()
                        .flex_shrink_0()
                        .text_size(px(icon_size))
                        .text_color(theme.colors.text_muted)
                        .child(if self.loading { "◐" } else { "🔍" }),
                )
                // Input area
                .child(
                    div()
                        .flex_1()
                        .text_size(px(font_size))
                        .when(has_query, |d| {
                            d.text_color(theme.colors.text).child(self.query.clone())
                        })
                        .when(!has_query, |d| {
                            d.text_color(theme.colors.text_muted)
                                .child(self.placeholder.clone())
                        }),
                )
                // Result count
                .when_some(self.result_count, |d, (current, total)| {
                    d.child(
                        div()
                            .flex_shrink_0()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child(format!("{}/{}", current, total)),
                    )
                })
                // Clear button
                .when(self.show_clear && has_query, |d| {
                    d.child(
                        div()
                            .id("search-clear")
                            .flex_shrink_0()
                            .size(px(16.0))
                            .rounded(px(4.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .cursor_pointer()
                            .hover(|s| {
                                s.bg(theme.colors.surface_hover)
                                    .text_color(theme.colors.text)
                            })
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.clear(cx);
                            }))
                            .child("×"),
                    )
                })
                // Shortcut hint
                .when_some(self.shortcut_hint.clone(), |d, hint| {
                    d.when(!has_query && !is_focused, |d| {
                        d.child(
                            div()
                                .flex_shrink_0()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(4.0))
                                .bg(theme.colors.surface_hover)
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .child(hint),
                        )
                    })
                }),
        )
    }
}
