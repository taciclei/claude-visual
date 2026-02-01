//! Completion dropdown implementation

use super::types::{CompletionDropdownEvent, SimpleColors};
use crate::lsp::protocol::{CompletionItem, CompletionItemKind};
use gpui::*;

/// Completion dropdown for displaying LSP autocomplete suggestions
pub struct CompletionDropdown {
    /// Available completion items
    pub(super) items: Vec<CompletionItem>,
    /// Currently selected index
    pub(super) selected_index: usize,
    /// Position on screen (x, y)
    pub(super) position: Point<Pixels>,
    /// Whether the dropdown is visible
    pub(super) is_visible: bool,
    /// Filter text for highlighting
    pub(super) filter_text: String,
    /// Maximum visible items
    pub(super) max_visible_items: usize,
    /// Scroll offset
    pub(super) scroll_offset: usize,
}

impl CompletionDropdown {
    /// Create a new completion dropdown
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            items: Vec::new(),
            selected_index: 0,
            position: point(px(0.0), px(0.0)),
            is_visible: false,
            filter_text: String::new(),
            max_visible_items: 10,
            scroll_offset: 0,
        }
    }

    /// Show completions at a position
    pub fn show(
        &mut self,
        items: Vec<CompletionItem>,
        position: Point<Pixels>,
        filter_text: String,
        cx: &mut Context<Self>,
    ) {
        self.items = items;
        self.position = position;
        self.filter_text = filter_text;
        self.selected_index = 0;
        self.scroll_offset = 0;
        self.is_visible = !self.items.is_empty();
        cx.notify();
    }

    /// Hide the dropdown
    pub fn hide(&mut self, cx: &mut Context<Self>) {
        self.is_visible = false;
        self.items.clear();
        cx.emit(CompletionDropdownEvent::Closed);
        cx.notify();
    }

    /// Check if the dropdown is visible
    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    /// Select the next item
    pub fn select_next(&mut self, cx: &mut Context<Self>) {
        if self.items.is_empty() {
            return;
        }
        self.selected_index = (self.selected_index + 1) % self.items.len();
        self.ensure_selected_visible();
        cx.notify();
    }

    /// Select the previous item
    pub fn select_prev(&mut self, cx: &mut Context<Self>) {
        if self.items.is_empty() {
            return;
        }
        self.selected_index = if self.selected_index == 0 {
            self.items.len() - 1
        } else {
            self.selected_index - 1
        };
        self.ensure_selected_visible();
        cx.notify();
    }

    /// Confirm the current selection
    pub fn confirm(&mut self, cx: &mut Context<Self>) {
        if let Some(item) = self.items.get(self.selected_index).cloned() {
            cx.emit(CompletionDropdownEvent::Selected(item));
            self.hide(cx);
        }
    }

    /// Ensure the selected item is visible
    pub(super) fn ensure_selected_visible(&mut self) {
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        } else if self.selected_index >= self.scroll_offset + self.max_visible_items {
            self.scroll_offset = self.selected_index - self.max_visible_items + 1;
        }
    }

    /// Get icon for completion item kind
    pub(super) fn kind_icon(kind: Option<CompletionItemKind>) -> &'static str {
        match kind {
            Some(CompletionItemKind::Text) => "T",
            Some(CompletionItemKind::Method) => "m",
            Some(CompletionItemKind::Function) => "f",
            Some(CompletionItemKind::Constructor) => "C",
            Some(CompletionItemKind::Field) => "F",
            Some(CompletionItemKind::Variable) => "v",
            Some(CompletionItemKind::Class) => "c",
            Some(CompletionItemKind::Interface) => "i",
            Some(CompletionItemKind::Module) => "M",
            Some(CompletionItemKind::Property) => "p",
            Some(CompletionItemKind::Unit) => "u",
            Some(CompletionItemKind::Value) => "V",
            Some(CompletionItemKind::Enum) => "e",
            Some(CompletionItemKind::Keyword) => "k",
            Some(CompletionItemKind::Snippet) => "s",
            Some(CompletionItemKind::Color) => "#",
            Some(CompletionItemKind::File) => "📄",
            Some(CompletionItemKind::Reference) => "&",
            Some(CompletionItemKind::Folder) => "📁",
            Some(CompletionItemKind::EnumMember) => "E",
            Some(CompletionItemKind::Constant) => "K",
            Some(CompletionItemKind::Struct) => "S",
            Some(CompletionItemKind::Event) => "!",
            Some(CompletionItemKind::Operator) => "±",
            Some(CompletionItemKind::TypeParameter) => "<>",
            None => "?",
        }
    }

    /// Get color for completion item kind
    pub(super) fn kind_color(kind: Option<CompletionItemKind>, colors: &SimpleColors) -> Hsla {
        match kind {
            Some(CompletionItemKind::Function) | Some(CompletionItemKind::Method) => {
                colors.syntax.function
            }
            Some(CompletionItemKind::Variable) | Some(CompletionItemKind::Field) => {
                colors.syntax.variable
            }
            Some(CompletionItemKind::Class)
            | Some(CompletionItemKind::Struct)
            | Some(CompletionItemKind::Interface) => colors.syntax.type_name,
            Some(CompletionItemKind::Keyword) => colors.syntax.keyword,
            Some(CompletionItemKind::Constant) | Some(CompletionItemKind::EnumMember) => {
                colors.syntax.constant
            }
            Some(CompletionItemKind::Module) => colors.syntax.type_name,
            _ => colors.text_muted,
        }
    }
}

impl EventEmitter<CompletionDropdownEvent> for CompletionDropdown {}
