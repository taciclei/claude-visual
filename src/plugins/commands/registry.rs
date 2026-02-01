//! Command registry implementation

use anyhow::Result;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

use super::types::*;

/// Registry of all available slash commands
pub struct CommandRegistry {
    /// Registered commands (name -> command)
    pub(crate) commands: Arc<RwLock<HashMap<String, SlashCommand>>>,
    /// Built-in command handlers
    pub(crate) handlers: Arc<RwLock<HashMap<String, CommandHandler>>>,
}

impl CommandRegistry {
    /// Create a new command registry with built-in commands
    pub fn new() -> Self {
        let registry = Self {
            commands: Arc::new(RwLock::new(HashMap::new())),
            handlers: Arc::new(RwLock::new(HashMap::new())),
        };

        // Register built-in commands
        registry.register_builtins();

        registry
    }

    /// Register built-in commands
    fn register_builtins(&self) {
        super::builtins::register_all(|command, handler| {
            self.register_builtin(command, handler);
        });
    }

    /// Register a built-in command with handler
    pub(crate) fn register_builtin(&self, command: SlashCommand, handler: CommandHandler) {
        let name = command.name.clone();
        self.commands.write().insert(name.clone(), command);
        self.handlers.write().insert(name, handler);
    }

    /// Register a command from an extension
    pub fn register(&self, command: SlashCommand) -> Result<()> {
        let name = command.name.clone();

        // Check if command already exists
        if self.commands.read().contains_key(&name) {
            anyhow::bail!("Command /{} is already registered", name);
        }

        self.commands.write().insert(name, command);
        Ok(())
    }

    /// Unregister a command
    pub fn unregister(&self, name: &str) -> Result<()> {
        let mut commands = self.commands.write();

        // Don't allow unregistering built-in commands
        if let Some(cmd) = commands.get(name) {
            if cmd.is_builtin {
                anyhow::bail!("Cannot unregister built-in command /{}", name);
            }
        }

        commands.remove(name);
        self.handlers.write().remove(name);
        Ok(())
    }

    /// Unregister all commands from an extension
    pub fn unregister_extension(&self, extension_id: &str) {
        let mut commands = self.commands.write();
        let to_remove: Vec<String> = commands
            .iter()
            .filter(|(_, cmd)| cmd.extension_id.as_deref() == Some(extension_id))
            .map(|(name, _)| name.clone())
            .collect();

        for name in to_remove {
            commands.remove(&name);
            self.handlers.write().remove(&name);
        }
    }

    /// Get a command by name
    pub fn get(&self, name: &str) -> Option<SlashCommand> {
        self.commands.read().get(name).cloned()
    }

    /// List all registered commands
    pub fn list(&self) -> Vec<SlashCommand> {
        self.commands.read().values().cloned().collect()
    }

    /// List commands for autocomplete (sorted by name)
    pub fn list_for_autocomplete(&self, prefix: &str) -> Vec<SlashCommand> {
        let prefix = prefix.to_lowercase();
        let mut commands: Vec<_> = self
            .commands
            .read()
            .values()
            .filter(|cmd| cmd.name.to_lowercase().starts_with(&prefix))
            .cloned()
            .collect();

        commands.sort_by(|a, b| a.name.cmp(&b.name));
        commands
    }

    /// Execute a command
    pub fn execute(&self, name: &str, args: &str, ctx: &CommandContext) -> CommandResult {
        // Check if command exists
        let command = match self.get(name) {
            Some(cmd) => cmd,
            None => return CommandResult::Error(format!("Unknown command: /{}", name)),
        };

        // For built-in commands, use the handler
        if command.is_builtin {
            if let Some(handler) = self.handlers.read().get(name) {
                return handler(args, ctx);
            }
        }

        // For extension commands, we would call into WASM here
        // For now, return a placeholder
        CommandResult::Text(format!(
            "Executed /{} {} (from extension: {})",
            name,
            args,
            command.extension_id.as_deref().unwrap_or("unknown")
        ))
    }

    /// Parse a message to check if it starts with a command
    pub fn parse_command(input: &str) -> Option<(&str, &str)> {
        let input = input.trim();
        if !input.starts_with('/') {
            return None;
        }

        let input = &input[1..]; // Remove leading /
        let parts: Vec<&str> = input.splitn(2, char::is_whitespace).collect();

        let name = parts.first()?;
        let args = parts.get(1).copied().unwrap_or("");

        Some((name, args))
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}
