//! Skeleton loading placeholder components

mod card;
mod line;
mod skeleton;
mod text;
mod types;

// Re-export all public types
pub use card::SkeletonCard;
pub use line::SkeletonLine;
pub use skeleton::Skeleton;
pub use text::SkeletonText;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_line_builder() {
        let line = SkeletonLine::new().with_height(20.0).with_width(100.0);

        assert_eq!(line.height, 20.0);
        assert_eq!(line.width, Some(100.0));
    }

    #[test]
    fn test_last_line_width_clamping() {
        let width = 1.5_f32.clamp(0.0, 1.0);
        assert_eq!(width, 1.0);

        let width = (-0.5_f32).clamp(0.0, 1.0);
        assert_eq!(width, 0.0);
    }
}
