//! Utility functions for completion

/// Simple fuzzy matching (returns score if matches)
pub(crate) fn fuzzy_match(text: &str, query: &str) -> Option<i32> {
    if query.is_empty() {
        return Some(0);
    }

    let text_lower = text.to_lowercase();
    let query_lower = query.to_lowercase();

    // Exact prefix match
    if text_lower.starts_with(&query_lower) {
        return Some(100 + (query.len() as i32 * 2));
    }

    // Contains match
    if text_lower.contains(&query_lower) {
        return Some(50 + query.len() as i32);
    }

    // Fuzzy match (all query chars in order)
    let mut query_chars = query_lower.chars().peekable();
    let mut score = 0;
    let mut consecutive = 0;

    for ch in text_lower.chars() {
        if query_chars.peek() == Some(&ch) {
            query_chars.next();
            score += 10 + consecutive * 2;
            consecutive += 1;
        } else {
            consecutive = 0;
        }
    }

    if query_chars.peek().is_none() {
        Some(score)
    } else {
        None
    }
}
