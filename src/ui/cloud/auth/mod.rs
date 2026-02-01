//! Authentication UI
//!
//! Login/signup dialog for cloud authentication.

mod buttons;
mod dialog;
mod events;
mod feedback;
mod header;
mod layout;

pub use dialog::{AuthDialog, AuthMode};
pub use events::AuthDialogEvent;
