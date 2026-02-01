//! LSP Protocol Types
//!
//! Types for Language Server Protocol communication.

mod actions;
mod capabilities;
mod completion;
mod diagnostics;
mod documentation;
mod hover;
mod initialization;
mod signature;
mod symbols;
mod types;

// Re-export all public types
pub use actions::*;
pub use capabilities::*;
pub use completion::*;
pub use diagnostics::*;
pub use documentation::*;
pub use hover::*;
pub use initialization::*;
pub use signature::*;
pub use symbols::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let pos = Position::new(10, 5);
        assert_eq!(pos.line, 10);
        assert_eq!(pos.character, 5);
    }

    #[test]
    fn test_range() {
        let range = Range::new(Position::new(0, 0), Position::new(10, 20));
        assert_eq!(range.start.line, 0);
        assert_eq!(range.end.line, 10);
    }
}
