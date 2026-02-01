//! Tests

#[cfg(test)]
mod tests {
    use super::super::types::*;
    use crate::terminal::TextStyle;

    #[test]
    fn test_terminal_line() {
        let line = TerminalLine {
            spans: vec![StyledSpan {
                text: "Hello".to_string(),
                style: TextStyle::default(),
            }],
        };
        assert_eq!(line.spans.len(), 1);
    }

    #[test]
    fn test_styled_span() {
        let span = StyledSpan {
            text: "test".to_string(),
            style: TextStyle {
                bold: true,
                ..Default::default()
            },
        };
        assert!(span.style.bold);
    }
}
