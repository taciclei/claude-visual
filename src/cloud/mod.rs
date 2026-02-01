//! Cloud & Collaboration Module
//!
//! Provides cloud sync, authentication, sharing, and team collaboration capabilities.

pub mod auth;
pub mod storage;
pub mod sync;
pub mod team;

pub use auth::{AuthError, AuthState, CloudAuth, OAuthProvider, UserProfile};
pub use storage::{CloudStorage, EncryptedData, StorageError};
pub use sync::{ConflictResolution, SyncError, SyncManager, SyncOperation, SyncStatus};
pub use team::{
    ActivityEntry, ActivityTarget, ActivityType, AnalyticsPeriod, InvitationStatus,
    ProjectPermission, SharedProject, Team, TeamError, TeamInvitation, TeamManager, TeamMember,
    TeamRole, TeamSettings, UsageAnalytics,
};
