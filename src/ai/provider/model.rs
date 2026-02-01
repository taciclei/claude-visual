//! Model information and metadata

use serde::{Deserialize, Serialize};

/// Model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Provider name
    pub provider: String,
    /// Maximum context length
    pub context_length: usize,
    /// Whether model supports streaming
    pub supports_streaming: bool,
    /// Whether model supports tool use
    pub supports_tools: bool,
    /// Whether model supports vision
    pub supports_vision: bool,
    /// Cost per 1K input tokens (USD)
    pub input_cost_per_1k: Option<f64>,
    /// Cost per 1K output tokens (USD)
    pub output_cost_per_1k: Option<f64>,
}
