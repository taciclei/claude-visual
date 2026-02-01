//! Event API for extensions to subscribe to application events

use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

use super::types::{ApiResult, EventSubscription};

/// Event API for extensions to subscribe to application events
pub struct EventApi {
    /// Event subscribers (event_name -> callback_ids)
    subscribers: Arc<RwLock<HashMap<String, Vec<EventSubscription>>>>,
}

impl Default for EventApi {
    fn default() -> Self {
        Self::new()
    }
}

impl EventApi {
    /// Create a new event API instance
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Subscribe to an event
    pub fn subscribe(&self, extension_id: &str, event_name: &str, callback_index: u32) -> ApiResult {
        let id = uuid::Uuid::new_v4().to_string();
        let subscription = EventSubscription {
            id: id.clone(),
            extension_id: extension_id.to_string(),
            callback_index,
        };

        self.subscribers
            .write()
            .entry(event_name.to_string())
            .or_default()
            .push(subscription);

        ApiResult::data(id)
    }

    /// Unsubscribe from an event
    pub fn unsubscribe(&self, extension_id: &str, subscription_id: &str) -> ApiResult {
        let mut subscribers = self.subscribers.write();

        for subs in subscribers.values_mut() {
            let initial_len = subs.len();
            subs.retain(|s| !(s.id == subscription_id && s.extension_id == extension_id));
            if subs.len() < initial_len {
                return ApiResult::ok();
            }
        }

        ApiResult::error("Subscription not found")
    }

    /// Get subscriptions for an event
    pub fn get_subscriptions(&self, event_name: &str) -> Vec<EventSubscription> {
        self.subscribers
            .read()
            .get(event_name)
            .cloned()
            .unwrap_or_default()
    }

    /// Remove all subscriptions for an extension
    pub fn remove_extension_subscriptions(&self, extension_id: &str) {
        let mut subscribers = self.subscribers.write();
        for subs in subscribers.values_mut() {
            subs.retain(|s| s.extension_id != extension_id);
        }
    }
}

/// Available events that extensions can subscribe to
pub mod constants {
    /// Fired when a conversation is started
    pub const CONVERSATION_STARTED: &str = "conversation.started";
    /// Fired when a message is sent
    pub const MESSAGE_SENT: &str = "message.sent";
    /// Fired when a response is received
    pub const RESPONSE_RECEIVED: &str = "response.received";
    /// Fired when a project is opened
    pub const PROJECT_OPENED: &str = "project.opened";
    /// Fired when a project is closed
    pub const PROJECT_CLOSED: &str = "project.closed";
    /// Fired when the theme changes
    pub const THEME_CHANGED: &str = "theme.changed";
    /// Fired when settings are saved
    pub const SETTINGS_SAVED: &str = "settings.saved";
}
