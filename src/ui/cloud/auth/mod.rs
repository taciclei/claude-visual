//! Authentication UI
//!
//! Login/signup dialog for cloud authentication.

mod dialog;
mod events;
mod buttons;
mod header;
mod feedback;
mod layout;

pub use dialog::{AuthDialog, AuthMode};
pub use events::AuthDialogEvent;
