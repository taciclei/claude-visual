//! Core LSP integration logic

use crate::lsp::protocol::{Location, Position, Range};
use gpui::*;

use super::tokenizer;
use super::types::{CodeLspConfig, CodeLspEvent, CodeToken};

/// Code block LSP integration component
pub struct CodeLspIntegration {
    /// Configuration
    pub(crate) config: CodeLspConfig,
    /// Code content
    pub(crate) code: String,
    /// Language
    pub(crate) language: Option<String>,
    /// Parsed tokens
    pub(crate) tokens: Vec<CodeToken>,
    /// Currently hovered token
    pub(crate) hovered_token: Option<usize>,
    /// Currently selected symbol
    pub(crate) selected_symbol: Option<String>,
    /// Hover timer
    pub(crate) hover_pending: bool,
}

impl CodeLspIntegration {
    /// Create new LSP integration
    pub fn new(code: String, language: Option<String>, config: CodeLspConfig) -> Self {
        let tokens = tokenizer::tokenize(&code, language.as_deref());

        Self {
            config,
            code,
            language,
            tokens,
            hovered_token: None,
            selected_symbol: None,
            hover_pending: false,
        }
    }

    /// Find token at byte offset
    pub fn token_at_offset(&self, offset: usize) -> Option<&CodeToken> {
        self.tokens
            .iter()
            .find(|t| offset >= t.start && offset < t.end)
    }

    /// Find token at line/column
    pub fn token_at_position(&self, line: usize, column: usize) -> Option<&CodeToken> {
        self.tokens
            .iter()
            .find(|t| t.line == line && column >= t.column && column < t.column + t.text.len())
    }

    /// Get position from byte offset
    pub fn offset_to_position(&self, offset: usize) -> Position {
        let mut line = 0;
        let mut col = 0;
        let mut current_offset = 0;

        for ch in self.code.chars() {
            if current_offset >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
            current_offset += ch.len_utf8();
        }

        Position {
            line: line as u32,
            character: col as u32,
        }
    }

    /// Handle mouse click at position
    pub fn handle_click(
        &mut self,
        line: usize,
        column: usize,
        modifiers: Modifiers,
    ) -> Option<CodeLspEvent> {
        if !self.config.enable_goto_definition {
            return None;
        }

        // Ctrl+Click or Cmd+Click triggers go-to-definition
        if modifiers.control || modifiers.platform {
            if let Some(token) = self.token_at_position(line, column) {
                if token.token_type.supports_goto() {
                    // In a real implementation, this would call the LSP server
                    // For now, we emit an event with a placeholder location
                    let position = Position {
                        line: token.line as u32,
                        character: token.column as u32,
                    };

                    return Some(CodeLspEvent::GoToDefinition(Location {
                        uri: self
                            .config
                            .virtual_file_path
                            .as_ref()
                            .map(|p| format!("file://{}", p.display()))
                            .unwrap_or_else(|| "file:///unknown".to_string()),
                        range: Range {
                            start: position.clone(),
                            end: Position {
                                line: position.line,
                                character: position.character + token.text.len() as u32,
                            },
                        },
                    }));
                }
            }
        }

        None
    }

    /// Handle mouse hover
    pub fn handle_hover(&mut self, line: usize, column: usize) -> Option<CodeLspEvent> {
        if !self.config.enable_hover {
            return None;
        }

        let token_idx = self
            .tokens
            .iter()
            .position(|t| t.line == line && column >= t.column && column < t.column + t.text.len());

        if self.hovered_token != token_idx {
            self.hovered_token = token_idx;
            self.hover_pending = token_idx.is_some();
        }

        // In a real implementation, this would query the LSP server
        // after hover_delay_ms
        None
    }

    /// Handle symbol selection for highlighting
    pub fn handle_selection(
        &mut self,
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_col: usize,
    ) -> Option<CodeLspEvent> {
        if !self.config.enable_symbol_highlight {
            return None;
        }

        // Check if selection is exactly one token
        if start_line == end_line {
            let token_text = if let Some(token) = self.token_at_position(start_line, start_col) {
                if token.column == start_col && token.column + token.text.len() == end_col {
                    if token.token_type.supports_goto() {
                        Some((token.text.clone(), token.line, token.column))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };

            if let Some((text, line, column)) = token_text {
                self.selected_symbol = Some(text.clone());
                return Some(CodeLspEvent::SymbolHighlighted {
                    symbol: text.clone(),
                    range: Range {
                        start: Position {
                            line: line as u32,
                            character: column as u32,
                        },
                        end: Position {
                            line: line as u32,
                            character: (column + text.len()) as u32,
                        },
                    },
                });
            }
        }

        self.selected_symbol = None;
        None
    }

    /// Get all occurrences of selected symbol
    pub fn find_symbol_occurrences(&self) -> Vec<Range> {
        let Some(symbol) = &self.selected_symbol else {
            return Vec::new();
        };

        self.tokens
            .iter()
            .filter(|t| &t.text == symbol && t.token_type.supports_goto())
            .map(|t| Range {
                start: Position {
                    line: t.line as u32,
                    character: t.column as u32,
                },
                end: Position {
                    line: t.line as u32,
                    character: (t.column + t.text.len()) as u32,
                },
            })
            .collect()
    }

    /// Get tokens
    pub fn tokens(&self) -> &[CodeToken] {
        &self.tokens
    }

    /// Get code
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Get language
    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    /// Get config
    pub fn config(&self) -> &CodeLspConfig {
        &self.config
    }

    /// Check if symbol is selected
    pub fn selected_symbol(&self) -> Option<&str> {
        self.selected_symbol.as_deref()
    }
}
