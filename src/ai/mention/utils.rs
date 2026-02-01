//! Utility functions for mention parsing

/// Find the end of a word (stops at whitespace or certain punctuation)
pub fn find_word_end(text: &str) -> usize {
    text.char_indices()
        .find(|(_, c)| c.is_whitespace() || matches!(*c, ',' | ';' | ')' | ']' | '}' | '"' | '\''))
        .map(|(i, _)| i)
        .unwrap_or(text.len())
}

/// Find the end of a file path
pub fn find_path_end(text: &str) -> usize {
    text.char_indices()
        .find(|(_, c)| {
            c.is_whitespace() || matches!(*c, ',' | ';' | ')' | ']' | '}' | '"' | '\'' | '>' | '|')
        })
        .map(|(i, _)| i)
        .unwrap_or(text.len())
}

/// Find the end of a URL
pub fn find_url_end(text: &str) -> usize {
    text.char_indices()
        .find(|(_, c)| c.is_whitespace() || matches!(*c, '>' | '"' | '\'' | ')' | ']'))
        .map(|(i, _)| i)
        .unwrap_or(text.len())
}

/// Find the end of an identifier (word characters only)
pub fn find_identifier_end(text: &str) -> usize {
    text.char_indices()
        .find(|(_, c)| !c.is_alphanumeric() && *c != '_' && *c != ':')
        .map(|(i, _)| i)
        .unwrap_or(text.len())
}

/// Parse a line range like "10" or "10-20"
pub fn parse_line_range(text: &str) -> Option<(usize, Option<usize>)> {
    if text.is_empty() {
        return None;
    }

    if let Some(dash_pos) = text.find('-') {
        let start = text[..dash_pos].parse().ok()?;
        let end = text[dash_pos + 1..].parse().ok()?;
        Some((start, Some(end)))
    } else {
        let line = text.parse().ok()?;
        Some((line, None))
    }
}
