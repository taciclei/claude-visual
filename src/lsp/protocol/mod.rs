//! LSP Protocol Types
//!
//! Types for Language Server Protocol communication.

mod types;
mod diagnostics;
mod documentation;
mod completion;
mod hover;
mod signature;
mod symbols;
mod capabilities;
mod initialization;
mod actions;

// Re-export all public types
pub use types::*;
pub use diagnostics::*;
pub use documentation::*;
pub use completion::*;
pub use hover::*;
pub use signature::*;
pub use symbols::*;
pub use capabilities::*;
pub use initialization::*;
pub use actions::*;

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
