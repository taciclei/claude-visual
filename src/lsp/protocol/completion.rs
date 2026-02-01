//! Code completion types

use super::documentation::Documentation;
use super::types::TextEdit;
use serde::{Deserialize, Serialize};

/// Completion item kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Unit = 11,
    Value = 12,
    Enum = 13,
    Keyword = 14,
    Snippet = 15,
    Color = 16,
    File = 17,
    Reference = 18,
    Folder = 19,
    EnumMember = 20,
    Constant = 21,
    Struct = 22,
    Event = 23,
    Operator = 24,
    TypeParameter = 25,
}

impl CompletionItemKind {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Text => "📝",
            Self::Method => "🔧",
            Self::Function => "ƒ",
            Self::Constructor => "🏗",
            Self::Field => "📋",
            Self::Variable => "𝑥",
            Self::Class => "📦",
            Self::Interface => "🔌",
            Self::Module => "📁",
            Self::Property => "🏷",
            Self::Unit => "📏",
            Self::Value => "💎",
            Self::Enum => "📊",
            Self::Keyword => "🔑",
            Self::Snippet => "✂️",
            Self::Color => "🎨",
            Self::File => "📄",
            Self::Reference => "🔗",
            Self::Folder => "📂",
            Self::EnumMember => "📈",
            Self::Constant => "🔒",
            Self::Struct => "🧱",
            Self::Event => "⚡",
            Self::Operator => "➕",
            Self::TypeParameter => "🅿",
        }
    }
}

/// Completion item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItem {
    /// Label to display
    pub label: String,
    /// Kind of completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<CompletionItemKind>,
    /// Detail (type annotation, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// Documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
    /// String to insert when selected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text: Option<String>,
    /// Text edit to apply
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_edit: Option<TextEdit>,
    /// Additional text edits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_text_edits: Option<Vec<TextEdit>>,
    /// Sort text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_text: Option<String>,
    /// Filter text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_text: Option<String>,
}

/// Completion options
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionOptions {
    /// Trigger characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_characters: Option<Vec<String>>,
    /// Resolve provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_provider: Option<bool>,
}

/// Completion client capabilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionClientCapabilities {
    /// Snippet support
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet_support: Option<bool>,
}
