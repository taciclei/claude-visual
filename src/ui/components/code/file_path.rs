//! File path display component

use gpui::*;
use gpui::prelude::*;

/// File path display
#[derive(Clone)]
pub struct FilePath {
    path: String,
    clickable: bool,
    show_icon: bool,
}

impl FilePath {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            clickable: false,
            show_icon: true,
        }
    }

    pub fn clickable(mut self) -> Self {
        self.clickable = true;
        self
    }

    pub fn no_icon(mut self) -> Self {
        self.show_icon = false;
        self
    }

    fn file_icon(path: &str) -> &'static str {
        if path.ends_with('/') || !path.contains('.') {
            return "📁";
        }

        let ext = path.rsplit('.').next().unwrap_or("");
        match ext {
            "rs" => "🦀",
            "js" | "ts" | "jsx" | "tsx" => "📜",
            "py" => "🐍",
            "md" => "📝",
            "json" => "📋",
            "toml" | "yaml" | "yml" => "⚙️",
            "html" | "css" => "🌐",
            "sh" | "bash" => "💻",
            _ => "📄",
        }
    }
}

impl RenderOnce for FilePath {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bg = hsla(0.0, 0.0, 0.15, 1.0);
        let text = hsla(0.55, 0.6, 0.7, 1.0); // Blue for paths
        let accent = hsla(0.6, 0.8, 0.6, 1.0);

        let icon = Self::file_icon(&self.path);

        let mut container = div()
            .px_2()
            .py_1()
            .bg(bg)
            .rounded(px(4.0))
            .flex()
            .items_center()
            .gap_2()
            .font_family("monospace")
            .text_sm();

        if self.clickable {
            container = container
                .cursor_pointer()
                .hover(|s| s.bg(hsla(0.0, 0.0, 0.2, 1.0)).text_color(accent));
        }

        if self.show_icon {
            container = container.child(
                div()
                    .text_sm()
                    .child(icon)
            );
        }

        container.child(
            div()
                .text_color(text)
                .child(self.path)
        )
    }
}
