//! Diff parsing logic

use super::types::{DiffHunk, DiffLine};

/// Parse unified diff text into hunks with line numbers
pub(super) fn parse_diff(diff_text: &str) -> (Vec<DiffHunk>, usize, usize) {
    let mut hunks = Vec::new();
    let mut current_hunk: Option<DiffHunk> = None;
    let mut additions = 0;
    let mut deletions = 0;
    let mut old_line = 0;
    let mut new_line = 0;

    for line in diff_text.lines() {
        if line.starts_with("@@") {
            // Save previous hunk if exists
            if let Some(hunk) = current_hunk.take() {
                hunks.push(hunk);
            }

            // Parse hunk header: @@ -old_start,old_count +new_start,new_count @@
            let (old_start, old_count, new_start, new_count) = parse_hunk_header(line);
            old_line = old_start;
            new_line = new_start;

            current_hunk = Some(DiffHunk {
                header: line.to_string(),
                lines: vec![DiffLine::HunkHeader {
                    header: line.to_string(),
                    old_start,
                    old_count,
                    new_start,
                    new_count,
                }],
                collapsed: false,
            });
        } else if let Some(ref mut hunk) = current_hunk {
            if line.starts_with('+') && !line.starts_with("+++") {
                hunk.lines.push(DiffLine::Added {
                    content: line[1..].to_string(),
                    new_line,
                });
                new_line += 1;
                additions += 1;
            } else if line.starts_with('-') && !line.starts_with("---") {
                hunk.lines.push(DiffLine::Removed {
                    content: line[1..].to_string(),
                    old_line,
                });
                old_line += 1;
                deletions += 1;
            } else if line.starts_with(' ') || line.is_empty() {
                let content = if line.is_empty() {
                    String::new()
                } else {
                    line[1..].to_string()
                };
                hunk.lines.push(DiffLine::Context {
                    content,
                    old_line,
                    new_line,
                });
                old_line += 1;
                new_line += 1;
            }
        }
    }

    // Don't forget the last hunk
    if let Some(hunk) = current_hunk {
        hunks.push(hunk);
    }

    (hunks, additions, deletions)
}

/// Parse hunk header to extract line numbers
fn parse_hunk_header(header: &str) -> (usize, usize, usize, usize) {
    // Format: @@ -old_start,old_count +new_start,new_count @@ optional context
    let parts: Vec<&str> = header.split_whitespace().collect();

    let mut old_start = 1;
    let mut old_count = 0;
    let mut new_start = 1;
    let mut new_count = 0;

    if parts.len() >= 3 {
        // Parse old range
        if let Some(old_range) = parts.get(1) {
            let old_range = old_range.trim_start_matches('-');
            let old_parts: Vec<&str> = old_range.split(',').collect();
            old_start = old_parts.first().and_then(|s| s.parse().ok()).unwrap_or(1);
            old_count = old_parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
        }
        // Parse new range
        if let Some(new_range) = parts.get(2) {
            let new_range = new_range.trim_start_matches('+');
            let new_parts: Vec<&str> = new_range.split(',').collect();
            new_start = new_parts.first().and_then(|s| s.parse().ok()).unwrap_or(1);
            new_count = new_parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
        }
    }

    (old_start, old_count, new_start, new_count)
}
