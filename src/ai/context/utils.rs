//! Utility functions for context management

/// Estimate token count for text
pub(crate) fn estimate_tokens(text: &str) -> usize {
    // Rough estimate: ~4 characters per token
    text.len() / 4
}

/// Detect language from MIME type
pub(crate) fn mime_to_language(mime: &str) -> Option<String> {
    match mime {
        "text/x-rust" | "application/x-rust" => Some("rust".to_string()),
        "text/javascript" | "application/javascript" => Some("javascript".to_string()),
        "text/typescript" | "application/typescript" => Some("typescript".to_string()),
        "text/x-python" | "application/x-python" => Some("python".to_string()),
        "text/x-go" => Some("go".to_string()),
        "text/x-java" => Some("java".to_string()),
        "text/x-c" => Some("c".to_string()),
        "text/x-c++" => Some("cpp".to_string()),
        "text/html" => Some("html".to_string()),
        "text/css" => Some("css".to_string()),
        "application/json" => Some("json".to_string()),
        "application/xml" | "text/xml" => Some("xml".to_string()),
        "text/x-yaml" | "application/x-yaml" => Some("yaml".to_string()),
        "text/x-toml" => Some("toml".to_string()),
        "text/markdown" | "text/x-markdown" => Some("markdown".to_string()),
        "application/x-sh" | "text/x-shellscript" => Some("bash".to_string()),
        "text/x-sql" => Some("sql".to_string()),
        _ => None,
    }
}

/// Detect language from URI extension
pub(crate) fn uri_to_language(uri: &str) -> Option<String> {
    // Extract extension from URI
    let path = uri.rsplit('/').next()?;
    let ext = path.rsplit('.').next()?.to_lowercase();

    match ext.as_str() {
        "rs" => Some("rust".to_string()),
        "js" => Some("javascript".to_string()),
        "ts" => Some("typescript".to_string()),
        "tsx" => Some("typescript".to_string()),
        "jsx" => Some("javascript".to_string()),
        "py" => Some("python".to_string()),
        "go" => Some("go".to_string()),
        "java" => Some("java".to_string()),
        "c" | "h" => Some("c".to_string()),
        "cpp" | "cc" | "hpp" => Some("cpp".to_string()),
        "html" | "htm" => Some("html".to_string()),
        "css" => Some("css".to_string()),
        "json" => Some("json".to_string()),
        "xml" => Some("xml".to_string()),
        "yaml" | "yml" => Some("yaml".to_string()),
        "toml" => Some("toml".to_string()),
        "md" | "markdown" => Some("markdown".to_string()),
        "sh" | "bash" => Some("bash".to_string()),
        "sql" => Some("sql".to_string()),
        _ => None,
    }
}
