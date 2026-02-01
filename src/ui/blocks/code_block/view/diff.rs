//! Diff computation and rendering helpers

use gpui::*;

use super::super::types::{DiffLine, LineChangeType};
use super::CodeBlockView;

impl CodeBlockView {
    /// Compute diff between old and new code
    pub(crate) fn compute_diff(old_code: &str, new_code: &str) -> Vec<DiffLine> {
        let old_lines: Vec<&str> = old_code.lines().collect();
        let new_lines: Vec<&str> = new_code.lines().collect();

        // Simple line-by-line diff (could be improved with LCS algorithm)
        let mut diff_lines = Vec::new();
        let mut old_idx = 0;
        let mut new_idx = 0;

        while old_idx < old_lines.len() || new_idx < new_lines.len() {
            if old_idx < old_lines.len() && new_idx < new_lines.len() {
                if old_lines[old_idx] == new_lines[new_idx] {
                    // Context (unchanged) line
                    diff_lines.push(DiffLine {
                        content: old_lines[old_idx].to_string(),
                        change_type: LineChangeType::Context,
                        old_line_num: Some(old_idx + 1),
                        new_line_num: Some(new_idx + 1),
                    });
                    old_idx += 1;
                    new_idx += 1;
                } else {
                    // Lines differ - check if it's modification or add/remove
                    // Look ahead to see if the old line appears later in new code
                    let old_in_new = new_lines[new_idx..]
                        .iter()
                        .position(|&l| l == old_lines[old_idx]);
                    let new_in_old = old_lines[old_idx..]
                        .iter()
                        .position(|&l| l == new_lines[new_idx]);

                    match (old_in_new, new_in_old) {
                        (Some(offset), _) if offset < 3 => {
                            // Old line found nearby - these are additions before it
                            for _ in 0..offset {
                                diff_lines.push(DiffLine {
                                    content: new_lines[new_idx].to_string(),
                                    change_type: LineChangeType::Added,
                                    old_line_num: None,
                                    new_line_num: Some(new_idx + 1),
                                });
                                new_idx += 1;
                            }
                        }
                        (_, Some(offset)) if offset < 3 => {
                            // New line found nearby - these are removals before it
                            for _ in 0..offset {
                                diff_lines.push(DiffLine {
                                    content: old_lines[old_idx].to_string(),
                                    change_type: LineChangeType::Removed,
                                    old_line_num: Some(old_idx + 1),
                                    new_line_num: None,
                                });
                                old_idx += 1;
                            }
                        }
                        _ => {
                            // Modification - show as remove + add
                            diff_lines.push(DiffLine {
                                content: old_lines[old_idx].to_string(),
                                change_type: LineChangeType::ModifiedOld,
                                old_line_num: Some(old_idx + 1),
                                new_line_num: None,
                            });
                            diff_lines.push(DiffLine {
                                content: new_lines[new_idx].to_string(),
                                change_type: LineChangeType::ModifiedNew,
                                old_line_num: None,
                                new_line_num: Some(new_idx + 1),
                            });
                            old_idx += 1;
                            new_idx += 1;
                        }
                    }
                }
            } else if old_idx < old_lines.len() {
                // Remaining old lines are removals
                diff_lines.push(DiffLine {
                    content: old_lines[old_idx].to_string(),
                    change_type: LineChangeType::Removed,
                    old_line_num: Some(old_idx + 1),
                    new_line_num: None,
                });
                old_idx += 1;
            } else {
                // Remaining new lines are additions
                diff_lines.push(DiffLine {
                    content: new_lines[new_idx].to_string(),
                    change_type: LineChangeType::Added,
                    old_line_num: None,
                    new_line_num: Some(new_idx + 1),
                });
                new_idx += 1;
            }
        }

        diff_lines
    }

    /// Get diff statistics
    pub fn diff_stats(&self) -> (usize, usize) {
        let added = self
            .diff_lines
            .iter()
            .filter(|l| {
                matches!(
                    l.change_type,
                    LineChangeType::Added | LineChangeType::ModifiedNew
                )
            })
            .count();
        let removed = self
            .diff_lines
            .iter()
            .filter(|l| {
                matches!(
                    l.change_type,
                    LineChangeType::Removed | LineChangeType::ModifiedOld
                )
            })
            .count();
        (added, removed)
    }

    /// Get background color for a diff line
    pub(crate) fn diff_line_bg(
        &self,
        change_type: LineChangeType,
        theme: &crate::app::theme::Theme,
    ) -> Hsla {
        match change_type {
            LineChangeType::Context => gpui::transparent_black(),
            LineChangeType::Added | LineChangeType::ModifiedNew => {
                theme.colors.success.opacity(0.15)
            }
            LineChangeType::Removed | LineChangeType::ModifiedOld => {
                theme.colors.error.opacity(0.15)
            }
        }
    }

    /// Get text color for a diff line prefix
    pub(crate) fn diff_prefix_color(
        &self,
        change_type: LineChangeType,
        theme: &crate::app::theme::Theme,
    ) -> Hsla {
        match change_type {
            LineChangeType::Context => theme.colors.text_muted,
            LineChangeType::Added | LineChangeType::ModifiedNew => theme.colors.success,
            LineChangeType::Removed | LineChangeType::ModifiedOld => theme.colors.error,
        }
    }
}
