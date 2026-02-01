//! Session management modules for ChatView
//!
//! This module contains all methods related to session management split by functionality:
//! - history: Session history and resume
//! - info: Session info and details
//! - health: Session health tracking, token and cost tracking
//! - conversation: Conversation save/load and management
//! - export: Export functionality in various formats

mod conversation;
mod export;
mod health;
mod history;
mod info;
