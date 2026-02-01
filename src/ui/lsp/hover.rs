//! Hover Panel
//!
//! Displays hover information from LSP servers.

use gpui::*;
use gpui::prelude::*;
use gpui::prelude::*;

use crate::lsp::protocol::{Hover, HoverContents, MarkedString, MarkupContent};

struct SimpleColors {
    surface: Hsla,
    border: Hsla,
    text: Hsla,
    text_muted: Hsla,
    editor_background: Hsla,
}

fn default_colors() -> SimpleColors {
    SimpleColors {
        surface: hsla(220.0 / 360.0, 0.13, 0.12, 1.0),
        border: hsla(220.0 / 360.0, 0.13, 0.20, 1.0),
        text: hsla(0.0, 0.0, 0.93, 1.0),
        text_muted: hsla(0.0, 0.0, 0.60, 1.0),
        editor_background: hsla(220.0 / 360.0, 0.13, 0.09, 1.0),
    }
}

/// Events emitted by the hover panel
#[derive(Debug, Clone)]
pub enum HoverPanelEvent {
    /// Panel was closed
    Closed,
    /// Go to definition requested
    GoToDefinition,
}

/// Hover panel for displaying LSP hover information
pub struct HoverPanel {
    /// Current hover information
    hover: Option<Hover>,
    /// Position on screen (x, y)
    position: Point<Pixels>,
    /// Whether the panel is visible
    is_visible: bool,
    /// Maximum width
    max_width: Pixels,
}

impl HoverPanel {
    /// Create a new hover panel
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            hover: None,
            position: point(px(0.0), px(0.0)),
            is_visible: false,
            max_width: px(500.0),
        }
    }

    /// Show hover information at a position
    pub fn show(&mut self, hover: Hover, position: Point<Pixels>, cx: &mut Context<Self>) {
        self.hover = Some(hover);
        self.position = position;
        self.is_visible = true;
        cx.notify();
    }

    /// Hide the hover panel
    pub fn hide(&mut self, cx: &mut Context<Self>) {
        self.is_visible = false;
        self.hover = None;
        cx.emit(HoverPanelEvent::Closed);
        cx.notify();
    }

    /// Check if the panel is visible
    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    /// Extract text from hover contents
    fn extract_text(contents: &HoverContents) -> Vec<HoverContentBlock> {
        match contents {
            HoverContents::String(s) => vec![HoverContentBlock::Text(s.clone())],
            HoverContents::Array(strings) => {
                strings
                    .iter()
                    .map(|s| match s {
                        MarkedString::String(text) => HoverContentBlock::Text(text.clone()),
                        MarkedString::LanguageString { language, value } => {
                            HoverContentBlock::Code {
                                language: language.clone(),
                                code: value.clone(),
                            }
                        }
                    })
                    .collect()
            }
            HoverContents::Markup(markup) => Self::parse_markup(markup),
        }
    }

    /// Parse MarkupContent into blocks
    fn parse_markup(markup: &MarkupContent) -> Vec<HoverContentBlock> {
        let mut blocks = Vec::new();
        let text = &markup.value;

        // Simple markdown parsing for code blocks
        let mut in_code_block = false;
        let mut current_language = String::new();
        let mut current_code = String::new();
        let mut current_text = String::new();

        for line in text.lines() {
            if line.starts_with("```") {
                if in_code_block {
                    // End of code block
                    blocks.push(HoverContentBlock::Code {
                        language: current_language.clone(),
                        code: current_code.trim().to_string(),
                    });
                    current_code.clear();
                    current_language.clear();
                    in_code_block = false;
                } else {
                    // Start of code block
                    if !current_text.trim().is_empty() {
                        blocks.push(HoverContentBlock::Text(current_text.trim().to_string()));
                        current_text.clear();
                    }
                    current_language = line[3..].trim().to_string();
                    in_code_block = true;
                }
            } else if in_code_block {
                current_code.push_str(line);
                current_code.push('\n');
            } else {
                current_text.push_str(line);
                current_text.push('\n');
            }
        }

        // Handle remaining content
        if !current_text.trim().is_empty() {
            blocks.push(HoverContentBlock::Text(current_text.trim().to_string()));
        }
        if !current_code.trim().is_empty() {
            blocks.push(HoverContentBlock::Code {
                language: current_language,
                code: current_code.trim().to_string(),
            });
        }

        if blocks.is_empty() {
            blocks.push(HoverContentBlock::Text(text.clone()));
        }

        blocks
    }
}

/// A block of hover content
#[derive(Debug, Clone)]
enum HoverContentBlock {
    /// Plain text or markdown
    Text(String),
    /// Code block with language
    Code { language: String, code: String },
}

impl EventEmitter<HoverPanelEvent> for HoverPanel {}

impl Render for HoverPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let Some(hover) = &self.hover else {
            return div().into_any_element();
        };

        let blocks = Self::extract_text(&hover.contents);
        let colors = default_colors();

        div()
            .absolute()
            .left(self.position.x)
            .top(self.position.y)
            .max_w(self.max_width)
            .bg(colors.surface)
            .border_1()
            .border_color(colors.border)
            .rounded_md()
            .shadow_lg()
            .p_2()
            .overflow_hidden()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .children(blocks.into_iter().map(|block| {
                        let colors = default_colors();
                        match block {
                            HoverContentBlock::Text(text) => div()
                                .text_sm()
                                .text_color(colors.text)
                                .child(text)
                                .into_any_element(),
                            HoverContentBlock::Code { language, code } => div()
                                .bg(colors.editor_background)
                                .rounded_sm()
                                .p_2()
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap_1()
                                        .when(!language.is_empty(), |this| {
                                            let colors = default_colors();
                                            this.child(
                                                div()
                                                    .text_xs()
                                                    .text_color(colors.text_muted)
                                                    .child(language),
                                            )
                                        })
                                        .child(
                                            div()
                                                .font_family("monospace")
                                                .text_sm()
                                                .text_color(colors.text)
                                                .whitespace_nowrap()
                                                .child(code),
                                        ),
                                )
                                .into_any_element(),
                        }
                    })),
            )
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                this.hide(cx);
            }))
            .into_any_element()
    }
}
