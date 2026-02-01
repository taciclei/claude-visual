//! Analysis and statistics for conversation summarization

use super::summarizer::Summarizer;
use super::types::*;

impl Summarizer {
    /// Extract topics from conversation (basic implementation)
    pub fn extract_topics(&self) -> Vec<String> {
        let mut topics = Vec::new();
        let mut topic_counts: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for msg in &self.messages {
            // Look for common programming topics
            let content_lower = msg.content.to_lowercase();

            let keywords = [
                ("rust", "Rust"),
                ("javascript", "JavaScript"),
                ("typescript", "TypeScript"),
                ("python", "Python"),
                ("react", "React"),
                ("api", "API"),
                ("database", "Database"),
                ("git", "Git"),
                ("test", "Testing"),
                ("debug", "Debugging"),
                ("error", "Error handling"),
                ("performance", "Performance"),
                ("refactor", "Refactoring"),
                ("deploy", "Deployment"),
                ("security", "Security"),
            ];

            for (keyword, topic) in keywords {
                if content_lower.contains(keyword) {
                    *topic_counts.entry(topic.to_string()).or_insert(0) += 1;
                }
            }
        }

        // Sort by count and take top 5
        let mut sorted: Vec<_> = topic_counts.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));

        for (topic, _) in sorted.into_iter().take(5) {
            topics.push(topic);
        }

        topics
    }

    /// Get statistics about the conversation
    pub fn stats(&self) -> SummarizationStats {
        SummarizationStats {
            total_messages: self.messages.len(),
            summarized_messages: self.summaries.iter().map(|s| s.message_count).sum(),
            total_tokens: self.total_tokens,
            summary_tokens: self.summary_tokens(),
            message_tokens: self.message_tokens(),
            compression_ratio: if self.total_tokens > 0 {
                1.0 - (self.summary_tokens() as f32
                    / (self.summary_tokens()
                        + self
                            .summaries
                            .iter()
                            .map(|s| s.original_tokens)
                            .sum::<usize>()) as f32)
            } else {
                0.0
            },
            summary_count: self.summaries.len(),
        }
    }
}
