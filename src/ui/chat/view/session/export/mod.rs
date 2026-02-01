//! Export methods for conversations

mod html;
mod json;
mod markdown;
mod text;

use super::super::core::ChatView;
use super::super::types::ExportFormat;

impl ChatView {
    /// Export with current settings
    pub fn export_with_format(&self) -> String {
        match self.export.format {
            ExportFormat::Markdown => self.export_to_markdown(),
            ExportFormat::Json => self.export_to_json(),
            ExportFormat::Html => self.export_to_html(),
            ExportFormat::PlainText => self.export_to_plain_text(),
        }
    }
}
