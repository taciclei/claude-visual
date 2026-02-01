//! Diff preview functionality

mod operations;
mod render;
mod syntax;
mod utils;

pub(in crate::ui::workspace) use operations::*;
pub(in crate::ui::workspace) use render::*;
pub(in crate::ui::workspace) use syntax::*;
pub(in crate::ui::workspace) use utils::*;
