//! Helper functions for status bar rendering

/// Format duration as HHh MMm
pub(crate) fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

/// Generate usage bar using braille characters
pub(crate) fn render_usage_bar(usage_percent: u8) -> String {
    let filled = (usage_percent as usize * 10) / 100;
    let empty = 10 - filled;
    let mut bar = String::new();
    for _ in 0..filled {
        bar.push('\u{28FF}'); // Full braille block ⣿
    }
    for i in 0..empty {
        if i == 0 && filled > 0 {
            bar.push('\u{2846}'); // Partial braille ⡆
        } else {
            bar.push('\u{2800}'); // Empty braille ⠀
        }
    }
    bar
}

/// Shorten path for display (e.g., "…/Sites/claude-visual")
pub(crate) fn shorten_path(path: &Option<String>) -> String {
    if let Some(path) = path {
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        if parts.len() > 2 {
            format!("…/{}/{}", parts[parts.len() - 2], parts[parts.len() - 1])
        } else {
            path.clone()
        }
    } else {
        "No project".to_string()
    }
}

/// Get session health indicator color and label
pub(crate) fn health_indicator(health: f32) -> (&'static str, &'static str) {
    if health > 0.7 {
        ("💚", "Healthy")
    } else if health > 0.4 {
        ("💛", "Fair")
    } else {
        ("❤️", "Low")
    }
}

/// Get response latency indicator
pub(crate) fn latency_indicator(latency_ms: Option<u64>) -> Option<(&'static str, String)> {
    latency_ms.map(|ms| {
        if ms < 1000 {
            ("⚡", format!("{}ms", ms))
        } else if ms < 5000 {
            ("⏱️", format!("{:.1}s", ms as f64 / 1000.0))
        } else {
            ("🐌", format!("{:.1}s", ms as f64 / 1000.0))
        }
    })
}
