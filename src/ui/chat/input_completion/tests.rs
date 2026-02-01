//! Tests for input completion

use super::utils::fuzzy_match;
use super::*;

#[test]
fn test_fuzzy_match() {
    assert!(fuzzy_match("hello", "").is_some());
    assert!(fuzzy_match("hello", "hel").is_some());
    assert!(fuzzy_match("hello", "hlo").is_some());
    assert!(fuzzy_match("hello", "xyz").is_none());
}

#[test]
fn test_completion_trigger() {
    assert_eq!(
        CompletionTrigger::from_char('@'),
        Some(CompletionTrigger::Mention)
    );
    assert_eq!(
        CompletionTrigger::from_char('/'),
        Some(CompletionTrigger::Command)
    );
    assert_eq!(CompletionTrigger::from_char('a'), None);
}

#[test]
fn test_command_completion() {
    let item = ChatCompletionItem::command("help", "Show help");
    assert_eq!(item.label, "/help");
    assert_eq!(item.kind, ChatCompletionKind::Command);
}

#[test]
fn test_completion_manager() {
    let mut manager = InputCompletionManager::new();

    // Type /he
    manager.handle_input("/he", 3);
    assert!(manager.is_active());

    let items = manager.items();
    assert!(!items.is_empty());
    assert!(items.iter().any(|i| i.label.contains("help")));
}

#[test]
fn test_selection_navigation() {
    let mut manager = InputCompletionManager::new();
    manager.handle_input("/", 1);

    let initial = manager.selected_index();
    manager.select_next();
    assert_ne!(manager.selected_index(), initial);

    manager.select_prev();
    assert_eq!(manager.selected_index(), initial);
}

#[test]
fn test_accept_completion() {
    let mut manager = InputCompletionManager::new();
    manager.handle_input("/help", 5);

    let result = manager.accept();
    assert!(result.is_some());

    let result = result.unwrap();
    assert!(result.insert_text.contains("help"));
    assert!(!manager.is_active());
}
