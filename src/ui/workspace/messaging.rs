//! Claude messaging functionality

use super::core::Workspace;
use crate::claude::client::PromptOptions;
use crate::claude::message::{ClaudeEvent, ClaudeMessage};
use gpui::*;
use tokio::sync::mpsc;

impl Workspace {
    /// Cancel the current streaming request
    pub(in crate::ui::workspace) fn cancel_streaming(&mut self, cx: &mut Context<Self>) {
        if let Some(sender) = self.cancel_sender.take() {
            let _ = sender.try_send(());
            tracing::info!("Cancelled streaming request");
        }

        // Reset chat view streaming state
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |chat, cx| {
                chat.handle_claude_event(
                    ClaudeEvent::Error {
                        message: "Response cancelled by user".to_string(),
                    },
                    cx,
                );
            });
        }
    }

    /// Send a message to Claude
    pub(in crate::ui::workspace) fn send_message(
        &mut self,
        message: String,
        cx: &mut Context<Self>,
    ) {
        tracing::info!("Workspace::send_message called with: '{}'", message);
        let cwd = self.app_state.current_directory();
        tracing::info!("Using cwd: {:?}", cwd);

        // Cancel any existing request
        if let Some(sender) = self.cancel_sender.take() {
            let _ = sender.try_send(());
        }

        // Create cancellation channel
        let (cancel_tx, mut cancel_rx) = mpsc::channel(1);
        self.cancel_sender = Some(cancel_tx);

        // Get prompt options from chat view (think mode, model, session ID)
        let prompt_options = self
            .chat_views
            .get(self.active_chat_index)
            .map(|view| {
                let chat = view.read(cx);
                PromptOptions {
                    think_mode: chat.is_think_mode_enabled(),
                    model: chat.get_current_model().map(|m| m.id.clone()),
                    session_id: chat.current_session_id(),
                }
            })
            .unwrap_or_default();

        // Log the options being used
        if prompt_options.think_mode {
            tracing::info!("Think mode enabled for this request");
        }
        if let Some(ref model) = prompt_options.model {
            tracing::info!("Using model: {}", model);
        }

        // Legacy: Get session ID (now included in prompt_options)
        let session_id = prompt_options.session_id.clone();

        if session_id.is_some() {
            tracing::info!("Continuing session: {:?}", session_id);
        }

        // Add user message to chat
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |chat, cx| {
                chat.add_message(ClaudeMessage::user(message.clone()), cx);
            });

            // Update tab title with message preview
            let title_preview = if message.len() > 25 {
                format!("{}...", &message[..22])
            } else {
                message.clone()
            };
            self.tab_bar.update(cx, |bar, cx| {
                bar.update_active_title(title_preview, cx);
            });
        }

        // Update status bar (streaming started)
        self.update_status_bar(cx);

        // Spawn Claude process
        let client = self.claude_client.clone();
        let active_index = self.active_chat_index;
        cx.spawn(async move |this, cx| {
            match client
                .send_prompt_with_options(&message, cwd.as_deref(), prompt_options)
                .await
            {
                Ok(mut stream) => {
                    use futures::StreamExt;
                    // Use futures::select instead of tokio::select for GPUI compatibility
                    loop {
                        // Check for cancellation (non-blocking)
                        match cancel_rx.try_recv() {
                            Ok(_) | Err(mpsc::error::TryRecvError::Disconnected) => {
                                tracing::info!("Stream cancelled");
                                break;
                            }
                            Err(mpsc::error::TryRecvError::Empty) => {
                                // Continue processing stream
                            }
                        }

                        match stream.next().await {
                            Some(event) => {
                                tracing::info!("Workspace received Claude event: {:?}", event);
                                let should_break = matches!(
                                    event,
                                    ClaudeEvent::AssistantEnd | ClaudeEvent::Error { .. }
                                );
                                let _ = this.update(cx, |workspace, cx| {
                                    if let Some(chat_view) = workspace.chat_views.get(active_index)
                                    {
                                        chat_view.update(cx, |chat, cx| {
                                            chat.handle_claude_event(event, cx);
                                        });
                                    }
                                });
                                if should_break {
                                    break;
                                }
                            }
                            None => break,
                        }
                    }
                    // Clear the cancel sender when done and update status bar
                    let _ = this.update(cx, |workspace, cx| {
                        workspace.cancel_sender = None;
                        workspace.update_status_bar(cx);
                    });
                }
                Err(e) => {
                    tracing::error!("Failed to send message to Claude: {}", e);
                    let _ = this.update(cx, |workspace, cx| {
                        workspace.cancel_sender = None;
                        if let Some(chat_view) = workspace.chat_views.get(active_index) {
                            chat_view.update(cx, |chat, cx| {
                                chat.add_message(ClaudeMessage::error(format!("Error: {}", e)), cx);
                            });
                        }
                        // Update status bar (streaming ended with error)
                        workspace.update_status_bar(cx);
                    });
                }
            }
        })
        .detach();
    }
}
