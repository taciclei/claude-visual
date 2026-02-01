//! PTY event types

/// Events from the PTY
#[derive(Debug, Clone)]
pub enum PtyEvent {
    /// Output data received
    Output(String),
    /// Process exited with code
    Exit(i32),
    /// Error occurred
    Error(String),
    /// Title changed (from escape sequence)
    TitleChanged(String),
    /// Bell character received
    Bell,
}
