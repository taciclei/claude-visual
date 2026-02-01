//! Skeleton loading placeholder components

mod types;
mod skeleton;
mod line;
mod text;
mod card;

// Re-export all public types
pub use types::*;
pub use skeleton::Skeleton;
pub use line::SkeletonLine;
pub use text::SkeletonText;
pub use card::SkeletonCard;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_line_builder() {
        let line = SkeletonLine::new()
            .with_height(20.0)
            .with_width(100.0);

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
