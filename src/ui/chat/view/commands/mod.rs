//! Command palette related methods for ChatView
//!
//! This module contains all command palette functionality including:
//! - Command definitions (get_palette_commands)
//! - Command filtering and search
//! - Command execution logic
//! - Palette toggle and state management

mod definitions;
mod executor;
mod filtering;
mod palette;

pub(crate) use definitions::*;
pub(crate) use executor::*;
pub(crate) use filtering::*;
pub(crate) use palette::*;
