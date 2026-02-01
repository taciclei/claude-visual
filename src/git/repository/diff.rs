//! Diff operations

use anyhow::Result;

use super::Repository;

impl Repository {
    /// Get diff for a specific file (workdir changes, unstaged)
    pub fn file_diff(&self, path: &str) -> Result<String> {
        let mut opts = git2::DiffOptions::new();
        opts.pathspec(path);

        // Get diff between index and workdir (unstaged changes)
        let diff = self.repo.diff_index_to_workdir(None, Some(&mut opts))?;

        let mut diff_output = String::new();
        diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
            let prefix = match line.origin() {
                '+' => "+",
                '-' => "-",
                ' ' => " ",
                'H' | 'F' => "", // Hunk header, file header
                _ => "",
            };
            if !prefix.is_empty() {
                diff_output.push_str(prefix);
            }
            if let Ok(content) = std::str::from_utf8(line.content()) {
                diff_output.push_str(content);
            }
            true
        })?;

        // If no unstaged changes, try staged changes
        if diff_output.is_empty() {
            let head = self.repo.head()?.peel_to_tree()?;
            let diff = self
                .repo
                .diff_tree_to_index(Some(&head), None, Some(&mut opts))?;

            diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
                let prefix = match line.origin() {
                    '+' => "+",
                    '-' => "-",
                    ' ' => " ",
                    _ => "",
                };
                if !prefix.is_empty() {
                    diff_output.push_str(prefix);
                }
                if let Ok(content) = std::str::from_utf8(line.content()) {
                    diff_output.push_str(content);
                }
                true
            })?;
        }

        Ok(diff_output)
    }

    /// Get diff stats for a specific file
    pub fn file_diff_stats(&self, path: &str) -> Result<(usize, usize)> {
        let mut opts = git2::DiffOptions::new();
        opts.pathspec(path);

        // Get diff between index and workdir
        let diff = self.repo.diff_index_to_workdir(None, Some(&mut opts))?;
        let stats = diff.stats()?;

        Ok((stats.insertions(), stats.deletions()))
    }
}
