//! Export format types

/// Export format options
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ExportFormat {
    #[default]
    Markdown,
    Json,
    Html,
    PlainText,
}

impl ExportFormat {
    /// Display name for the format
    pub fn display_name(&self) -> &'static str {
        match self {
            ExportFormat::Markdown => "Markdown",
            ExportFormat::Json => "JSON",
            ExportFormat::Html => "HTML",
            ExportFormat::PlainText => "Plain Text",
        }
    }

    /// File extension for the format
    pub fn extension(&self) -> &'static str {
        match self {
            ExportFormat::Markdown => "md",
            ExportFormat::Json => "json",
            ExportFormat::Html => "html",
            ExportFormat::PlainText => "txt",
        }
    }

    /// Icon for the format
    pub fn icon(&self) -> &'static str {
        match self {
            ExportFormat::Markdown => "📝",
            ExportFormat::Json => "{}",
            ExportFormat::Html => "🌐",
            ExportFormat::PlainText => "📄",
        }
    }

    /// All formats
    pub fn all() -> &'static [ExportFormat] {
        &[
            ExportFormat::Markdown,
            ExportFormat::Json,
            ExportFormat::Html,
            ExportFormat::PlainText,
        ]
    }
}
