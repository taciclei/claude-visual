//! Mention handling

use gpui::prelude::*;
use gpui::*;

use super::ChatInput;
use crate::ai::mention::{get_mention_at_cursor, parse_mentions, Mention, MentionKind};

impl ChatInput {
    /// Update parsed mentions from current text
    pub(super) fn update_mentions(&mut self) {
        self.mentions = parse_mentions(&self.text);
        self.partial_mention = get_mention_at_cursor(&self.text, self.cursor_position);
    }

    /// Update mentions and file autocomplete (requires context)
    pub(super) fn update_mentions_with_autocomplete(&mut self, cx: &mut Context<Self>) {
        self.mentions = parse_mentions(&self.text);
        self.partial_mention = get_mention_at_cursor(&self.text, self.cursor_position);
        self.update_file_autocomplete(cx);
    }

    /// Get the current mentions
    pub fn mentions(&self) -> &[Mention] {
        &self.mentions
    }

    /// Get file paths from mentions
    pub fn file_mentions(&self) -> Vec<std::path::PathBuf> {
        self.mentions
            .iter()
            .filter_map(|m| match &m.kind {
                MentionKind::File(path) => Some(path.clone()),
                MentionKind::FileRange { path, .. } => Some(path.clone()),
                _ => None,
            })
            .collect()
    }

    /// Remove a file mention by path
    pub fn remove_file_mention(&mut self, path: &std::path::Path, cx: &mut Context<Self>) {
        // Find the mention to remove
        if let Some(mention) = self.mentions.iter().find(|m| match &m.kind {
            MentionKind::File(p) => p == path,
            MentionKind::FileRange { path: p, .. } => p == path,
            _ => false,
        }) {
            // Remove the mention text from the input
            let start = mention.start;
            let end = mention.end;

            // Also remove trailing whitespace if present
            let end_with_space = if self.text.get(end..end + 1) == Some(" ") {
                end + 1
            } else {
                end
            };

            self.text = format!("{}{}", &self.text[..start], &self.text[end_with_space..]);

            // Adjust cursor if needed
            if self.cursor_position > start {
                self.cursor_position = self.cursor_position.saturating_sub(end_with_space - start);
            }

            // Re-parse mentions
            self.update_mentions();
            cx.notify();
        }
    }

    /// Render text with highlighted mentions
    pub(super) fn render_text_with_mentions(&self, theme: &crate::app::theme::Theme) -> Div {
        if self.mentions.is_empty() {
            return div()
                .text_sm()
                .text_color(theme.colors.text)
                .whitespace_nowrap()
                .child(self.text.clone());
        }

        // Sort mentions by position
        let mut mentions = self.mentions.clone();
        mentions.sort_by_key(|m| m.start);

        let mut elements: Vec<AnyElement> = Vec::new();
        let mut last_end = 0;

        for mention in &mentions {
            // Add text before mention
            if mention.start > last_end {
                let text_before = &self.text[last_end..mention.start];
                elements.push(
                    div()
                        .text_sm()
                        .text_color(theme.colors.text)
                        .child(text_before.to_string())
                        .into_any_element(),
                );
            }

            // Add highlighted mention
            let mention_color = match &mention.kind {
                MentionKind::File(_) | MentionKind::FileRange { .. } => theme.colors.accent,
                MentionKind::Snippet(_) => theme.colors.success,
                MentionKind::Url(_) => theme.colors.info,
                MentionKind::Symbol(_) => theme.colors.warning,
            };

            elements.push(
                div()
                    .text_sm()
                    .text_color(mention_color)
                    .font_weight(FontWeight::MEDIUM)
                    .bg(mention_color.opacity(0.15))
                    .px_1()
                    .rounded_sm()
                    .child(mention.raw.clone())
                    .into_any_element(),
            );

            last_end = mention.end;
        }

        // Add text after last mention
        if last_end < self.text.len() {
            let text_after = &self.text[last_end..];
            elements.push(
                div()
                    .text_sm()
                    .text_color(theme.colors.text)
                    .child(text_after.to_string())
                    .into_any_element(),
            );
        }

        div()
            .flex()
            .flex_wrap()
            .whitespace_nowrap()
            .children(elements)
    }
}
