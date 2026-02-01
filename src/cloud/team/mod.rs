//! Team Features Module
//!
//! Provides team workspaces, shared projects, activity feed, and usage analytics.

pub mod activity;
pub mod analytics;
pub mod invitations;
pub mod manager;
pub mod projects;
pub mod types;

// Re-export commonly used types
pub use activity::{ActivityEntry, ActivityTarget, ActivityType};
pub use analytics::{AnalyticsPeriod, DailyUsage, ProjectUsage, UsageAnalytics, UserUsage};
pub use invitations::{InvitationStatus, TeamInvitation};
pub use manager::TeamManager;
pub use projects::{ProjectPermission, SharedProject};
pub use types::{Team, TeamError, TeamMember, TeamRole, TeamSettings};
