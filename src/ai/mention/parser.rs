//! Mention parsing logic

use super::types::{Mention, MentionKind, PartialMention, PartialMentionKind};
use super::utils::{
    find_identifier_end, find_path_end, find_url_end, find_word_end, parse_line_range,
};
use std::path::PathBuf;

/// Parse mentions from input text
pub fn parse_mentions(text: &str) -> Vec<Mention> {
    let mut mentions = Vec::new();
    let mut chars = text.char_indices().peekable();

    while let Some((i, c)) = chars.next() {
        if c == '@' {
            // Check if this is the start of a mention
            if let Some(mention) = parse_mention_at(text, i) {
                mentions.push(mention);
            }
        }
    }

    mentions
}

/// Parse a single mention starting at the given position
fn parse_mention_at(text: &str, start: usize) -> Option<Mention> {
    let remaining = &text[start..];

    // Skip the @
    let after_at = &remaining[1..];

    // Check for explicit prefixes
    if after_at.starts_with("file:") {
        return parse_file_mention(text, start, 6); // @file:
    }
    if after_at.starts_with("snippet:") {
        return parse_snippet_mention(text, start, 9); // @snippet:
    }
    if after_at.starts_with("url:") {
        return parse_url_mention(text, start, 5); // @url:
    }
    if after_at.starts_with("symbol:") {
        return parse_symbol_mention(text, start, 8); // @symbol:
    }

    // Check for implicit file path (starts with / or ./ or ../ or looks like a file)
    if after_at.starts_with('/') || after_at.starts_with("./") || after_at.starts_with("../") {
        return parse_file_mention(text, start, 1); // Just after @
    }

    // Check if it looks like a file path (contains / and has extension)
    let word_end = find_word_end(after_at);
    let word = &after_at[..word_end];
    if word.contains('/') || (word.contains('.') && !word.starts_with('.')) {
        return parse_file_mention(text, start, 1);
    }

    None
}

/// Parse a file mention
fn parse_file_mention(text: &str, start: usize, prefix_len: usize) -> Option<Mention> {
    let path_start = start + prefix_len;
    let remaining = &text[path_start..];

    // Find the end of the path
    let path_end = find_path_end(remaining);
    if path_end == 0 {
        return None;
    }

    let path_str = &remaining[..path_end];

    // Check for line range (e.g., :10-20 or :10)
    if let Some(colon_pos) = path_str.rfind(':') {
        let file_part = &path_str[..colon_pos];
        let range_part = &path_str[colon_pos + 1..];

        if let Some((start_line, end_line)) = parse_line_range(range_part) {
            let raw = text[start..path_start + path_end].to_string();
            return Some(Mention {
                kind: MentionKind::FileRange {
                    path: PathBuf::from(file_part),
                    start_line,
                    end_line,
                },
                start,
                end: path_start + path_end,
                raw,
            });
        }
    }

    let raw = text[start..path_start + path_end].to_string();
    Some(Mention {
        kind: MentionKind::File(PathBuf::from(path_str)),
        start,
        end: path_start + path_end,
        raw,
    })
}

/// Parse a snippet mention
fn parse_snippet_mention(text: &str, start: usize, prefix_len: usize) -> Option<Mention> {
    let name_start = start + prefix_len;
    let remaining = &text[name_start..];

    let name_end = find_word_end(remaining);
    if name_end == 0 {
        return None;
    }

    let name = remaining[..name_end].to_string();
    let raw = text[start..name_start + name_end].to_string();

    Some(Mention {
        kind: MentionKind::Snippet(name),
        start,
        end: name_start + name_end,
        raw,
    })
}

/// Parse a URL mention
fn parse_url_mention(text: &str, start: usize, prefix_len: usize) -> Option<Mention> {
    let url_start = start + prefix_len;
    let remaining = &text[url_start..];

    let url_end = find_url_end(remaining);
    if url_end == 0 {
        return None;
    }

    let url = remaining[..url_end].to_string();
    let raw = text[start..url_start + url_end].to_string();

    Some(Mention {
        kind: MentionKind::Url(url),
        start,
        end: url_start + url_end,
        raw,
    })
}

/// Parse a symbol mention
fn parse_symbol_mention(text: &str, start: usize, prefix_len: usize) -> Option<Mention> {
    let name_start = start + prefix_len;
    let remaining = &text[name_start..];

    let name_end = find_identifier_end(remaining);
    if name_end == 0 {
        return None;
    }

    let name = remaining[..name_end].to_string();
    let raw = text[start..name_start + name_end].to_string();

    Some(Mention {
        kind: MentionKind::Symbol(name),
        start,
        end: name_start + name_end,
        raw,
    })
}

/// Check if a mention is being typed at the cursor position
pub fn get_mention_at_cursor(text: &str, cursor: usize) -> Option<PartialMention> {
    // Find the start of the current word
    let before_cursor = &text[..cursor];
    let word_start = before_cursor
        .rfind(|c: char| c.is_whitespace())
        .map(|i| i + 1)
        .unwrap_or(0);

    let current_word = &text[word_start..cursor];

    if current_word.starts_with('@') {
        let prefix = &current_word[1..];

        // Determine the mention type being typed
        let kind = if prefix.starts_with("file:") {
            Some(PartialMentionKind::File)
        } else if prefix.starts_with("snippet:") {
            Some(PartialMentionKind::Snippet)
        } else if prefix.starts_with("url:") {
            Some(PartialMentionKind::Url)
        } else if prefix.starts_with("symbol:") {
            Some(PartialMentionKind::Symbol)
        } else if prefix.contains('/') || prefix.starts_with('.') {
            Some(PartialMentionKind::File)
        } else {
            Some(PartialMentionKind::Unknown)
        };

        kind.map(|k| PartialMention {
            kind: k,
            start: word_start,
            prefix: prefix.to_string(),
        })
    } else {
        None
    }
}
