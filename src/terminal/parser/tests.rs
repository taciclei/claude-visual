//! Tests for ANSI parser

#[cfg(test)]
mod tests {
    use crate::terminal::parser::{AnsiColor, AnsiEvent, AnsiParser, TextStyle};

    #[test]
    fn test_parse_plain_text() {
        let mut parser = AnsiParser::new();
        let events = parser.parse("Hello, World!");
        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], AnsiEvent::Text(t) if t == "Hello, World!"));
    }

    #[test]
    fn test_parse_newline() {
        let mut parser = AnsiParser::new();
        let events = parser.parse("line1\nline2");
        assert_eq!(events.len(), 3);
        assert!(matches!(&events[0], AnsiEvent::Text(t) if t == "line1"));
        assert!(matches!(&events[1], AnsiEvent::Newline));
        assert!(matches!(&events[2], AnsiEvent::Text(t) if t == "line2"));
    }

    #[test]
    fn test_parse_sgr() {
        let mut parser = AnsiParser::new();
        let events = parser.parse("\x1b[31mred\x1b[0m");

        // Should have: Style(red), Text("red"), Style(reset)
        assert!(events.len() >= 2);
    }

    #[test]
    fn test_color_to_rgb() {
        assert_eq!(AnsiColor::Red.to_rgb(), (205, 49, 49));
        assert_eq!(AnsiColor::BrightGreen.to_rgb(), (35, 209, 139));
    }

    #[test]
    fn test_text_style_apply_sgr() {
        let mut style = TextStyle::default();

        style.apply_sgr(1);
        assert!(style.bold);

        style.apply_sgr(31);
        assert!(matches!(style.fg_color, Some(AnsiColor::Red)));

        style.apply_sgr(0);
        assert!(!style.bold);
        assert!(style.fg_color.is_none());
    }
}
