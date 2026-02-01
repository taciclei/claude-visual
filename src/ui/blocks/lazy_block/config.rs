//! Configuration for lazy loading

/// Configuration for lazy loading
#[derive(Debug, Clone)]
pub struct LazyBlockConfig {
    /// Height estimate for placeholder
    pub estimated_height: f32,
    /// Minimum time visible before loading (debounce)
    pub visibility_delay_ms: u64,
    /// Whether to preload when near viewport
    pub preload_margin: f32,
    /// Show skeleton while loading
    pub show_skeleton: bool,
}

impl Default for LazyBlockConfig {
    fn default() -> Self {
        Self {
            estimated_height: 100.0,
            visibility_delay_ms: 50,
            preload_margin: 200.0,
            show_skeleton: true,
        }
    }
}

impl LazyBlockConfig {
    /// Config for code blocks
    pub fn for_code_block(line_count: usize) -> Self {
        Self {
            estimated_height: (line_count as f32 * 18.0).min(500.0), // ~18px per line, max 500px
            visibility_delay_ms: 16, // ~1 frame
            preload_margin: 300.0,
            show_skeleton: true,
        }
    }

    /// Config for diff blocks
    pub fn for_diff_block(line_count: usize) -> Self {
        Self {
            estimated_height: (line_count as f32 * 20.0).min(600.0),
            visibility_delay_ms: 16,
            preload_margin: 300.0,
            show_skeleton: true,
        }
    }

    /// Config for images
    pub fn for_image(width: f32, height: f32) -> Self {
        Self {
            estimated_height: height,
            visibility_delay_ms: 0, // Load immediately when visible
            preload_margin: 500.0, // Preload images earlier
            show_skeleton: true,
        }
    }
}
