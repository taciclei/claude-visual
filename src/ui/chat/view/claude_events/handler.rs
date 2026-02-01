//! Main Claude event handler

use gpui::*;

use crate::claude::message::{ClaudeEvent, ClaudeMessage, MessageRole};
use crate::ui::chat::view::{ActiveTask, ChatView, ConnectionStatus, NotificationType};

impl ChatView {
    /// Handle a Claude event from the stream
    pub fn handle_claude_event(&mut self, event: ClaudeEvent, cx: &mut Context<Self>) {
        match event {
            ClaudeEvent::AssistantStart => {
                self.streaming.is_streaming = true;
                self.streaming.streaming_dots = 0;
                self.connection_status = ConnectionStatus::Connected;
                self.streaming.current_message = Some(String::new());
                self.current_thinking = None;
                // Track response start time for latency measurement
                self.streaming.response_start_time = Some(chrono::Utc::now());
                // Reset streaming metrics for new response
                self.reset_streaming_metrics();
                self.stats.total_api_requests += 1;
                // Disable input during streaming
                self.input.update(cx, |input, cx| {
                    input.set_disabled(true, cx);
                });
                // Create a streaming message view
                let streaming_msg = ClaudeMessage {
                    role: MessageRole::Assistant,
                    content: String::new(),
                    timestamp: chrono::Utc::now(),
                    tool_name: None,
                    is_error: false,
                };
                self.streaming_message_view = Some(self.create_message_view(streaming_msg, cx));
                // Start animation
                self.start_streaming_animation(cx);
                cx.notify();
            }
            ClaudeEvent::ContentBlockDelta { delta } => {
                // Estimate tokens for streaming speed (before mutable borrow)
                let estimated_new_tokens = (delta.len() as f64 / 3.0).ceil() as usize;

                // Update the current message content
                if let Some(ref mut msg) = self.streaming.current_message {
                    msg.push_str(&delta);
                }

                // Update streaming speed
                self.update_streaming_speed(estimated_new_tokens);

                // Update existing streaming view or create new one
                if let Some(ref view) = self.streaming_message_view {
                    // Update existing view's content (more efficient than recreating)
                    if let Some(ref content) = self.streaming.current_message {
                        let content_clone = content.clone();
                        view.update(cx, |v, cx| {
                            v.update_content(content_clone, cx);
                        });
                    }
                } else if let Some(ref content) = self.streaming.current_message {
                    // Create new view only if one doesn't exist
                    let streaming_msg = ClaudeMessage {
                        role: MessageRole::Assistant,
                        content: content.clone(),
                        timestamp: chrono::Utc::now(),
                        tool_name: None,
                        is_error: false,
                    };
                    self.streaming_message_view = Some(self.create_message_view(streaming_msg, cx));
                }
                cx.notify();
            }
            ClaudeEvent::AssistantEnd => {
                self.streaming.is_streaming = false;
                self.streaming_message_view = None;
                // Clear current state trackers
                self.current_tool_name = None;
                self.current_thinking = None;
                // Re-enable input
                self.input.update(cx, |input, cx| {
                    input.set_disabled(false, cx);
                });
                if let Some(content) = self.streaming.current_message.take() {
                    let message = ClaudeMessage::assistant(content);
                    self.save_message(&message);
                    let view = self.create_message_view(message.clone(), cx);
                    self.message_views.push(view);
                    self.messages.push(message);
                }
                // Clear any previous error on success
                self.last_error = None;
                // Reset connection retry count on success
                self.stats.connection_retry_count = 0;
                // Track response latency
                if let Some(start) = self.streaming.response_start_time.take() {
                    let latency = chrono::Utc::now()
                        .signed_duration_since(start)
                        .num_milliseconds() as u64;
                    self.update_response_latency(latency, cx);
                }
                // Update contextual suggestions based on new message
                self.update_suggestions(cx);
                // Generate quick reply suggestions
                self.generate_quick_reply_suggestions(cx);
                // Calculate session health
                self.calculate_session_health(cx);
                // Auto-generate title after first response if none exists
                if self.conversation_title.is_none() && self.messages.len() <= 3 {
                    self.auto_generate_title(cx);
                }
                cx.notify();
            }
            ClaudeEvent::ToolUse { name, input } => {
                // Track current tool for display
                self.current_tool_name = Some(name.clone());
                let message = ClaudeMessage::tool_use(name, input);
                self.save_message(&message);
                let view = self.create_message_view(message.clone(), cx);
                self.message_views.push(view);
                self.messages.push(message);
                cx.notify();
            }
            ClaudeEvent::ToolResult { output, is_error } => {
                // Clear current tool (tool execution finished)
                self.current_tool_name = None;
                let message = ClaudeMessage::tool_result(output, is_error);
                self.save_message(&message);
                let view = self.create_message_view(message.clone(), cx);
                self.message_views.push(view);
                self.messages.push(message);
                cx.notify();
            }
            ClaudeEvent::Error { message: msg } => {
                self.streaming.is_streaming = false;
                self.connection_status = ConnectionStatus::Error;
                self.streaming_message_view = None;
                // Re-enable input on error
                self.input.update(cx, |input, cx| {
                    input.set_disabled(false, cx);
                });
                // Record error for potential retry
                // Try to get the last user prompt for retry
                let original_prompt = self
                    .messages
                    .iter()
                    .rev()
                    .find(|m| m.role == MessageRole::User)
                    .map(|m| m.content.clone());
                self.record_error(msg.clone(), original_prompt, cx);

                let message = ClaudeMessage::error(msg);
                self.save_message(&message);
                let view = self.create_message_view(message.clone(), cx);
                self.message_views.push(view);
                self.messages.push(message);
                // Update suggestions (will show error-related suggestions)
                self.update_suggestions(cx);
                cx.notify();
            }
            ClaudeEvent::Thinking { content } => {
                // Display thinking content as a collapsible message
                tracing::debug!("Received thinking: {} chars", content.len());
                if !content.trim().is_empty() {
                    // Store current thinking for display
                    self.current_thinking = Some(content.clone());

                    // Only add to messages if show_thinking is enabled
                    if self.show_thinking {
                        let message = ClaudeMessage::thinking(content);
                        let view = self.create_message_view(message.clone(), cx);
                        // Auto-collapse thinking messages
                        view.update(cx, |v, cx| v.set_collapsed(true, cx));
                        self.message_views.push(view);
                        self.messages.push(message);
                    }
                }
                cx.notify();
            }
            ClaudeEvent::Usage {
                input_tokens,
                output_tokens,
                cost_usd,
            } => {
                // Update session stats
                self.stats.input_tokens += input_tokens;
                self.stats.output_tokens += output_tokens;
                if let Some(cost) = cost_usd {
                    self.stats.cost += cost;
                }
                // Update context usage (total tokens used in conversation)
                self.context_used = self.stats.input_tokens + self.stats.output_tokens;

                // Check if context is getting full and show notification
                let usage_pct = self.context_usage_percentage();
                if usage_pct > 0.90 && usage_pct <= 0.95 {
                    self.show_notification(
                        "Context is 90% full. Consider using /compact to free up space.",
                        NotificationType::Warning,
                        cx,
                    );
                } else if usage_pct > 0.95 {
                    self.show_notification(
                        "Context is almost full! Use /compact now to avoid errors.",
                        NotificationType::Error,
                        cx,
                    );
                }

                tracing::info!(
                    "Usage: {} input, {} output tokens, cost: ${:.4}, context: {:.1}%",
                    input_tokens,
                    output_tokens,
                    cost_usd.unwrap_or(0.0),
                    usage_pct * 100.0
                );
                cx.notify();
            }
            ClaudeEvent::TaskStarted {
                description,
                task_id,
            } => {
                tracing::info!("Task started: {} (id: {:?})", description, task_id);
                // Add to active tasks
                let was_empty = self.active_tasks.is_empty();
                self.active_tasks.push(ActiveTask {
                    task_id: task_id.clone(),
                    description: description.clone(),
                    started_at: chrono::Utc::now(),
                    progress: None,
                    status: None,
                });
                // Start animation if this is the first task
                if was_empty {
                    self.start_streaming_animation(cx);
                }
                cx.notify();
            }
            ClaudeEvent::TaskCompleted { task_id, result } => {
                tracing::info!("Task completed (id: {:?}): {} chars", task_id, result.len());
                // Remove from active tasks
                if let Some(ref id) = task_id {
                    self.active_tasks.retain(|t| t.task_id.as_ref() != Some(id));
                } else {
                    // If no task_id, remove the first task
                    if !self.active_tasks.is_empty() {
                        self.active_tasks.remove(0);
                    }
                }
                cx.notify();
            }
            ClaudeEvent::SystemInit { info } => {
                tracing::info!(
                    "Session initialized: model={}, {} tools, {} commands",
                    info.model,
                    info.tools.len(),
                    info.slash_commands.len()
                );
                // Update input with available commands for autocomplete
                let commands = info.slash_commands.clone();
                self.input.update(cx, |input, cx| {
                    input.set_available_commands(commands, cx);
                });
                // Update available models to mark the current one
                let current_model = info.model.clone();
                for model in &mut self.available_models {
                    model.is_current =
                        current_model.contains(&model.name) || model.id == current_model;
                }
                // Update connection status to connected
                self.connection_status = ConnectionStatus::Connected;

                // Show session capabilities notification
                let capabilities = self.format_session_capabilities(&info);
                self.show_notification(capabilities, NotificationType::Success, cx);

                self.session_info = Some(info);
                cx.notify();
            }
            ClaudeEvent::PermissionRequest {
                request_id,
                tool,
                action,
                command,
            } => {
                tracing::info!(
                    "Permission requested: {} - {} (id: {})",
                    tool,
                    action,
                    request_id
                );
                // Create and add permission request to pending list
                self.handle_permission_event(request_id, tool, action, command, cx);
                cx.notify();
            }
            ClaudeEvent::PermissionResponse {
                request_id,
                granted,
            } => {
                // This is an acknowledgement from the CLI that our response was received
                tracing::debug!(
                    "Permission response acknowledged: {} = {}",
                    request_id,
                    granted
                );
            }
        }
    }
}
