//! Extension API bindings for WASM plugins
//!
//! This module defines the interface that plugins can use to interact
//! with Claude Visual. It provides host functions that are exposed to
//! WASM modules through the wasmtime linker.

mod types;
mod ui;
mod fs;
mod settings;
mod events;
mod core;

// Re-export all public types and functions
pub use types::{
    ExtensionContext,
    ApiResult,
    Notification,
    NotificationLevel,
    StatusItem,
    EventSubscription,
};

pub use ui::UiApi;
pub use fs::FileSystemApi;
pub use settings::SettingsApi;
pub use events::EventApi;
pub use core::{API_VERSION, ExtensionApi};

/// Available events that extensions can subscribe to
pub mod event_names {
    pub use super::events::constants::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_api_notifications() {
        let ui = UiApi::new();

        // Show a notification
        let result = ui.show_notification("test-ext", "Test", Some("Body"), NotificationLevel::Info);
        assert!(result.is_ok());

        // Check notifications exist
        let notifications = ui.drain_notifications();
        assert_eq!(notifications.len(), 1);
        assert_eq!(notifications[0].title, "Test");
        assert_eq!(notifications[0].extension_id, "test-ext");

        // Notifications should be drained
        let notifications = ui.drain_notifications();
        assert!(notifications.is_empty());
    }

    #[test]
    fn test_ui_api_status_items() {
        let ui = UiApi::new();

        // Set a status item
        let result = ui.set_status_item("test-ext", "status-1", "Hello", Some("Tooltip"));
        assert!(result.is_ok());

        // Get status items
        let items = ui.get_status_items();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].text, "Hello");

        // Remove status item
        let result = ui.remove_status_item("test-ext", "status-1");
        assert!(result.is_ok());

        // Should be empty now
        let items = ui.get_status_items();
        assert!(items.is_empty());
    }

    #[test]
    fn test_settings_api() {
        let settings = SettingsApi::new();

        // Set a value
        let result = settings.set("test-ext", "key1", "value1");
        assert!(result.is_ok());

        // Get the value
        let result = settings.get("test-ext", "key1");
        assert!(result.is_ok());
        assert_eq!(result.get_data(), Some("value1"));

        // Get non-existent value
        let result = settings.get("test-ext", "key2");
        assert!(!result.is_ok());
    }

    #[test]
    fn test_event_api() {
        let events = EventApi::new();

        // Subscribe to an event
        let result = events.subscribe("test-ext", "test.event", 0);
        assert!(result.is_ok());
        let subscription_id = result.get_data().unwrap().to_string();

        // Get subscriptions
        let subs = events.get_subscriptions("test.event");
        assert_eq!(subs.len(), 1);

        // Unsubscribe
        let result = events.unsubscribe("test-ext", &subscription_id);
        assert!(result.is_ok());

        // Should be empty now
        let subs = events.get_subscriptions("test.event");
        assert!(subs.is_empty());
    }

    #[test]
    fn test_extension_api_cleanup() {
        let api = ExtensionApi::new();

        // Add some resources
        api.ui.show_notification("test-ext", "Test", None, NotificationLevel::Info);
        api.ui.set_status_item("test-ext", "status-1", "Hello", None);
        api.events.subscribe("test-ext", "test.event", 0);

        // Cleanup
        api.cleanup_extension("test-ext");

        // All should be removed
        assert!(api.ui.drain_notifications().is_empty());
        assert!(api.ui.get_status_items().is_empty());
        assert!(api.events.get_subscriptions("test.event").is_empty());
    }
}
