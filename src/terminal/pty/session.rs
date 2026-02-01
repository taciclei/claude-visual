//! PTY session management

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::{Child, ChildStdin};
use tokio::sync::mpsc;
use std::process::{Command, Stdio};

use super::config::PtyConfig;
use super::error::PtyError;
use super::event::PtyEvent;
use super::key::{TerminalKey, function_key_sequence};

/// PTY instance for running shell commands
pub struct Pty {
    /// Configuration
    config: PtyConfig,
    /// Child process
    child: Option<Child>,
    /// Stdin writer
    stdin: Option<ChildStdin>,
    /// Output buffer
    output_buffer: Arc<Mutex<VecDeque<String>>>,
    /// Event sender
    event_tx: Option<mpsc::UnboundedSender<PtyEvent>>,
    /// Is running
    is_running: bool,
    /// Command history
    command_history: Vec<String>,
    /// Current command being typed
    current_input: String,
}

impl Pty {
    /// Create a new PTY with default config
    pub fn new() -> Self {
        Self::with_config(PtyConfig::default())
    }

    /// Create a new PTY with custom config
    pub fn with_config(config: PtyConfig) -> Self {
        Self {
            config,
            child: None,
            stdin: None,
            output_buffer: Arc::new(Mutex::new(VecDeque::new())),
            event_tx: None,
            is_running: false,
            command_history: Vec::new(),
            current_input: String::new(),
        }
    }

    /// Start the PTY session
    pub async fn start(&mut self) -> Result<mpsc::UnboundedReceiver<PtyEvent>, PtyError> {
        let (tx, rx) = mpsc::unbounded_channel();
        self.event_tx = Some(tx.clone());

        let mut cmd = Command::new(&self.config.shell);
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // Set working directory
        if let Some(cwd) = &self.config.cwd {
            cmd.current_dir(cwd);
        }

        // Set environment variables
        for (key, value) in &self.config.env {
            cmd.env(key, value);
        }

        // Set terminal size environment
        cmd.env("COLUMNS", self.config.size.0.to_string());
        cmd.env("LINES", self.config.size.1.to_string());
        cmd.env("TERM", "xterm-256color");

        // Convert to tokio process
        let mut child = tokio::process::Command::from(cmd)
            .spawn()
            .map_err(|e| PtyError::Spawn(e.to_string()))?;

        // Take stdin for writing
        self.stdin = child.stdin.take();

        // Read stdout in background
        if let Some(stdout) = child.stdout.take() {
            let buffer = self.output_buffer.clone();
            let tx_clone = tx.clone();
            let history_size = self.config.history_size;

            tokio::spawn(async move {
                Self::read_output(stdout, buffer, tx_clone, history_size).await;
            });
        }

        // Read stderr in background
        if let Some(stderr) = child.stderr.take() {
            let buffer = self.output_buffer.clone();
            let tx_clone = tx.clone();
            let history_size = self.config.history_size;

            tokio::spawn(async move {
                Self::read_output(stderr, buffer, tx_clone, history_size).await;
            });
        }

        // Monitor process exit
        let tx_exit = tx.clone();
        tokio::spawn(async move {
            if let Ok(status) = child.wait().await {
                let code = status.code().unwrap_or(-1);
                let _ = tx_exit.send(PtyEvent::Exit(code));
            }
        });

        self.is_running = true;

        Ok(rx)
    }

    /// Read output from stream
    async fn read_output(
        mut stream: impl AsyncReadExt + Unpin,
        buffer: Arc<Mutex<VecDeque<String>>>,
        tx: mpsc::UnboundedSender<PtyEvent>,
        history_size: usize,
    ) {
        let mut buf = [0u8; 4096];

        loop {
            match stream.read(&mut buf).await {
                Ok(0) => break, // EOF
                Ok(n) => {
                    let text = String::from_utf8_lossy(&buf[..n]).to_string();

                    // Check for special sequences
                    if text.contains('\x07') {
                        let _ = tx.send(PtyEvent::Bell);
                    }

                    // Store in buffer
                    {
                        let mut buf = buffer.lock().unwrap();
                        buf.push_back(text.clone());
                        while buf.len() > history_size {
                            buf.pop_front();
                        }
                    }

                    // Send event
                    let _ = tx.send(PtyEvent::Output(text));
                }
                Err(e) => {
                    let _ = tx.send(PtyEvent::Error(e.to_string()));
                    break;
                }
            }
        }
    }

    /// Write input to PTY
    pub async fn write(&mut self, data: &str) -> Result<(), PtyError> {
        if let Some(stdin) = &mut self.stdin {
            stdin
                .write_all(data.as_bytes())
                .await
                .map_err(PtyError::Io)?;
            stdin.flush().await.map_err(PtyError::Io)?;
            Ok(())
        } else {
            Err(PtyError::NotRunning)
        }
    }

    /// Write a line (with newline)
    pub async fn write_line(&mut self, line: &str) -> Result<(), PtyError> {
        self.write(&format!("{}\n", line)).await?;

        // Add to command history
        if !line.trim().is_empty() {
            self.command_history.push(line.to_string());
        }

        Ok(())
    }

    /// Send a special key
    pub async fn send_key(&mut self, key: TerminalKey) -> Result<(), PtyError> {
        let sequence = match key {
            TerminalKey::Enter => "\n",
            TerminalKey::Tab => "\t",
            TerminalKey::Backspace => "\x7f",
            TerminalKey::Escape => "\x1b",
            TerminalKey::Up => "\x1b[A",
            TerminalKey::Down => "\x1b[B",
            TerminalKey::Right => "\x1b[C",
            TerminalKey::Left => "\x1b[D",
            TerminalKey::Home => "\x1b[H",
            TerminalKey::End => "\x1b[F",
            TerminalKey::PageUp => "\x1b[5~",
            TerminalKey::PageDown => "\x1b[6~",
            TerminalKey::Delete => "\x1b[3~",
            TerminalKey::Insert => "\x1b[2~",
            TerminalKey::F(n) => {
                return self.write(&function_key_sequence(n)).await;
            }
            TerminalKey::Ctrl(c) => {
                let code = (c.to_ascii_lowercase() as u8) - b'a' + 1;
                return self.write(&String::from(code as char)).await;
            }
        };
        self.write(sequence).await
    }

    /// Resize the terminal
    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.config.size = (cols, rows);
        // Note: Full PTY resize would require ioctl on Unix
        // For now, we just update the config
    }

    /// Stop the PTY session
    pub async fn stop(&mut self) -> Result<(), PtyError> {
        if let Some(mut child) = self.child.take() {
            child.kill().await.map_err(PtyError::Io)?;
        }
        self.stdin = None;
        self.is_running = false;
        Ok(())
    }

    /// Check if running
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Get command history
    pub fn command_history(&self) -> &[String] {
        &self.command_history
    }

    /// Get recent output lines
    pub fn recent_output(&self, lines: usize) -> Vec<String> {
        let buffer = self.output_buffer.lock().unwrap();
        buffer.iter().rev().take(lines).rev().cloned().collect()
    }

    /// Get all output
    pub fn all_output(&self) -> String {
        let buffer = self.output_buffer.lock().unwrap();
        buffer.iter().cloned().collect()
    }

    /// Clear output buffer
    pub fn clear_buffer(&mut self) {
        let mut buffer = self.output_buffer.lock().unwrap();
        buffer.clear()
    }

    /// Update current input (for display)
    pub fn set_current_input(&mut self, input: String) {
        self.current_input = input;
    }

    /// Get current input
    pub fn current_input(&self) -> &str {
        &self.current_input
    }
}

impl Default for Pty {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pty_creation() {
        let pty = Pty::new();
        assert!(!pty.is_running());
        assert!(pty.command_history().is_empty());
    }
}
