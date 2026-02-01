//! Core MessageView struct and methods

use gpui::*;
use std::sync::Arc;

use super::types::{MessageAction, MessageReaction, MessageViewEvent};
use super::utils::format_relative_time;
use crate::app::state::AppState;
use crate::claude::message::{ClaudeMessage, MessageRole};

/// Stateful message view component
pub struct MessageView {
    pub(super) message: ClaudeMessage,
    pub(super) app_state: Arc<AppState>,
    pub(super) collapsed: bool,
    /// Whether context menu is visible
    pub(super) show_context_menu: bool,
    /// Position for context menu
    pub(super) context_menu_position: Point<Pixels>,
    /// Current reaction (if any)
    pub(super) reaction: Option<MessageReaction>,
    /// Whether message is bookmarked
    pub(super) bookmarked: bool,
    /// Optional bookmark note
    pub(super) bookmark_note: Option<String>,
    /// Whether this message is selected (for keyboard navigation)
    pub(super) selected: bool,
    /// Whether mouse is hovering over this message
    pub(super) hovered: bool,
}

impl MessageView {
    pub fn new(message: ClaudeMessage, app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            message,
            app_state,
            collapsed: false,
            show_context_menu: false,
            context_menu_position: Point::default(),
            reaction: None,
            bookmarked: false,
            bookmark_note: None,
            selected: false,
            hovered: false,
        }
    }

    /// Set hover state
    pub fn set_hovered(&mut self, hovered: bool, cx: &mut Context<Self>) {
        if self.hovered != hovered {
            self.hovered = hovered;
            cx.notify();
        }
    }

    /// Check if message is hovered
    pub fn is_hovered(&self) -> bool {
        self.hovered
    }

    /// Set selected state
    pub fn set_selected(&mut self, selected: bool, cx: &mut Context<Self>) {
        if self.selected != selected {
            self.selected = selected;
            cx.notify();
        }
    }

    /// Check if message is selected
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    /// Toggle bookmark state
    pub fn toggle_bookmark(&mut self, cx: &mut Context<Self>) {
        self.bookmarked = !self.bookmarked;
        if !self.bookmarked {
            self.bookmark_note = None;
        }
        cx.emit(MessageViewEvent::BookmarkToggled(self.bookmarked));
        cx.notify();
    }

    /// Set bookmark with optional note
    pub fn set_bookmark(&mut self, bookmarked: bool, note: Option<String>, cx: &mut Context<Self>) {
        self.bookmarked = bookmarked;
        self.bookmark_note = if bookmarked { note } else { None };
        cx.emit(MessageViewEvent::BookmarkToggled(self.bookmarked));
        cx.notify();
    }

    /// Check if message is bookmarked
    pub fn is_bookmarked(&self) -> bool {
        self.bookmarked
    }

    /// Get bookmark note
    pub fn bookmark_note(&self) -> Option<&str> {
        self.bookmark_note.as_deref()
    }

    /// Set or toggle a reaction
    pub fn set_reaction(&mut self, reaction: MessageReaction, cx: &mut Context<Self>) {
        if self.reaction == Some(reaction) {
            // Toggle off if same reaction
            self.reaction = None;
        } else {
            self.reaction = Some(reaction);
            cx.emit(MessageViewEvent::Reacted(reaction));
        }
        cx.notify();
    }

    /// Get current reaction
    pub fn reaction(&self) -> Option<MessageReaction> {
        self.reaction
    }

    /// Check if message can have reactions (only assistant messages)
    pub fn can_react(&self) -> bool {
        matches!(self.message.role, MessageRole::Assistant)
    }

    /// Toggle collapsed state
    pub fn toggle_collapsed(&mut self, cx: &mut Context<Self>) {
        self.collapsed = !self.collapsed;
        cx.notify();
    }

    /// Set collapsed state
    pub fn set_collapsed(&mut self, collapsed: bool, cx: &mut Context<Self>) {
        if self.collapsed != collapsed {
            self.collapsed = collapsed;
            cx.notify();
        }
    }

    /// Check if message is collapsed
    pub fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    /// Estimate token count for this message
    /// Rough approximation: ~1.3 tokens per word for English text
    pub(super) fn estimate_tokens(&self) -> usize {
        let word_count = self.message.content.split_whitespace().count();
        (word_count as f64 * 1.3).ceil() as usize
    }

    /// Estimate cost for this message based on tokens
    /// Prices (per 1M tokens): Input $3, Output $15 (Claude 3.5 Sonnet approx)
    pub(super) fn estimate_cost(&self) -> f64 {
        let tokens = self.estimate_tokens() as f64;
        match self.message.role {
            MessageRole::User => tokens * 3.0 / 1_000_000.0, // Input pricing
            MessageRole::Assistant => tokens * 15.0 / 1_000_000.0, // Output pricing
            _ => 0.0,
        }
    }

    /// Format token count with K suffix
    pub(super) fn format_tokens(tokens: usize) -> String {
        if tokens >= 1_000 {
            format!("{:.1}K", tokens as f64 / 1_000.0)
        } else {
            tokens.to_string()
        }
    }

    /// Show context menu at position
    pub fn show_context_menu(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        self.show_context_menu = true;
        self.context_menu_position = position;
        cx.notify();
    }

    /// Hide context menu
    pub fn hide_context_menu(&mut self, cx: &mut Context<Self>) {
        self.show_context_menu = false;
        cx.notify();
    }

    /// Execute a context menu action
    pub fn execute_action(&mut self, action: MessageAction, cx: &mut Context<Self>) {
        self.hide_context_menu(cx);

        match action {
            MessageAction::Copy => {
                self.copy_to_clipboard(cx);
            }
            MessageAction::CopyAsMarkdown => {
                self.copy_as_markdown(cx);
            }
            MessageAction::Regenerate => {
                cx.emit(MessageViewEvent::RegenerateResponse);
            }
            MessageAction::Edit => {
                cx.emit(MessageViewEvent::Edit(self.message.content.clone()));
            }
            MessageAction::Quote => {
                cx.emit(MessageViewEvent::Quote(self.message.content.clone()));
            }
            MessageAction::RetryFromHere => {
                cx.emit(MessageViewEvent::RetryFromHere);
            }
            MessageAction::Bookmark => {
                self.toggle_bookmark(cx);
            }
            MessageAction::Delete => {
                cx.emit(MessageViewEvent::Delete);
            }
        }
    }

    /// Copy message content to clipboard
    pub fn copy_to_clipboard(&self, cx: &mut Context<Self>) {
        cx.write_to_clipboard(ClipboardItem::new_string(self.message.content.clone()));
        tracing::info!("Message copied to clipboard");
    }

    /// Copy message as Markdown format
    pub fn copy_as_markdown(&self, cx: &mut Context<Self>) {
        let role_prefix = match self.message.role {
            MessageRole::User => "**You:**",
            MessageRole::Assistant => "**Claude:**",
            MessageRole::ToolUse => "**Tool:**",
            MessageRole::ToolResult => "**Result:**",
            MessageRole::Error => "**Error:**",
            MessageRole::Thinking => "**💭 Thinking:**",
            MessageRole::System => "**System:**",
        };
        let markdown = format!("{}\n\n{}", role_prefix, self.message.content);
        cx.write_to_clipboard(ClipboardItem::new_string(markdown));
        tracing::info!("Message copied as Markdown");
    }

    /// Get available actions for current message
    pub(super) fn available_actions(&self) -> Vec<MessageAction> {
        let all_actions = [
            MessageAction::Copy,
            MessageAction::CopyAsMarkdown,
            MessageAction::Regenerate,
            MessageAction::Edit,
            MessageAction::Quote,
            MessageAction::RetryFromHere,
            MessageAction::Bookmark,
            MessageAction::Delete,
        ];

        all_actions
            .into_iter()
            .filter(|action| action.available_for_role(self.message.role))
            .collect()
    }

    /// Get message role label
    pub(super) fn role_label(&self) -> &'static str {
        match self.message.role {
            MessageRole::User => "You",
            MessageRole::Assistant => "Claude",
            MessageRole::ToolUse => "Tool",
            MessageRole::ToolResult => "Result",
            MessageRole::Error => "Error",
            MessageRole::Thinking => "💭 Thinking",
            MessageRole::System => "System",
        }
    }

    /// Get timestamp formatted (relative for recent, absolute for older)
    pub(super) fn formatted_time(&self) -> String {
        format_relative_time(self.message.timestamp)
    }

    /// Get full timestamp (always absolute)
    #[allow(dead_code)]
    pub(super) fn full_timestamp(&self) -> String {
        self.message
            .timestamp
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    }

    /// Count code blocks in message content
    pub(super) fn code_block_count(&self) -> usize {
        self.message.content.matches("```").count() / 2
    }

    /// Check if message has code blocks
    pub(super) fn has_code_blocks(&self) -> bool {
        self.code_block_count() > 0
    }
}
