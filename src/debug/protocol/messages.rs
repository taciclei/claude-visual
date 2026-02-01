//! Core DAP message types

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// DAP message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DapMessage {
    #[serde(rename = "request")]
    Request(DapRequest),
    #[serde(rename = "response")]
    Response(DapResponse),
    #[serde(rename = "event")]
    Event(DapEvent),
}

/// DAP request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DapRequest {
    /// Sequence number
    pub seq: i64,
    /// Request command
    pub command: String,
    /// Request arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<JsonValue>,
}

impl DapRequest {
    pub fn new(seq: i64, command: &str, arguments: Option<JsonValue>) -> Self {
        Self {
            seq,
            command: command.to_string(),
            arguments,
        }
    }
}

/// DAP response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DapResponse {
    /// Sequence number
    pub seq: i64,
    /// Request sequence number
    pub request_seq: i64,
    /// Success status
    pub success: bool,
    /// Command that was executed
    pub command: String,
    /// Error message (if success is false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Response body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<JsonValue>,
}

/// DAP event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DapEvent {
    /// Sequence number
    pub seq: i64,
    /// Event type
    pub event: String,
    /// Event body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<JsonValue>,
}
