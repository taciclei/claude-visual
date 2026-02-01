//! Hover information types

use serde::{Deserialize, Serialize};
use super::types::Range;
use super::documentation::{MarkupContent, MarkupKind, MarkedString};

/// Hover information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hover {
    /// Contents
    pub contents: HoverContents,
    /// Range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
}

/// Hover contents
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HoverContents {
    String(String),
    Markup(MarkupContent),
    Array(Vec<MarkedString>),
}

/// Hover client capabilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoverClientCapabilities {
    /// Content format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_format: Option<Vec<MarkupKind>>,
}
