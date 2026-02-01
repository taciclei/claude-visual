//! Terminal key handling

/// Terminal special keys
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalKey {
    Enter,
    Tab,
    Backspace,
    Escape,
    Up,
    Down,
    Right,
    Left,
    Home,
    End,
    PageUp,
    PageDown,
    Delete,
    Insert,
    F(u8),
    Ctrl(char),
}

/// Get function key escape sequence
pub(crate) fn function_key_sequence(n: u8) -> String {
    match n {
        1 => "\x1bOP".to_string(),
        2 => "\x1bOQ".to_string(),
        3 => "\x1bOR".to_string(),
        4 => "\x1bOS".to_string(),
        5 => "\x1b[15~".to_string(),
        6 => "\x1b[17~".to_string(),
        7 => "\x1b[18~".to_string(),
        8 => "\x1b[19~".to_string(),
        9 => "\x1b[20~".to_string(),
        10 => "\x1b[21~".to_string(),
        11 => "\x1b[23~".to_string(),
        12 => "\x1b[24~".to_string(),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_key_sequences() {
        assert_eq!(function_key_sequence(1), "\x1bOP");
        assert_eq!(function_key_sequence(5), "\x1b[15~");
    }
}
