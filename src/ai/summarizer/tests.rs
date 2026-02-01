//! Tests for conversation summarization

#[cfg(test)]
mod tests {
    use crate::ai::summarizer::{
        ConversationMessage, ConversationSummary, MessageRole, SummarizationConfig, Summarizer,
    };

    #[test]
    fn test_conversation_message() {
        let msg = ConversationMessage::user("Hello, can you help me?");
        assert_eq!(msg.role, MessageRole::User);
        assert!(!msg.has_code);
        assert!(msg.token_count > 0);
    }

    #[test]
    fn test_message_with_code() {
        let msg = ConversationMessage::assistant("Here's some code:\n```rust\nfn main() {}\n```");
        assert!(msg.has_code);
    }

    #[test]
    fn test_summarizer_needs_summarization() {
        let config = SummarizationConfig {
            token_threshold: 100,
            min_messages: 2,
            preserve_recent: 1,
            ..Default::default()
        };

        let mut summarizer = Summarizer::new(config);

        // Not enough messages
        summarizer.add_message(ConversationMessage::user("a".repeat(200)));
        assert!(!summarizer.needs_summarization());

        // Enough messages, exceeds threshold
        summarizer.add_message(ConversationMessage::assistant("b".repeat(200)));
        assert!(summarizer.needs_summarization());
    }

    #[test]
    fn test_messages_to_summarize() {
        let config = SummarizationConfig {
            preserve_recent: 2,
            ..Default::default()
        };

        let mut summarizer = Summarizer::new(config);

        summarizer.add_message(ConversationMessage::user("msg1"));
        summarizer.add_message(ConversationMessage::assistant("msg2"));
        summarizer.add_message(ConversationMessage::user("msg3"));
        summarizer.add_message(ConversationMessage::assistant("msg4"));

        let to_summarize = summarizer.messages_to_summarize();
        assert_eq!(to_summarize.len(), 2);
    }

    #[test]
    fn test_summary_compression_ratio() {
        let summary = ConversationSummary::new(
            "Brief summary",
            10,
            1000,
            (chrono::Utc::now(), chrono::Utc::now()),
        );

        // Should have high compression ratio
        assert!(summary.compression_ratio() > 0.9);
    }

    #[test]
    fn test_extract_topics() {
        let mut summarizer = Summarizer::new(SummarizationConfig::default());

        summarizer.add_message(ConversationMessage::user(
            "I need help with Rust and database performance"
        ));
        summarizer.add_message(ConversationMessage::assistant(
            "Let me help you with the Rust code and optimize the database queries"
        ));

        let topics = summarizer.extract_topics();
        assert!(topics.contains(&"Rust".to_string()));
        assert!(topics.contains(&"Database".to_string()));
        assert!(topics.contains(&"Performance".to_string()));
    }
}
