//! Permission management for ChatView
//!
//! This module contains permission-related methods extracted from core.rs.
//! Handles permission requests from Claude CLI and user approval/denial.

use gpui::*;
use super::types::{PermissionRequest, NotificationType, ChatViewEvent};
use super::core::ChatView;

impl ChatView {
    // ==================== Permissions ====================

    /// Toggle permissions panel
    pub fn toggle_permissions_panel(&mut self, cx: &mut Context<Self>) {
        self.panels.permissions_panel = !self.panels.permissions_panel;
        cx.notify();
    }

    /// Add a pending permission request
    pub fn add_permission_request(&mut self, request: PermissionRequest, cx: &mut Context<Self>) {
        // Auto-open panel if this is first permission request
        if self.pending_permissions.is_empty() {
            self.panels.permissions_panel = true;
        }
        self.pending_permissions.push(request);
        cx.notify();
    }

    /// Create a permission request from a Claude event
    pub fn handle_permission_event(
        &mut self,
        request_id: String,
        tool: String,
        action: String,
        command: Option<String>,
        cx: &mut Context<Self>,
    ) {
        let request = PermissionRequest::from_event(request_id, tool, action, command);
        self.add_permission_request(request, cx);
    }

    /// Approve a permission request
    pub fn approve_permission(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.pending_permissions.len() {
            let request = self.pending_permissions.remove(index);

            // Emit permission response event to workspace
            cx.emit(ChatViewEvent::PermissionResponse {
                request_id: request.request_id.clone(),
                granted: true,
            });

            self.show_notification(
                &format!("Approved: {} - {}", request.tool, request.action),
                NotificationType::Success,
                cx
            );

            // Close panel if no more requests
            if self.pending_permissions.is_empty() {
                self.panels.permissions_panel = false;
            }
            cx.notify();
        }
    }

    /// Deny a permission request
    pub fn deny_permission(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.pending_permissions.len() {
            let request = self.pending_permissions.remove(index);

            // Emit permission response event to workspace
            cx.emit(ChatViewEvent::PermissionResponse {
                request_id: request.request_id.clone(),
                granted: false,
            });

            self.show_notification(
                &format!("Denied: {} - {}", request.tool, request.action),
                NotificationType::Info,
                cx
            );

            // Close panel if no more requests
            if self.pending_permissions.is_empty() {
                self.panels.permissions_panel = false;
            }
            cx.notify();
        }
    }

    /// Approve all pending permission requests
    pub fn approve_all_permissions(&mut self, cx: &mut Context<Self>) {
        let requests: Vec<_> = self.pending_permissions.drain(..).collect();
        let count = requests.len();

        if count > 0 {
            // Emit response for each request
            for request in requests {
                cx.emit(ChatViewEvent::PermissionResponse {
                    request_id: request.request_id,
                    granted: true,
                });
            }

            self.show_notification(
                &format!("Approved {} permission{}", count, if count == 1 { "" } else { "s" }),
                NotificationType::Success,
                cx
            );

            self.panels.permissions_panel = false;
            cx.notify();
        }
    }

    /// Deny all pending permission requests
    pub fn deny_all_permissions(&mut self, cx: &mut Context<Self>) {
        let requests: Vec<_> = self.pending_permissions.drain(..).collect();
        let count = requests.len();

        if count > 0 {
            // Emit response for each request
            for request in requests {
                cx.emit(ChatViewEvent::PermissionResponse {
                    request_id: request.request_id,
                    granted: false,
                });
            }

            self.show_notification(
                &format!("Denied {} permission{}", count, if count == 1 { "" } else { "s" }),
                NotificationType::Info,
                cx
            );

            self.panels.permissions_panel = false;
            cx.notify();
        }
    }

    /// Check if there are pending permissions
    pub fn has_pending_permissions(&self) -> bool {
        !self.pending_permissions.is_empty()
    }

    /// Get pending permission count
    pub fn pending_permission_count(&self) -> usize {
        self.pending_permissions.len()
    }

    /// Check if there are high-risk pending permissions
    pub fn has_high_risk_permissions(&self) -> bool {
        use super::types::PermissionRisk;
        self.pending_permissions.iter().any(|p| p.risk_level == PermissionRisk::High)
    }
}
