//! Video player control utilities

/// Format time in seconds to HH:MM:SS or MM:SS
pub fn format_time(seconds: f64) -> String {
    let hours = (seconds / 3600.0).floor() as i32;
    let mins = ((seconds % 3600.0) / 60.0).floor() as i32;
    let secs = (seconds % 60.0).floor() as i32;

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, mins, secs)
    } else {
        format!("{}:{:02}", mins, secs)
    }
}

/// Format view count with K/M suffixes
pub fn format_views(views: u64) -> String {
    if views >= 1_000_000 {
        format!("{}M views", views / 1_000_000)
    } else if views >= 1_000 {
        format!("{}K views", views / 1_000)
    } else {
        format!("{} views", views)
    }
}
