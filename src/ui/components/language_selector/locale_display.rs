//! Locale display component

use gpui::prelude::*;
use gpui::*;

/// Locale display component
#[derive(IntoElement)]
pub struct LocaleDisplay {
    id: ElementId,
    language_code: SharedString,
    region_code: Option<SharedString>,
    show_flag: bool,
}

impl LocaleDisplay {
    pub fn new(id: impl Into<ElementId>, language_code: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            language_code: language_code.into(),
            region_code: None,
            show_flag: true,
        }
    }

    pub fn region(mut self, region: impl Into<SharedString>) -> Self {
        self.region_code = Some(region.into());
        self
    }

    pub fn show_flag(mut self, show: bool) -> Self {
        self.show_flag = show;
        self
    }

    fn get_flag(code: &str) -> &'static str {
        match code.to_lowercase().as_str() {
            "en" | "us" => "🇺🇸",
            "gb" | "uk" => "🇬🇧",
            "fr" => "🇫🇷",
            "de" => "🇩🇪",
            "es" => "🇪🇸",
            "pt" | "br" => "🇧🇷",
            "it" => "🇮🇹",
            "nl" => "🇳🇱",
            "ru" => "🇷🇺",
            "zh" | "cn" => "🇨🇳",
            "ja" | "jp" => "🇯🇵",
            "ko" | "kr" => "🇰🇷",
            "ar" | "sa" => "🇸🇦",
            _ => "🌐",
        }
    }
}

impl RenderOnce for LocaleDisplay {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let display_code = if let Some(region) = &self.region_code {
            format!("{}-{}", self.language_code, region).to_uppercase()
        } else {
            self.language_code.to_uppercase().to_string()
        };

        let flag_code = self
            .region_code
            .as_ref()
            .map(|r| r.as_ref())
            .unwrap_or(self.language_code.as_ref());

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(6.0))
            .when(self.show_flag, |el| {
                el.child(div().text_size(px(16.0)).child(Self::get_flag(flag_code)))
            })
            .child(
                div()
                    .text_size(px(12.0))
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .text_color(hsla(0.0, 0.0, 0.7, 1.0))
                    .child(display_code),
            )
    }
}
