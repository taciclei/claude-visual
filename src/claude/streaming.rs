//! Async streaming handler for Claude responses

use std::pin::Pin;
use std::task::{Context, Poll};

use futures::Stream;
use tokio::sync::mpsc;

use super::message::ClaudeEvent;

/// A streaming response from Claude
pub struct ClaudeStream {
    receiver: mpsc::Receiver<ClaudeEvent>,
}

impl ClaudeStream {
    /// Create a new stream from a receiver
    pub fn new(receiver: mpsc::Receiver<ClaudeEvent>) -> Self {
        Self { receiver }
    }

    /// Create a stream and sender pair
    pub fn channel(buffer: usize) -> (ClaudeStreamSender, Self) {
        let (tx, rx) = mpsc::channel(buffer);
        (ClaudeStreamSender { sender: tx }, Self::new(rx))
    }
}

impl Stream for ClaudeStream {
    type Item = ClaudeEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.receiver).poll_recv(cx)
    }
}

/// Sender for Claude stream events
pub struct ClaudeStreamSender {
    sender: mpsc::Sender<ClaudeEvent>,
}

impl ClaudeStreamSender {
    /// Send an event to the stream
    pub async fn send(
        &self,
        event: ClaudeEvent,
    ) -> Result<(), mpsc::error::SendError<ClaudeEvent>> {
        self.sender.send(event).await
    }

    /// Try to send an event without blocking
    pub fn try_send(
        &self,
        event: ClaudeEvent,
    ) -> Result<(), mpsc::error::TrySendError<ClaudeEvent>> {
        self.sender.try_send(event)
    }

    /// Check if the receiver is closed
    pub fn is_closed(&self) -> bool {
        self.sender.is_closed()
    }
}

/// Accumulator for building up a complete message from streaming events
#[derive(Debug, Default)]
pub struct MessageAccumulator {
    content: String,
    tool_uses: Vec<ToolUseAccumulator>,
    current_tool: Option<ToolUseAccumulator>,
}

#[derive(Debug, Clone)]
pub struct ToolUseAccumulator {
    pub name: String,
    pub input: String,
}

impl MessageAccumulator {
    /// Create a new accumulator
    pub fn new() -> Self {
        Self::default()
    }

    /// Process an event
    pub fn process(&mut self, event: &ClaudeEvent) {
        match event {
            ClaudeEvent::ContentBlockDelta { delta } => {
                if let Some(ref mut tool) = self.current_tool {
                    tool.input.push_str(delta);
                } else {
                    self.content.push_str(delta);
                }
            }
            ClaudeEvent::ToolUse { name, input: _ } => {
                // Start a new tool use
                self.current_tool = Some(ToolUseAccumulator {
                    name: name.clone(),
                    input: String::new(),
                });
            }
            ClaudeEvent::ToolResult { .. } => {
                // End current tool use
                if let Some(tool) = self.current_tool.take() {
                    self.tool_uses.push(tool);
                }
            }
            _ => {}
        }
    }

    /// Get the accumulated content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Get the tool uses
    pub fn tool_uses(&self) -> &[ToolUseAccumulator] {
        &self.tool_uses
    }

    /// Reset the accumulator
    pub fn reset(&mut self) {
        self.content.clear();
        self.tool_uses.clear();
        self.current_tool = None;
    }
}
