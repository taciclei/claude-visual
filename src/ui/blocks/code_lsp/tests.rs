//! Tests for code block LSP integration

#[cfg(test)]
mod tests {
    use crate::ui::blocks::code_lsp::{CodeLspConfig, CodeLspIntegration, TokenType};

    #[test]
    fn test_tokenization() {
        let code = "fn main() {\n    let x = 42;\n}";
        let integration = CodeLspIntegration::new(
            code.to_string(),
            Some("rust".to_string()),
            CodeLspConfig::default(),
        );

        let tokens = integration.tokens();
        assert!(!tokens.is_empty());

        // Check 'fn' is a keyword
        let fn_token = tokens.iter().find(|t| t.text == "fn");
        assert!(fn_token.is_some());
        assert_eq!(fn_token.unwrap().token_type, TokenType::Keyword);

        // Check 'main' is an identifier
        let main_token = tokens.iter().find(|t| t.text == "main");
        assert!(main_token.is_some());
        assert_eq!(main_token.unwrap().token_type, TokenType::Identifier);
    }

    #[test]
    fn test_token_at_position() {
        let code = "let foo = bar;";
        let integration = CodeLspIntegration::new(
            code.to_string(),
            Some("rust".to_string()),
            CodeLspConfig::default(),
        );

        let token = integration.token_at_position(0, 4);
        assert!(token.is_some());
        assert_eq!(token.unwrap().text, "foo");
    }

    #[test]
    fn test_symbol_occurrences() {
        let code = "let x = x + x;";
        let mut integration = CodeLspIntegration::new(
            code.to_string(),
            Some("rust".to_string()),
            CodeLspConfig::default(),
        );

        // Select 'x'
        integration.handle_selection(0, 4, 0, 5);

        let occurrences = integration.find_symbol_occurrences();
        assert_eq!(occurrences.len(), 3); // x appears 3 times
    }

    #[test]
    fn test_config_presets() {
        let full = CodeLspConfig::full();
        assert!(full.enable_goto_definition);
        assert!(full.enable_hover);

        let goto_only = CodeLspConfig::goto_only();
        assert!(goto_only.enable_goto_definition);
        assert!(!goto_only.enable_hover);

        let minimal = CodeLspConfig::minimal();
        assert!(!minimal.enable_goto_definition);
        assert!(!minimal.enable_hover);
    }
}
