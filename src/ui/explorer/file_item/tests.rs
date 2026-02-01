//! Tests for file_item module

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::ui::explorer::file_item::{FileEntry, FileType, GitStatus};

    #[test]
    fn test_file_type() {
        assert_eq!(FileType::Directory, FileType::Directory);
        assert_ne!(FileType::File, FileType::Directory);
    }

    #[test]
    fn test_git_status_char() {
        assert_eq!(GitStatus::Modified.char(), 'M');
        assert_eq!(GitStatus::Added.char(), 'A');
        assert_eq!(GitStatus::Untracked.char(), '?');
    }

    #[test]
    fn test_file_entry_icon() {
        let entry = FileEntry {
            name: "main.rs".to_string(),
            path: PathBuf::from("main.rs"),
            file_type: FileType::File,
            git_status: None,
            size: None,
            modified: None,
            is_expanded: false,
            children: Vec::new(),
            depth: 0,
            is_selected: false,
            is_hovered: false,
        };
        assert_eq!(entry.icon(), "🦀");
    }

    #[test]
    fn test_file_entry_directory_icon() {
        let mut entry = FileEntry::directory(PathBuf::from("/test"), "test".to_string(), 0);
        assert_eq!(entry.icon(), "📁");

        entry.is_expanded = true;
        assert_eq!(entry.icon(), "📂");
    }

    #[test]
    fn test_formatted_size() {
        let mut entry = FileEntry::file(PathBuf::from("test.txt"), "test.txt".to_string(), 0);

        entry.size = Some(512);
        assert_eq!(entry.formatted_size(), Some("512 B".to_string()));

        entry.size = Some(1536);
        assert_eq!(entry.formatted_size(), Some("1.5 KB".to_string()));

        entry.size = Some(1024 * 1024 * 2);
        assert_eq!(entry.formatted_size(), Some("2.0 MB".to_string()));
    }
}
