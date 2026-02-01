//! Collapsible trigger with chevron icon

use gpui::*;
use gpui::prelude::*;

/// Collapsible trigger with chevron icon
#[derive(IntoElement)]
pub struct CollapsibleTrigger {
    label: SharedString,
    expanded: bool,
    sublabel: Option<SharedString>,
    text_color: Option<Hsla>,
    icon_color: Option<Hsla>,
    font_size: f32,
    bold: bool,
}

impl CollapsibleTrigger {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            expanded: false,
            sublabel: None,
            text_color: None,
            icon_color: None,
            font_size: 14.0,
            bold: false,
        }
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    pub fn sublabel(mut self, sublabel: impl Into<SharedString>) -> Self {
        self.sublabel = Some(sublabel.into());
        self
    }

    pub fn text_color(mut self, color: Hsla) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn icon_color(mut self, color: Hsla) -> Self {
        self.icon_color = Some(color);
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }
}

impl RenderOnce for CollapsibleTrigger {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text_color = self.text_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.9,
            a: 1.0,
        });
        let icon_color = self.icon_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.6,
            a: 1.0,
        });

        let chevron = if self.expanded { "▼" } else { "▶" };

        let mut label_div = div()
            .text_size(px(self.font_size))
            .text_color(text_color)
            .child(self.label.clone());

        if self.bold {
            label_div = label_div.font_weight(gpui::FontWeight::BOLD);
        }

        div()
            .flex()
            .items_center()
            .justify_between()
            .gap_2()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_size(px(10.0))
                            .text_color(icon_color)
                            .child(chevron),
                    )
                    .child(label_div),
            )
            .when_some(self.sublabel, |d, sublabel| {
                d.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.5,
                            a: 1.0,
                        })
                        .child(sublabel),
                )
            })
    }
}
