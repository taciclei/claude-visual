//! Tests for comment system

#[cfg(test)]
mod tests {
    use super::super::{InlineComment, CommentThread, DiffComments};

    #[test]
    fn test_inline_comment() {
        let comment = InlineComment::new("user1".to_string(), "Great change!".to_string());
        assert!(!comment.resolved);
        assert!(!comment.edited);
        assert!(comment.reply_to.is_none());
    }

    #[test]
    fn test_reply() {
        let parent = InlineComment::new("user1".to_string(), "Question about this".to_string());
        let reply = InlineComment::reply(&parent.id, "user2".to_string(), "Here's the answer".to_string());

        assert_eq!(reply.reply_to.as_ref().unwrap(), &parent.id);
    }

    #[test]
    fn test_comment_thread() {
        let mut thread = CommentThread::new(0, 5, "new");
        thread.add_comment(InlineComment::new("user1".to_string(), "Comment 1".to_string()));
        thread.add_comment(InlineComment::new("user2".to_string(), "Comment 2".to_string()));

        assert_eq!(thread.comment_count(), 2);
        assert!(thread.has_unresolved());

        thread.resolve();
        assert!(!thread.has_unresolved());
    }

    #[test]
    fn test_diff_comments() {
        let mut comments = DiffComments::new("reviewer".to_string());

        comments.add_comment(0, 5, "new", "This looks good".to_string());
        comments.add_comment(0, 10, "old", "Why was this removed?".to_string());

        assert_eq!(comments.total_comments(), 2);
        assert!(comments.has_comments(0, 5, "new"));
        assert!(!comments.has_comments(0, 1, "new"));
    }
}
