//! JSON value display component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// JSON value display (colored)
#[derive(Clone)]
pub struct JsonValue {
    value: JsonType,
}

impl JsonValue {
    pub fn string(s: impl Into<String>) -> Self {
        Self {
            value: JsonType::String(s.into()),
        }
    }

    pub fn number(n: f64) -> Self {
        Self {
            value: JsonType::Number(n),
        }
    }

    pub fn boolean(b: bool) -> Self {
        Self {
            value: JsonType::Boolean(b),
        }
    }

    pub fn null() -> Self {
        Self {
            value: JsonType::Null,
        }
    }

    pub fn array(count: usize) -> Self {
        Self {
            value: JsonType::Array(count),
        }
    }

    pub fn object(count: usize) -> Self {
        Self {
            value: JsonType::Object(count),
        }
    }
}

impl RenderOnce for JsonValue {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let string_color = hsla(0.38, 0.6, 0.55, 1.0); // Green
        let number_color = hsla(0.55, 0.7, 0.6, 1.0); // Blue
        let bool_color = hsla(0.75, 0.6, 0.6, 1.0); // Purple
        let null_color = hsla(0.0, 0.0, 0.5, 1.0); // Gray
        let bracket_color = hsla(0.0, 0.0, 0.6, 1.0);

        let (color, text) = match self.value {
            JsonType::String(s) => (string_color, format!("\"{}\"", s)),
            JsonType::Number(n) => (number_color, format!("{}", n)),
            JsonType::Boolean(b) => (bool_color, format!("{}", b)),
            JsonType::Null => (null_color, "null".to_string()),
            JsonType::Array(count) => (bracket_color, format!("[{} items]", count)),
            JsonType::Object(count) => (bracket_color, format!("{{{} keys}}", count)),
        };

        div()
            .text_sm()
            .font_family("monospace")
            .text_color(color)
            .child(text)
    }
}
