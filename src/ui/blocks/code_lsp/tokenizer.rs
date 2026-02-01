//! Tokenization logic for code blocks

use super::types::{CodeToken, TokenType};

/// Simple tokenizer (real implementation would use tree-sitter)
pub(crate) fn tokenize(code: &str, _language: Option<&str>) -> Vec<CodeToken> {
    let mut tokens = Vec::new();
    let mut offset = 0;

    for (line_num, line) in code.lines().enumerate() {
        let mut col = 0;
        let mut chars = line.chars().peekable();
        let mut current_word = String::new();
        let mut word_start = 0;

        while let Some(ch) = chars.next() {
            if ch.is_alphanumeric() || ch == '_' {
                if current_word.is_empty() {
                    word_start = col;
                }
                current_word.push(ch);
            } else {
                // Flush current word
                if !current_word.is_empty() {
                    let token_type = classify_token(&current_word);
                    tokens.push(CodeToken {
                        start: offset + word_start,
                        end: offset + col,
                        text: current_word.clone(),
                        line: line_num,
                        column: word_start,
                        token_type,
                    });
                    current_word.clear();
                }
            }
            col += ch.len_utf8();
        }

        // Flush remaining word
        if !current_word.is_empty() {
            let token_type = classify_token(&current_word);
            tokens.push(CodeToken {
                start: offset + word_start,
                end: offset + col,
                text: current_word,
                line: line_num,
                column: word_start,
                token_type,
            });
        }

        offset += line.len() + 1; // +1 for newline
    }

    tokens
}

/// Classify a token (simplified - real impl uses tree-sitter)
pub(crate) fn classify_token(word: &str) -> TokenType {
    // Common keywords
    const KEYWORDS: &[&str] = &[
        "fn",
        "let",
        "const",
        "mut",
        "pub",
        "mod",
        "use",
        "struct",
        "enum",
        "impl",
        "trait",
        "type",
        "where",
        "if",
        "else",
        "match",
        "for",
        "while",
        "loop",
        "return",
        "break",
        "continue",
        "async",
        "await",
        "function",
        "var",
        "class",
        "interface",
        "extends",
        "import",
        "export",
        "def",
        "class",
        "from",
        "return",
        "if",
        "elif",
        "else",
    ];

    if KEYWORDS.contains(&word) {
        TokenType::Keyword
    } else if word
        .chars()
        .next()
        .map(|c| c.is_uppercase())
        .unwrap_or(false)
    {
        TokenType::Type
    } else if word.chars().all(|c| c.is_numeric()) {
        TokenType::Number
    } else {
        TokenType::Identifier
    }
}
