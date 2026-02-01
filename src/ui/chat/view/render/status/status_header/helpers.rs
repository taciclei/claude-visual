//! Helper utilities for status header rendering

/// Get the spinner character based on animation frame
pub(super) fn get_spinner(frame: usize) -> &'static str {
    match frame % 4 {
        0 => "◐",
        1 => "◓",
        2 => "◑",
        _ => "◒",
    }
}

/// Format elapsed time duration
pub(super) fn format_elapsed_time(elapsed_seconds: i64) -> String {
    if elapsed_seconds < 60 {
        format!("{}s", elapsed_seconds)
    } else {
        format!("{}m", elapsed_seconds / 60)
    }
}

/// Shorten a string to a maximum length with ellipsis
pub(super) fn shorten_with_ellipsis(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len])
    } else {
        s.to_string()
    }
}

/// Get the short session ID (first 8 characters)
pub(super) fn format_session_id_short(session_id: &str) -> String {
    if session_id.len() > 8 {
        format!("#{}", &session_id[..8])
    } else {
        format!("#{}", session_id)
    }
}

/// Get the folder name from a path
pub(super) fn get_folder_name(path: &str) -> String {
    std::path::Path::new(path)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string())
}
