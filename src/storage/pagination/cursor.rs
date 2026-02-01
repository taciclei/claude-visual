//! Pagination cursor types and utilities

/// Pagination cursor
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cursor {
    /// Cursor pointing to a specific item by ID
    After(String),
    /// Cursor pointing before a specific item by ID
    Before(String),
    /// Offset-based cursor
    Offset(usize),
}

impl Cursor {
    /// Parse cursor from string
    pub fn parse(s: &str) -> Option<Self> {
        if let Some(id) = s.strip_prefix("after:") {
            Some(Cursor::After(id.to_string()))
        } else if let Some(id) = s.strip_prefix("before:") {
            Some(Cursor::Before(id.to_string()))
        } else if let Some(offset) = s.strip_prefix("offset:") {
            offset.parse().ok().map(Cursor::Offset)
        } else {
            None
        }
    }

    /// Convert cursor to string
    pub fn to_string(&self) -> String {
        match self {
            Cursor::After(id) => format!("after:{}", id),
            Cursor::Before(id) => format!("before:{}", id),
            Cursor::Offset(offset) => format!("offset:{}", offset),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_parsing() {
        assert_eq!(
            Cursor::parse("after:123"),
            Some(Cursor::After("123".to_string()))
        );
        assert_eq!(
            Cursor::parse("before:456"),
            Some(Cursor::Before("456".to_string()))
        );
        assert_eq!(Cursor::parse("offset:10"), Some(Cursor::Offset(10)));
        assert_eq!(Cursor::parse("invalid"), None);
    }
}
