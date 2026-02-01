//! Search functionality for code blocks

use gpui::*;

use super::super::types::SearchMatch;
use super::CodeBlockView;

impl CodeBlockView {
    /// Toggle search visibility
    pub fn toggle_search(&mut self, cx: &mut Context<Self>) {
        self.search_visible = !self.search_visible;
        if !self.search_visible {
            self.search_query.clear();
            self.search_matches.clear();
            self.current_match_index = None;
        }
        cx.notify();
    }

    /// Show search
    pub fn show_search(&mut self, cx: &mut Context<Self>) {
        self.search_visible = true;
        cx.notify();
    }

    /// Hide search
    pub fn hide_search(&mut self, cx: &mut Context<Self>) {
        self.search_visible = false;
        self.search_query.clear();
        self.search_matches.clear();
        self.current_match_index = None;
        cx.notify();
    }

    /// Update search query and find matches
    pub fn set_search_query(&mut self, query: String, cx: &mut Context<Self>) {
        self.search_query = query;
        self.find_matches();
        if !self.search_matches.is_empty() {
            self.current_match_index = Some(0);
        } else {
            self.current_match_index = None;
        }
        cx.notify();
    }

    /// Find all matches in the code
    fn find_matches(&mut self) {
        self.search_matches.clear();

        if self.search_query.is_empty() {
            return;
        }

        let query_lower = self.search_query.to_lowercase();
        for (line_idx, line) in self.code.lines().enumerate() {
            let line_lower = line.to_lowercase();
            let mut search_start = 0;

            while let Some(pos) = line_lower[search_start..].find(&query_lower) {
                let start = search_start + pos;
                let end = start + self.search_query.len();
                self.search_matches.push(SearchMatch {
                    line: line_idx,
                    start,
                    end,
                });
                search_start = end;
            }
        }
    }

    /// Navigate to next match
    pub fn next_match(&mut self, cx: &mut Context<Self>) {
        if self.search_matches.is_empty() {
            return;
        }

        self.current_match_index = Some(match self.current_match_index {
            Some(idx) => (idx + 1) % self.search_matches.len(),
            None => 0,
        });
        cx.notify();
    }

    /// Navigate to previous match
    pub fn prev_match(&mut self, cx: &mut Context<Self>) {
        if self.search_matches.is_empty() {
            return;
        }

        self.current_match_index = Some(match self.current_match_index {
            Some(idx) => {
                if idx == 0 {
                    self.search_matches.len() - 1
                } else {
                    idx - 1
                }
            }
            None => self.search_matches.len() - 1,
        });
        cx.notify();
    }

    /// Get matches for a specific line
    pub(crate) fn get_line_matches(&self, line_idx: usize) -> Vec<&SearchMatch> {
        self.search_matches
            .iter()
            .filter(|m| m.line == line_idx)
            .collect()
    }

    /// Check if a match is the current match
    pub(crate) fn is_current_match(&self, line_idx: usize, start: usize) -> bool {
        if let Some(current_idx) = self.current_match_index {
            if let Some(current_match) = self.search_matches.get(current_idx) {
                return current_match.line == line_idx && current_match.start == start;
            }
        }
        false
    }
}
