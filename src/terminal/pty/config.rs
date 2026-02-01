//! PTY configuration types

use std::path::PathBuf;

/// PTY configuration
#[derive(Debug, Clone)]
pub struct PtyConfig {
    /// Shell to use (defaults to $SHELL or /bin/bash)
    pub shell: String,
    /// Working directory
    pub cwd: Option<PathBuf>,
    /// Environment variables
    pub env: Vec<(String, String)>,
    /// Terminal size (columns, rows)
    pub size: (u16, u16),
    /// History size (lines to keep)
    pub history_size: usize,
}

impl Default for PtyConfig {
    fn default() -> Self {
        Self {
            shell: std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string()),
            cwd: None,
            env: Vec::new(),
            size: (80, 24),
            history_size: 10000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pty_config_default() {
        let config = PtyConfig::default();
        assert!(!config.shell.is_empty());
        assert_eq!(config.size, (80, 24));
    }
}
