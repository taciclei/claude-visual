//! LSP Client tests

use std::path::PathBuf;

use super::LspClientConfig;

#[test]
fn test_config_rust_analyzer() {
    let config = LspClientConfig::rust_analyzer(PathBuf::from("/test"));
    assert_eq!(config.command, "rust-analyzer");
    assert!(config.root_uri.unwrap().contains("/test"));
}
