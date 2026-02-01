//! Built-in slash commands

mod general;
mod mcp;

use super::types::*;

/// Register all built-in commands
pub(super) fn register_all(register: impl Fn(SlashCommand, CommandHandler)) {
    general::register_general_commands(&register);
    mcp::register_mcp_commands(&register);
}
