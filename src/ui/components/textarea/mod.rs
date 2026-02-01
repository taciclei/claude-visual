//! Textarea component for multiline text input

mod types;
pub mod textarea;
mod code_textarea;

pub use types::{TextareaResize, TextareaEvent};
pub use textarea::Textarea;
pub use code_textarea::CodeTextarea;

#[cfg(test)]
mod tests {
    #[test]
    fn test_line_count() {
        let text = "line1\nline2\nline3";
        assert_eq!(text.lines().count(), 3);
    }

    #[test]
    fn test_max_length() {
        let max_length = 10usize;
        let text = "hello world";
        let truncated: String = text.chars().take(max_length).collect();
        assert_eq!(truncated, "hello worl");
    }
}
