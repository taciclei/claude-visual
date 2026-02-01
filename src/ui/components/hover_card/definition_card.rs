//! Code definition hover card component

use gpui::prelude::*;
use gpui::*;

/// Definition hover card (like for code symbols)
#[derive(IntoElement)]
pub struct DefinitionCard {
    name: SharedString,
    kind: SharedString,
    signature: Option<SharedString>,
    documentation: Option<SharedString>,
    file_path: Option<SharedString>,
    line_number: Option<u32>,
}

impl DefinitionCard {
    pub fn new(name: impl Into<SharedString>, kind: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            kind: kind.into(),
            signature: None,
            documentation: None,
            file_path: None,
            line_number: None,
        }
    }

    pub fn signature(mut self, signature: impl Into<SharedString>) -> Self {
        self.signature = Some(signature.into());
        self
    }

    pub fn documentation(mut self, doc: impl Into<SharedString>) -> Self {
        self.documentation = Some(doc.into());
        self
    }

    pub fn file_path(mut self, path: impl Into<SharedString>) -> Self {
        self.file_path = Some(path.into());
        self
    }

    pub fn line_number(mut self, line: u32) -> Self {
        self.line_number = Some(line);
        self
    }
}

impl RenderOnce for DefinitionCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .max_w(px(400.0))
            // Header with kind badge
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .px_2()
                            .py(px(2.0))
                            .bg(Hsla {
                                h: 0.58,
                                s: 0.5,
                                l: 0.25,
                                a: 1.0,
                            })
                            .rounded(px(4.0))
                            .text_size(px(11.0))
                            .text_color(Hsla {
                                h: 0.58,
                                s: 0.7,
                                l: 0.7,
                                a: 1.0,
                            })
                            .child(self.kind),
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(Hsla {
                                h: 0.0,
                                s: 0.0,
                                l: 0.95,
                                a: 1.0,
                            })
                            .child(self.name),
                    ),
            )
            // Signature
            .when_some(self.signature, |d, sig| {
                d.child(
                    div()
                        .bg(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.1,
                            a: 1.0,
                        })
                        .rounded(px(4.0))
                        .px_2()
                        .py_1()
                        .text_size(px(12.0))
                        .font_family("monospace")
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.8,
                            a: 1.0,
                        })
                        .child(sig),
                )
            })
            // Documentation
            .when_some(self.documentation, |d, doc| {
                d.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.6,
                            a: 1.0,
                        })
                        .line_height(px(16.0))
                        .child(doc),
                )
            })
            // File location
            .when(self.file_path.is_some(), |d| {
                let path = self.file_path.unwrap_or_default();
                let location = if let Some(line) = self.line_number {
                    format!("{}:{}", path, line)
                } else {
                    path.to_string()
                };
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .text_size(px(11.0))
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.4,
                            a: 1.0,
                        })
                        .child("📄")
                        .child(location),
                )
            })
    }
}
