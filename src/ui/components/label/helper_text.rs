use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Helper text below form fields
#[derive(IntoElement)]
pub struct HelperText {
    text: SharedString,
    variant: HelperTextVariant,
    icon: Option<SharedString>,
    color: Option<Hsla>,
}

impl HelperText {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            variant: HelperTextVariant::Default,
            icon: None,
            color: None,
        }
    }

    pub fn variant(mut self, variant: HelperTextVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn success(text: impl Into<SharedString>) -> Self {
        Self::new(text).variant(HelperTextVariant::Success)
    }

    pub fn warning(text: impl Into<SharedString>) -> Self {
        Self::new(text).variant(HelperTextVariant::Warning)
    }

    pub fn error(text: impl Into<SharedString>) -> Self {
        Self::new(text).variant(HelperTextVariant::Error)
    }

    pub fn info(text: impl Into<SharedString>) -> Self {
        Self::new(text).variant(HelperTextVariant::Info)
    }
}

impl RenderOnce for HelperText {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (default_color, default_icon) = match self.variant {
            HelperTextVariant::Default => (
                Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.5,
                    a: 1.0,
                },
                None,
            ),
            HelperTextVariant::Success => (
                Hsla {
                    h: 0.38,
                    s: 0.7,
                    l: 0.5,
                    a: 1.0,
                },
                Some("✓"),
            ),
            HelperTextVariant::Warning => (
                Hsla {
                    h: 0.12,
                    s: 0.9,
                    l: 0.5,
                    a: 1.0,
                },
                Some("⚠"),
            ),
            HelperTextVariant::Error => (
                Hsla {
                    h: 0.0,
                    s: 0.7,
                    l: 0.55,
                    a: 1.0,
                },
                Some("✕"),
            ),
            HelperTextVariant::Info => (
                Hsla {
                    h: 0.58,
                    s: 0.7,
                    l: 0.6,
                    a: 1.0,
                },
                Some("ℹ"),
            ),
        };

        let color = self.color.unwrap_or(default_color);
        let icon = self.icon.or_else(|| default_icon.map(|i| i.into()));

        let mut helper = div()
            .flex()
            .items_center()
            .gap_1()
            .text_size(px(12.0))
            .text_color(color);

        if let Some(icon) = icon {
            helper = helper.child(div().text_size(px(11.0)).child(icon));
        }

        helper.child(self.text)
    }
}
