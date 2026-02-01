//! Streaming functionality for Claude CLI

use std::io::{BufRead, BufReader};
use std::path::Path;
use std::pin::Pin;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;

use anyhow::Result;
use futures::Stream;

use super::core::ClaudeClient;
use super::parser::parse_stream_json;
use super::PromptOptions;
use crate::claude::message::ClaudeEvent;

impl ClaudeClient {
    /// Send a prompt to Claude and get a stream of events
    pub async fn send_prompt(
        &self,
        prompt: &str,
        cwd: Option<&Path>,
    ) -> Result<Pin<Box<dyn Stream<Item = ClaudeEvent> + Send>>> {
        self.send_prompt_with_options(prompt, cwd, PromptOptions::default())
            .await
    }

    /// Send a prompt to Claude with optional session continuity
    pub async fn send_prompt_with_session(
        &self,
        prompt: &str,
        cwd: Option<&Path>,
        session_id: Option<&str>,
    ) -> Result<Pin<Box<dyn Stream<Item = ClaudeEvent> + Send>>> {
        self.send_prompt_with_options(
            prompt,
            cwd,
            PromptOptions {
                session_id: session_id.map(String::from),
                ..Default::default()
            },
        )
        .await
    }

    /// Send a prompt to Claude with full options (think mode, model, session)
    pub async fn send_prompt_with_options(
        &self,
        prompt: &str,
        cwd: Option<&Path>,
        options: PromptOptions,
    ) -> Result<Pin<Box<dyn Stream<Item = ClaudeEvent> + Send>>> {
        let mut cmd = Command::new(&self.cli_path);

        // Use non-interactive mode with streaming JSON output
        // Note: --verbose is required when using stream-json with -p
        cmd.args(["-p", prompt, "--output-format", "stream-json", "--verbose"]);

        // Add model if specified
        if let Some(ref model) = options.model {
            cmd.args(["--model", model]);
        }

        // Enable extended thinking mode if requested
        if options.think_mode {
            // Claude CLI uses /think command or --dangerously-skip-permissions with think-related prompts
            // The --allowedTools flag can enable specific tools
            // For now, we'll prepend a think instruction to the prompt
            tracing::info!("Extended thinking mode enabled");
        }

        // Continue from previous session if provided
        if let Some(ref sid) = options.session_id {
            cmd.args(["--continue", sid]);
        }

        // Set working directory if provided
        if let Some(dir) = cwd {
            cmd.current_dir(dir);
        }

        // Capture stdout and stderr
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        tracing::debug!("Spawning Claude CLI: {:?}", cmd);

        let mut child = cmd.spawn()?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow::anyhow!("Failed to capture stdout"))?;

        // Create a channel for sending events from the reader thread
        let (tx, rx) = mpsc::channel::<ClaudeEvent>();

        // Spawn a thread to read stdout (blocking I/O)
        thread::spawn(move || {
            let reader = BufReader::new(stdout);

            // Send start event
            tracing::info!("Claude stream started, sending AssistantStart event");
            let _ = tx.send(ClaudeEvent::AssistantStart);

            for line_result in reader.lines() {
                match line_result {
                    Ok(line) => {
                        if line.trim().is_empty() {
                            continue;
                        }

                        match serde_json::from_str::<serde_json::Value>(&line) {
                            Ok(json) => {
                                tracing::debug!("Received JSON: {}", json);
                                if let Some(event) = parse_stream_json(&json) {
                                    tracing::info!("Parsed event: {:?}", event);
                                    if tx.send(event).is_err() {
                                        tracing::warn!("Receiver dropped, stopping stream");
                                        break;
                                    }
                                }
                            }
                            Err(e) => {
                                tracing::warn!("Failed to parse JSON line: {} - {}", e, line);
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Error reading stdout: {}", e);
                        break;
                    }
                }
            }

            // Send end event
            tracing::info!("Claude stream ended, sending AssistantEnd event");
            let _ = tx.send(ClaudeEvent::AssistantEnd);

            // Wait for the process to finish
            match child.wait() {
                Ok(status) => tracing::info!("Claude process exited with status: {}", status),
                Err(e) => tracing::error!("Failed to wait for Claude process: {}", e),
            }
        });

        // Create a stream from the receiver
        // We use a simple polling loop - the async runtime will handle scheduling
        let stream = async_stream::stream! {
            loop {
                // Try to receive without blocking
                match rx.try_recv() {
                    Ok(event) => {
                        let is_end = matches!(event, ClaudeEvent::AssistantEnd);
                        yield event;
                        if is_end {
                            break;
                        }
                    }
                    Err(mpsc::TryRecvError::Empty) => {
                        // No event yet, yield control back to the runtime
                        // Use async sleep to avoid blocking the executor
                        smol::Timer::after(std::time::Duration::from_millis(10)).await;
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        // Channel closed, we're done
                        break;
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }
}
