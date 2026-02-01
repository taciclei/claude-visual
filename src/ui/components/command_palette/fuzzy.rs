//! Fuzzy search for command palette

use super::types::Command;

/// Result of fuzzy matching a command
#[derive(Clone)]
pub struct FuzzyMatch<'a> {
    pub command: &'a Command,
    pub score: i32,
    /// Indices of matched characters in the label
    pub matched_indices: Vec<usize>,
}

impl<'a> FuzzyMatch<'a> {
    /// Perform fuzzy match of query against command
    pub fn from_query(command: &'a Command, query: &str) -> Option<Self> {
        if query.is_empty() {
            return Some(Self {
                command,
                score: 0,
                matched_indices: vec![],
            });
        }

        let label_lower = command.label.to_lowercase();
        let category_lower = command.category.to_lowercase();
        let query_lower = query.to_lowercase();
        let query_chars: Vec<char> = query_lower.chars().collect();

        // Try to match in label first (primary)
        if let Some((score, indices)) = fuzzy_score(&label_lower, &query_chars) {
            return Some(Self {
                command,
                score: score + 100, // Bonus for label match
                matched_indices: indices,
            });
        }

        // Try category match (secondary)
        if category_lower.contains(&query_lower) {
            return Some(Self {
                command,
                score: 50,
                matched_indices: vec![],
            });
        }

        // Try ID match (tertiary)
        if command.id.to_lowercase().contains(&query_lower) {
            return Some(Self {
                command,
                score: 25,
                matched_indices: vec![],
            });
        }

        None
    }
}

/// Calculate fuzzy score and matched indices
/// Returns (score, matched_indices) if query matches, None otherwise
fn fuzzy_score(text: &str, query_chars: &[char]) -> Option<(i32, Vec<usize>)> {
    if query_chars.is_empty() {
        return Some((0, vec![]));
    }

    let text_chars: Vec<char> = text.chars().collect();
    let mut query_idx = 0;
    let mut score: i32 = 0;
    let mut matched_indices = Vec::with_capacity(query_chars.len());
    let mut prev_match_idx: Option<usize> = None;

    for (i, &c) in text_chars.iter().enumerate() {
        if query_idx < query_chars.len() && c == query_chars[query_idx] {
            matched_indices.push(i);

            // Scoring bonuses
            if i == 0 {
                score += 15; // Start of string bonus
            } else if text_chars.get(i.saturating_sub(1)).map(|c| *c == ' ' || *c == '_' || *c == '-').unwrap_or(false) {
                score += 10; // Start of word bonus
            }

            // Consecutive match bonus
            if let Some(prev) = prev_match_idx {
                if i == prev + 1 {
                    score += 5; // Consecutive characters
                }
            }

            prev_match_idx = Some(i);
            query_idx += 1;
        }
    }

    if query_idx == query_chars.len() {
        // All characters matched
        // Shorter matches get higher scores (exact match bonus)
        let length_penalty = (text_chars.len() - query_chars.len()) as i32;
        score = score.saturating_sub(length_penalty);

        Some((score.max(1), matched_indices))
    } else {
        None // Not all query characters were found
    }
}

/// Search commands with fuzzy matching
pub fn fuzzy_search<'a>(
    commands: &'a [Command],
    query: &str,
    recent_commands: &[String],
) -> Vec<FuzzyMatch<'a>> {
    let mut matches: Vec<FuzzyMatch<'a>> = commands
        .iter()
        .filter_map(|cmd| FuzzyMatch::from_query(cmd, query))
        .collect();

    // Boost score for recent commands
    for m in &mut matches {
        if let Some(pos) = recent_commands.iter().position(|id| id == m.command.id) {
            // More recent = higher boost
            m.score += (50 - pos as i32 * 5).max(10);
        }
    }

    // Sort by score descending
    matches.sort_by(|a, b| b.score.cmp(&a.score));

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_score() {
        let query: Vec<char> = "nc".chars().collect();
        let result = fuzzy_score("new conversation", &query);
        assert!(result.is_some());
        let (score, indices) = result.unwrap();
        assert!(score > 0);
        assert_eq!(indices, vec![0, 4]); // 'n' at 0, 'c' at 4
    }

    #[test]
    fn test_fuzzy_match_exact() {
        let cmd = Command::new("test", "New Conversation", None, "Chat");
        let m = FuzzyMatch::from_query(&cmd, "new conv");
        assert!(m.is_some());
        let m = m.unwrap();
        assert!(m.score > 100); // Should have label bonus
    }

    #[test]
    fn test_fuzzy_match_category() {
        let cmd = Command::new("test", "Something Else", None, "Chat");
        let m = FuzzyMatch::from_query(&cmd, "chat");
        assert!(m.is_some());
        assert_eq!(m.unwrap().score, 50);
    }

    #[test]
    fn test_no_match() {
        let cmd = Command::new("test", "New Conversation", None, "Chat");
        let m = FuzzyMatch::from_query(&cmd, "xyz");
        assert!(m.is_none());
    }
}
