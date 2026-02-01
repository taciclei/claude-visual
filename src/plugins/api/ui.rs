//! UI API for extensions to interact with the user interface

use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

use super::types::{ApiResult, Notification, NotificationLevel, StatusItem};

/// UI API for extensions to interact with the user interface
pub struct UiApi {
    /// Pending notifications to show
    pub(crate) notifications: Arc<RwLock<Vec<Notification>>>,
    /// Registered status items
    pub(crate) status_items: Arc<RwLock<HashMap<String, StatusItem>>>,
}

impl Default for UiApi {
    fn default() -> Self {
        Self::new()
    }
}

impl UiApi {
    /// Create a new UI API instance
    pub fn new() -> Self {
        Self {
            notifications: Arc::new(RwLock::new(Vec::new())),
            status_items: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Show a notification to the user
    pub fn show_notification(
        &self,
        extension_id: &str,
        title: &str,
        body: Option<&str>,
        level: NotificationLevel,
    ) -> ApiResult {
        let id = uuid::Uuid::new_v4().to_string();
        let notification = Notification {
            id: id.clone(),
            title: title.to_string(),
            body: body.map(|s| s.to_string()),
            level,
            extension_id: extension_id.to_string(),
        };

        self.notifications.write().push(notification);
        ApiResult::data(id)
    }

    /// Dismiss a notification
    pub fn dismiss_notification(&self, extension_id: &str, notification_id: &str) -> ApiResult {
        let mut notifications = self.notifications.write();
        let initial_len = notifications.len();
        notifications.retain(|n| !(n.id == notification_id && n.extension_id == extension_id));

        if notifications.len() < initial_len {
            ApiResult::ok()
        } else {
            ApiResult::error("Notification not found")
        }
    }

    /// Set a status bar item
    pub fn set_status_item(
        &self,
        extension_id: &str,
        id: &str,
        text: &str,
        tooltip: Option<&str>,
    ) -> ApiResult {
        let item = StatusItem {
            id: id.to_string(),
            text: text.to_string(),
            tooltip: tooltip.map(|s| s.to_string()),
            icon: None,
            extension_id: extension_id.to_string(),
        };

        self.status_items.write().insert(id.to_string(), item);
        ApiResult::ok()
    }

    /// Remove a status bar item
    pub fn remove_status_item(&self, extension_id: &str, id: &str) -> ApiResult {
        let mut items = self.status_items.write();
        if let Some(item) = items.get(id) {
            if item.extension_id == extension_id {
                items.remove(id);
                return ApiResult::ok();
            }
        }
        ApiResult::error("Status item not found or owned by another extension")
    }

    /// Get pending notifications and clear them
    pub fn drain_notifications(&self) -> Vec<Notification> {
        std::mem::take(&mut *self.notifications.write())
    }

    /// Get all status items
    pub fn get_status_items(&self) -> Vec<StatusItem> {
        self.status_items.read().values().cloned().collect()
    }
}
