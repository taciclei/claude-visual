//! Textarea public API methods

use super::state::Textarea;
use crate::ui::components::textarea::types::*;
use gpui::*;

impl Textarea {
    /// Set text content
    pub fn set_text(&mut self, text: impl Into<String>, cx: &mut Context<Self>) {
        let text = text.into();
        self.text = if let Some(max) = self.max_length {
            text.chars().take(max).collect()
        } else {
            text
        };
        cx.notify();
    }

    /// Get text content
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Set placeholder
    pub fn set_placeholder(&mut self, placeholder: impl Into<String>, cx: &mut Context<Self>) {
        self.placeholder = placeholder.into();
        cx.notify();
    }

    /// Set rows
    pub fn set_rows(&mut self, rows: usize, cx: &mut Context<Self>) {
        self.rows = rows.max(1);
        cx.notify();
    }

    /// Set min rows
    pub fn set_min_rows(&mut self, min: usize, cx: &mut Context<Self>) {
        self.min_rows = min.max(1);
        cx.notify();
    }

    /// Set max rows
    pub fn set_max_rows(&mut self, max: usize, cx: &mut Context<Self>) {
        self.max_rows = max.max(1);
        cx.notify();
    }

    /// Set resize mode
    pub fn set_resize(&mut self, resize: TextareaResize, cx: &mut Context<Self>) {
        self.resize = resize;
        cx.notify();
    }

    /// Set disabled
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        cx.notify();
    }

    /// Set readonly
    pub fn set_readonly(&mut self, readonly: bool, cx: &mut Context<Self>) {
        self.readonly = readonly;
        cx.notify();
    }

    /// Set label
    pub fn set_label(&mut self, label: Option<String>, cx: &mut Context<Self>) {
        self.label = label;
        cx.notify();
    }

    /// Set helper text
    pub fn set_helper(&mut self, helper: Option<String>, cx: &mut Context<Self>) {
        self.helper = helper;
        cx.notify();
    }

    /// Set error
    pub fn set_error(&mut self, error: Option<String>, cx: &mut Context<Self>) {
        self.error = error;
        cx.notify();
    }

    /// Set max length
    pub fn set_max_length(&mut self, max: Option<usize>, cx: &mut Context<Self>) {
        self.max_length = max;
        if let Some(max_len) = max {
            if self.text.len() > max_len {
                self.text = self.text.chars().take(max_len).collect();
            }
        }
        cx.notify();
    }

    /// Set show count
    pub fn set_show_count(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_count = show;
        cx.notify();
    }

    /// Clear text
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.text.clear();
        cx.emit(TextareaEvent::Changed(String::new()));
        cx.notify();
    }

    /// Focus the textarea
    pub fn focus(&mut self, cx: &mut Context<Self>) {
        self.focused = true;
        cx.emit(TextareaEvent::Focus);
        cx.notify();
    }

    /// Get line count
    pub(super) fn line_count(&self) -> usize {
        self.text.lines().count().max(1)
    }

    /// Calculate visible rows based on content
    pub(super) fn visible_rows(&self) -> usize {
        match self.resize {
            TextareaResize::Auto => self.line_count().clamp(self.min_rows, self.max_rows),
            _ => self.rows,
        }
    }
}
