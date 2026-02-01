//! Core search logic implementation for ChatView

use super::core::ChatView;
use super::types::ConversationSearchResult;

impl ChatView {
    /// Perform search across all messages with filters
    /// This is the core search algorithm that builds the results list
    pub(crate) fn perform_search(&mut self) {
        self.search.results.clear();

        if self.search.query.is_empty() {
            return;
        }

        // Build regex pattern if regex mode is enabled
        let regex_pattern = if self.search.regex {
            regex::Regex::new(&self.search.query).ok()
        } else {
            None
        };

        for (msg_idx, message) in self.messages.iter().enumerate() {
            // Skip messages that don't match the role filter
            if !self.search.role_filter.includes_role(message.role) {
                continue;
            }

            for (line_num, line) in message.content.lines().enumerate() {
                if let Some(ref re) = regex_pattern {
                    // Regex search
                    for mat in re.find_iter(line) {
                        let start = mat.start();
                        let end = mat.end();

                        // Create snippet with context
                        let snippet_start = start.saturating_sub(30);
                        let snippet_end = (end + 30).min(line.len());
                        let mut snippet = line[snippet_start..snippet_end].to_string();
                        if snippet_start > 0 {
                            snippet = format!("...{}", snippet);
                        }
                        if snippet_end < line.len() {
                            snippet = format!("{}...", snippet);
                        }

                        self.search.results.push(ConversationSearchResult {
                            message_index: msg_idx,
                            line_number: line_num,
                            start,
                            end,
                            snippet,
                            role: message.role,
                        });
                    }
                } else {
                    // Plain text search (with optional case sensitivity)
                    let (search_line, search_query) = if self.search.case_sensitive {
                        (line.to_string(), self.search.query.clone())
                    } else {
                        (line.to_lowercase(), self.search.query.to_lowercase())
                    };

                    let mut search_start = 0;
                    while let Some(pos) = search_line[search_start..].find(&search_query) {
                        let start = search_start + pos;
                        let end = start + self.search.query.len();

                        // Create snippet with context
                        let snippet_start = start.saturating_sub(30);
                        let snippet_end = (end + 30).min(line.len());
                        let mut snippet = line[snippet_start..snippet_end].to_string();
                        if snippet_start > 0 {
                            snippet = format!("...{}", snippet);
                        }
                        if snippet_end < line.len() {
                            snippet = format!("{}...", snippet);
                        }

                        self.search.results.push(ConversationSearchResult {
                            message_index: msg_idx,
                            line_number: line_num,
                            start,
                            end,
                            snippet,
                            role: message.role,
                        });

                        search_start = start + 1;
                    }
                }
            }
        }
    }
}
