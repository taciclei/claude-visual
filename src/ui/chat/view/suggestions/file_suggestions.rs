//! File mention suggestions

use super::super::core::ChatView;

impl ChatView {
    /// Get quick file mention suggestions based on context
    pub fn get_file_suggestions(&self) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Add files from context
        for file in &self.context_files {
            if !suggestions.contains(&file.path) {
                suggestions.push(file.path.clone());
            }
        }

        // Add recently mentioned files
        for file in self.extract_mentioned_files() {
            if !suggestions.contains(&file) {
                suggestions.push(file);
            }
        }

        // Add recent files
        for recent in &self.recent_files {
            if !suggestions.contains(&recent.path) {
                suggestions.push(recent.path.clone());
            }
        }

        // Limit suggestions
        suggestions.truncate(10);
        suggestions
    }
}
