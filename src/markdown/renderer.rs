//! GPUI markdown renderer

use std::sync::Arc;

use gpui::*;
use gpui::prelude::*;

use crate::app::state::AppState;
use crate::ui::blocks::code_block::CodeBlockView;

use super::parser::{parse, MarkdownElement};

/// Markdown renderer component
#[derive(IntoElement)]
pub struct MarkdownRenderer {
    elements: Vec<MarkdownElement>,
    app_state: Arc<AppState>,
}

impl MarkdownRenderer {
    /// Create a new markdown renderer
    pub fn new(markdown: &str, app_state: Arc<AppState>) -> Self {
        let elements = parse(markdown);
        Self {
            elements,
            app_state,
        }
    }
}

impl RenderOnce for MarkdownRenderer {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme_clone = {
            let theme = self.app_state.theme.read(cx);
            theme.clone()
        };

        // Create code block entities for interactive code blocks
        let mut code_block_idx = 0;
        let mut code_block_entities: Vec<Entity<CodeBlockView>> = Vec::new();

        // Pre-create entities for all code blocks
        for element in &self.elements {
            if let MarkdownElement::CodeBlock { language, code } = element {
                let app_state = self.app_state.clone();
                let code = code.clone();
                let language = language.clone();
                let entity = cx.new(|cx| CodeBlockView::new(code, language, app_state, cx));
                code_block_entities.push(entity);
            }
        }

        div()
            .flex()
            .flex_col()
            .gap_2()
            .children(self.elements.into_iter().map(|element| {
                let result = render_element(
                    element,
                    &theme_clone,
                    self.app_state.clone(),
                    &code_block_entities,
                    &mut code_block_idx,
                );
                result
            }))
    }
}

fn render_element(
    element: MarkdownElement,
    theme: &crate::app::theme::Theme,
    app_state: Arc<AppState>,
    code_blocks: &[Entity<CodeBlockView>],
    code_block_idx: &mut usize,
) -> Div {
    match element {
        MarkdownElement::Text(text) => div().child(text),

        MarkdownElement::Paragraph(content) => div()
            .text_sm()
            .children(content.into_iter().map(|e| render_element(e, theme, app_state.clone(), code_blocks, code_block_idx))),

        MarkdownElement::Heading { level, content } => {
            let size = match level {
                1 => px(24.0),
                2 => px(20.0),
                3 => px(18.0),
                4 => px(16.0),
                _ => px(14.0),
            };
            div()
                .text_size(size)
                .font_weight(FontWeight::BOLD)
                .mt_4()
                .mb_2()
                .children(content.into_iter().map(|e| render_element(e, theme, app_state.clone(), code_blocks, code_block_idx)))
        }

        MarkdownElement::CodeBlock { .. } => {
            let entity = code_blocks.get(*code_block_idx).cloned();
            *code_block_idx += 1;
            div()
                .w_full()
                .my_2()
                .when_some(entity, |this, entity| this.child(entity))
        }

        MarkdownElement::InlineCode(code) => div()
            .px_1()
            .py_0p5()
            .rounded_sm()
            .bg(theme.colors.surface)
            .font_family("JetBrains Mono")
            .text_sm()
            .child(code),

        MarkdownElement::Bold(content) => div()
            .font_weight(FontWeight::BOLD)
            .flex()
            .flex_row()
            .children(content.into_iter().map(|e| render_element(e, theme, app_state.clone(), code_blocks, code_block_idx))),

        MarkdownElement::Italic(content) => div()
            .italic()
            .flex()
            .flex_row()
            .children(content.into_iter().map(|e| render_element(e, theme, app_state.clone(), code_blocks, code_block_idx))),

        MarkdownElement::Strikethrough(content) => div()
            .line_through()
            .flex()
            .flex_row()
            .children(content.into_iter().map(|e| render_element(e, theme, app_state.clone(), code_blocks, code_block_idx))),

        MarkdownElement::Link { url, content, .. } => div().child(
            div()
                .text_color(theme.colors.accent)
                .hover(|style| style.underline())
                .cursor_pointer()
                .flex()
                .flex_row()
                .children(content.into_iter().map(|e| render_element(e, theme, app_state.clone(), code_blocks, code_block_idx)))
                // Store URL as data attribute (for click handling)
                .id(ElementId::Name(url.into()))
        ),

        MarkdownElement::Image { url, alt } => div()
            .flex()
            .flex_col()
            .items_center()
            .my_2()
            .child(
                div()
                    .text_sm()
                    .text_color(theme.colors.text_muted)
                    .child(format!("[Image: {}]", alt)),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child(url),
            ),

        MarkdownElement::List(items) => div()
            .flex()
            .flex_col()
            .gap_1()
            .pl_4()
            .my_2()
            .children(items.into_iter().map(|item| {
                div()
                    .flex()
                    .flex_row()
                    .gap_2()
                    .child(div().text_sm().child("*"))
                    .child(
                        div()
                            .flex_1()
                            .children(item.into_iter().map(|e| render_element(e, theme, app_state.clone(), code_blocks, code_block_idx))),
                    )
            })),

        MarkdownElement::OrderedList { start, items } => div()
            .flex()
            .flex_col()
            .gap_1()
            .pl_4()
            .my_2()
            .children(items.into_iter().enumerate().map(|(i, item)| {
                div()
                    .flex()
                    .flex_row()
                    .gap_2()
                    .child(div().text_sm().min_w(px(20.0)).child(format!("{}.", start as usize + i)))
                    .child(
                        div()
                            .flex_1()
                            .children(item.into_iter().map(|e| render_element(e, theme, app_state.clone(), code_blocks, code_block_idx))),
                    )
            })),

        MarkdownElement::ListItem(content) => div()
            .flex()
            .flex_row()
            .children(content.into_iter().map(|e| render_element(e, theme, app_state.clone(), code_blocks, code_block_idx))),

        MarkdownElement::BlockQuote(content) => div()
            .pl_4()
            .border_l_2()
            .border_color(theme.colors.border)
            .my_2()
            .text_color(theme.colors.text_muted)
            .italic()
            .children(content.into_iter().map(|e| render_element(e, theme, app_state.clone(), code_blocks, code_block_idx))),

        MarkdownElement::HorizontalRule => div()
            .w_full()
            .h(px(1.0))
            .bg(theme.colors.border)
            .my_4(),

        MarkdownElement::Table { headers, rows } => div()
            .w_full()
            .my_2()
            .rounded_md()
            .border_1()
            .border_color(theme.colors.border)
            .overflow_hidden()
            // Header row
            .child(
                div()
                    .flex()
                    .flex_row()
                    .bg(theme.colors.surface)
                    .border_b_1()
                    .border_color(theme.colors.border)
                    .children(headers.into_iter().map(|h| {
                        div()
                            .flex_1()
                            .px_3()
                            .py_2()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(h)
                    })),
            )
            // Data rows
            .children(rows.into_iter().map(|row| {
                div()
                    .flex()
                    .flex_row()
                    .border_b_1()
                    .border_color(theme.colors.border)
                    .children(row.into_iter().map(|cell| {
                        div().flex_1().px_3().py_2().text_sm().child(cell)
                    }))
            })),

        MarkdownElement::SoftBreak => div().child(" "),

        MarkdownElement::HardBreak => div().h(px(16.0)),
    }
}
