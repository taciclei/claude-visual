//! Streaming metrics and message view creation utilities

use gpui::*;

use crate::claude::message::ClaudeMessage;
use crate::ui::chat::message::{MessageView, MessageViewEvent};

use super::core::ChatView;
use super::types::{ChatViewEvent, NotificationType};

impl ChatView {
    /// Format streaming speed for display
    pub fn format_streaming_speed(&self) -> String {
        if self.streaming.last_speed > 0.0 {
            format!("{:.1} tok/s", self.streaming.last_speed)
        } else {
            "-".to_string()
        }
    }

    /// Get streaming dots animation text
    pub(crate) fn get_streaming_dots(&self) -> &'static str {
        match self.streaming.streaming_dots % 4 {
            0 => "   ",
            1 => ".  ",
            2 => ".. ",
            _ => "...",
        }
    }

    /// Get an icon for a tool name
    pub(crate) fn get_tool_icon(tool_name: &str) -> &'static str {
        match tool_name.to_lowercase().as_str() {
            "read" => "📖",
            "write" => "✍️",
            "edit" => "✏️",
            "bash" => "💻",
            "glob" => "📁",
            "grep" => "🔍",
            "websearch" | "web_search" => "🌐",
            "webfetch" | "web_fetch" => "🌍",
            "task" => "🤖",
            "notebookedit" | "notebook_edit" => "📓",
            "skill" => "⚡",
            _ => "🔧",
        }
    }

    /// Create a new message view entity with event subscriptions
    pub(crate) fn create_message_view(
        &self,
        message: ClaudeMessage,
        cx: &mut Context<Self>,
    ) -> Entity<MessageView> {
        let app_state = self.app_state.clone();
        let view = cx.new(|cx| MessageView::new(message, app_state, cx));

        // Subscribe to message view events
        cx.subscribe(&view, |this, _, event: &MessageViewEvent, cx| {
            match event {
                MessageViewEvent::RerunCommand(cmd) => {
                    // Insert command as a prompt suggestion
                    let prompt = format!("Run this command: {}", cmd);
                    this.input.update(cx, |input, cx| {
                        input.clear(cx);
                        input.insert_text(&prompt, cx);
                    });
                    this.show_notification("Command ready to rerun", NotificationType::Info, cx);
                }
                MessageViewEvent::OpenFile(path) => {
                    cx.emit(ChatViewEvent::OpenFile(path.clone()));
                }
                MessageViewEvent::RegenerateResponse => {
                    this.regenerate_last_response(cx);
                }
                MessageViewEvent::RetryFromHere => {
                    this.regenerate_last_response(cx);
                }
                MessageViewEvent::Quote(content) => {
                    // Insert quoted content into input
                    let quoted =
                        format!("> {}\n\n", content.lines().collect::<Vec<_>>().join("\n> "));
                    this.input.update(cx, |input, cx| {
                        input.insert_text(&quoted, cx);
                    });
                }
                MessageViewEvent::Edit(content) => {
                    // Replace input with message content for editing
                    this.input.update(cx, |input, cx| {
                        input.clear(cx);
                        input.insert_text(content, cx);
                    });
                }
                MessageViewEvent::ExecuteSkill(cmd) => {
                    // Execute skill from error recovery suggestion
                    if cmd.starts_with('/') || cmd == "new_conversation" {
                        cx.emit(ChatViewEvent::Submit(cmd.clone()));
                    } else {
                        // For non-slash commands, set in input
                        this.input.update(cx, |input, cx| {
                            input.clear(cx);
                            input.insert_text(cmd, cx);
                        });
                    }
                }
                // Other events don't need special handling here
                _ => {}
            }
        })
        .detach();

        view
    }
}
