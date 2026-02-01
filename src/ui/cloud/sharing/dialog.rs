//! Share dialog implementation

use std::sync::Arc;

use gpui::*;

use crate::app::state::AppState;
use super::types::*;

/// Share dialog
pub struct ShareDialog {
    pub(super) app_state: Arc<AppState>,
    /// Conversation ID being shared
    pub(super) conversation_id: String,
    /// Conversation title
    pub(super) conversation_title: String,
    /// Existing share links
    pub(super) existing_links: Vec<ShareLink>,
    /// Selected permission
    pub(super) selected_permission: SharePermission,
    /// Password input
    pub(super) password_input: String,
    /// Use password protection
    pub(super) use_password: bool,
    /// Selected expiry option
    pub(super) expiry_option: ExpiryOption,
    /// Link just copied (for feedback)
    pub(super) copied_link_id: Option<String>,
    /// Is generating
    pub(super) is_generating: bool,
    /// Error message
    pub(super) error_message: Option<String>,
    /// Focus handle
    pub(super) focus_handle: FocusHandle,
}

impl ShareDialog {
    /// Create a new share dialog
    pub fn new(
        app_state: Arc<AppState>,
        conversation_id: String,
        conversation_title: String,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            app_state,
            conversation_id,
            conversation_title,
            existing_links: Vec::new(),
            selected_permission: SharePermission::View,
            password_input: String::new(),
            use_password: false,
            expiry_option: ExpiryOption::OneWeek,
            copied_link_id: None,
            is_generating: false,
            error_message: None,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Set existing links
    pub fn set_existing_links(&mut self, links: Vec<ShareLink>, cx: &mut Context<Self>) {
        self.existing_links = links;
        cx.notify();
    }

    /// Add a new link
    pub fn add_link(&mut self, link: ShareLink, cx: &mut Context<Self>) {
        self.existing_links.push(link);
        self.is_generating = false;
        cx.notify();
    }

    /// Remove a link
    pub fn remove_link(&mut self, link_id: &str, cx: &mut Context<Self>) {
        self.existing_links.retain(|l| l.id != link_id);
        cx.notify();
    }

    /// Set permission
    pub fn set_permission(&mut self, permission: SharePermission, cx: &mut Context<Self>) {
        self.selected_permission = permission;
        cx.notify();
    }

    /// Set password
    pub fn set_password(&mut self, password: String, cx: &mut Context<Self>) {
        self.password_input = password;
        cx.notify();
    }

    /// Toggle password protection
    pub fn toggle_password(&mut self, cx: &mut Context<Self>) {
        self.use_password = !self.use_password;
        if !self.use_password {
            self.password_input.clear();
        }
        cx.notify();
    }

    /// Set expiry option
    pub fn set_expiry(&mut self, expiry: ExpiryOption, cx: &mut Context<Self>) {
        self.expiry_option = expiry;
        cx.notify();
    }

    /// Mark link as copied
    pub fn mark_copied(&mut self, link_id: String, cx: &mut Context<Self>) {
        self.copied_link_id = Some(link_id);
        // Clear after delay (would need async handling)
        cx.notify();
    }

    /// Set generating state
    pub fn set_generating(&mut self, generating: bool, cx: &mut Context<Self>) {
        self.is_generating = generating;
        cx.notify();
    }

    /// Set error
    pub fn set_error(&mut self, message: Option<String>, cx: &mut Context<Self>) {
        self.error_message = message;
        cx.notify();
    }

    /// Generate a new link
    pub(super) fn generate_link(&mut self, cx: &mut Context<Self>) {
        self.is_generating = true;
        self.error_message = None;

        let password = if self.use_password && !self.password_input.is_empty() {
            Some(self.password_input.clone())
        } else {
            None
        };

        cx.emit(ShareDialogEvent::GenerateLink {
            conversation_id: self.conversation_id.clone(),
            permission: self.selected_permission,
            password,
            expires_in: self.expiry_option.to_duration(),
        });

        cx.notify();
    }
}

impl EventEmitter<ShareDialogEvent> for ShareDialog {}

impl Focusable for ShareDialog {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
