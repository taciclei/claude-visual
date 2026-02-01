//! Team manager for team operations

mod activity;
mod analytics;
mod core;
mod members;
mod projects;
mod teams;
#[cfg(test)]
mod tests;
mod types;

pub use types::TeamManager;
