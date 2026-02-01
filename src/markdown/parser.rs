//! Markdown parser using pulldown-cmark

use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};

/// Parsed markdown element
#[derive(Debug, Clone)]
pub enum MarkdownElement {
    /// Plain text
    Text(String),
    /// Heading with level (1-6)
    Heading { level: u8, content: Vec<MarkdownElement> },
    /// Paragraph
    Paragraph(Vec<MarkdownElement>),
    /// Code block with optional language
    CodeBlock { language: Option<String>, code: String },
    /// Inline code
    InlineCode(String),
    /// Bold text
    Bold(Vec<MarkdownElement>),
    /// Italic text
    Italic(Vec<MarkdownElement>),
    /// Strikethrough text
    Strikethrough(Vec<MarkdownElement>),
    /// Link with URL and title
    Link { url: String, title: Option<String>, content: Vec<MarkdownElement> },
    /// Image with URL and alt text
    Image { url: String, alt: String },
    /// Unordered list
    List(Vec<Vec<MarkdownElement>>),
    /// Ordered list
    OrderedList { start: u64, items: Vec<Vec<MarkdownElement>> },
    /// List item
    ListItem(Vec<MarkdownElement>),
    /// Block quote
    BlockQuote(Vec<MarkdownElement>),
    /// Horizontal rule
    HorizontalRule,
    /// Table
    Table { headers: Vec<String>, rows: Vec<Vec<String>> },
    /// Soft break (single newline)
    SoftBreak,
    /// Hard break (two newlines or <br>)
    HardBreak,
}

/// Parse markdown text into elements
pub fn parse(markdown: &str) -> Vec<MarkdownElement> {
    let options = Options::all();
    let parser = Parser::new_ext(markdown, options);

    let mut elements = Vec::new();
    let mut stack: Vec<(Tag<'_>, Vec<MarkdownElement>)> = Vec::new();
    let mut current_text = String::new();

    for event in parser {
        match event {
            Event::Start(tag) => {
                // Flush any accumulated text
                if !current_text.is_empty() {
                    let text = std::mem::take(&mut current_text);
                    if let Some((_, ref mut content)) = stack.last_mut() {
                        content.push(MarkdownElement::Text(text));
                    } else {
                        elements.push(MarkdownElement::Text(text));
                    }
                }
                stack.push((tag, Vec::new()));
            }
            Event::End(tag_end) => {
                if let Some((tag, content)) = stack.pop() {
                    let element = tag_to_element(tag, tag_end, content);
                    if let Some((_, ref mut parent_content)) = stack.last_mut() {
                        parent_content.push(element);
                    } else {
                        elements.push(element);
                    }
                }
            }
            Event::Text(text) => {
                current_text.push_str(&text);
            }
            Event::Code(code) => {
                let element = MarkdownElement::InlineCode(code.to_string());
                if let Some((_, ref mut content)) = stack.last_mut() {
                    content.push(element);
                } else {
                    elements.push(element);
                }
            }
            Event::SoftBreak => {
                if let Some((_, ref mut content)) = stack.last_mut() {
                    content.push(MarkdownElement::SoftBreak);
                } else {
                    elements.push(MarkdownElement::SoftBreak);
                }
            }
            Event::HardBreak => {
                if let Some((_, ref mut content)) = stack.last_mut() {
                    content.push(MarkdownElement::HardBreak);
                } else {
                    elements.push(MarkdownElement::HardBreak);
                }
            }
            Event::Rule => {
                elements.push(MarkdownElement::HorizontalRule);
            }
            _ => {}
        }
    }

    // Flush remaining text
    if !current_text.is_empty() {
        elements.push(MarkdownElement::Text(current_text));
    }

    elements
}

fn tag_to_element(tag: Tag<'_>, _tag_end: TagEnd, content: Vec<MarkdownElement>) -> MarkdownElement {
    match tag {
        Tag::Paragraph => MarkdownElement::Paragraph(content),
        Tag::Heading { level, .. } => MarkdownElement::Heading {
            level: level as u8,
            content,
        },
        Tag::BlockQuote(_) => MarkdownElement::BlockQuote(content),
        Tag::CodeBlock(kind) => {
            let language = match kind {
                CodeBlockKind::Fenced(lang) if !lang.is_empty() => Some(lang.to_string()),
                _ => None,
            };
            let code = content
                .into_iter()
                .filter_map(|e| {
                    if let MarkdownElement::Text(t) = e {
                        Some(t)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .join("");
            MarkdownElement::CodeBlock { language, code }
        }
        Tag::List(start) => {
            if let Some(start) = start {
                MarkdownElement::OrderedList {
                    start,
                    items: content
                        .into_iter()
                        .filter_map(|e| {
                            if let MarkdownElement::ListItem(items) = e {
                                Some(items)
                            } else {
                                None
                            }
                        })
                        .collect(),
                }
            } else {
                MarkdownElement::List(
                    content
                        .into_iter()
                        .filter_map(|e| {
                            if let MarkdownElement::ListItem(items) = e {
                                Some(items)
                            } else {
                                None
                            }
                        })
                        .collect(),
                )
            }
        }
        Tag::Item => MarkdownElement::ListItem(content),
        Tag::Emphasis => MarkdownElement::Italic(content),
        Tag::Strong => MarkdownElement::Bold(content),
        Tag::Strikethrough => MarkdownElement::Strikethrough(content),
        Tag::Link { dest_url, title, .. } => MarkdownElement::Link {
            url: dest_url.to_string(),
            title: if title.is_empty() {
                None
            } else {
                Some(title.to_string())
            },
            content,
        },
        Tag::Image { dest_url, title, .. } => MarkdownElement::Image {
            url: dest_url.to_string(),
            alt: title.to_string(),
        },
        _ => MarkdownElement::Paragraph(content),
    }
}

/// Extract all code blocks from markdown
pub fn extract_code_blocks(markdown: &str) -> Vec<(Option<String>, String)> {
    let elements = parse(markdown);
    let mut code_blocks = Vec::new();

    fn extract_from_elements(elements: &[MarkdownElement], blocks: &mut Vec<(Option<String>, String)>) {
        for element in elements {
            match element {
                MarkdownElement::CodeBlock { language, code } => {
                    blocks.push((language.clone(), code.clone()));
                }
                MarkdownElement::Paragraph(content)
                | MarkdownElement::BlockQuote(content)
                | MarkdownElement::Bold(content)
                | MarkdownElement::Italic(content)
                | MarkdownElement::ListItem(content) => {
                    extract_from_elements(content, blocks);
                }
                MarkdownElement::List(items) | MarkdownElement::OrderedList { items, .. } => {
                    for item in items {
                        extract_from_elements(item, blocks);
                    }
                }
                _ => {}
            }
        }
    }

    extract_from_elements(&elements, &mut code_blocks);
    code_blocks
}
