//! Symbol types for document outline

use super::types::Range;
use serde::{Deserialize, Serialize};

/// Symbol kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum SymbolKind {
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Enum = 10,
    Interface = 11,
    Function = 12,
    Variable = 13,
    Constant = 14,
    String = 15,
    Number = 16,
    Boolean = 17,
    Array = 18,
    Object = 19,
    Key = 20,
    Null = 21,
    EnumMember = 22,
    Struct = 23,
    Event = 24,
    Operator = 25,
    TypeParameter = 26,
}

impl SymbolKind {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::File => "📄",
            Self::Module => "📦",
            Self::Namespace => "🗃",
            Self::Package => "📦",
            Self::Class => "📦",
            Self::Method => "🔧",
            Self::Property => "🏷",
            Self::Field => "📋",
            Self::Constructor => "🏗",
            Self::Enum => "📊",
            Self::Interface => "🔌",
            Self::Function => "ƒ",
            Self::Variable => "𝑥",
            Self::Constant => "🔒",
            Self::String => "📝",
            Self::Number => "🔢",
            Self::Boolean => "✓",
            Self::Array => "[]",
            Self::Object => "{}",
            Self::Key => "🔑",
            Self::Null => "∅",
            Self::EnumMember => "📈",
            Self::Struct => "🧱",
            Self::Event => "⚡",
            Self::Operator => "➕",
            Self::TypeParameter => "🅿",
        }
    }
}

/// Document symbol
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSymbol {
    /// Symbol name
    pub name: String,
    /// Symbol detail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// Symbol kind
    pub kind: SymbolKind,
    /// Range of the whole symbol
    pub range: Range,
    /// Range of the symbol name
    pub selection_range: Range,
    /// Children
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<DocumentSymbol>>,
}
