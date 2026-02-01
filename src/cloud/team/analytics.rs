//! Usage analytics types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Usage analytics data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageAnalytics {
    /// Team ID
    pub team_id: String,
    /// Time period
    pub period: AnalyticsPeriod,
    /// Start of period
    pub period_start: DateTime<Utc>,
    /// End of period
    pub period_end: DateTime<Utc>,
    /// Total conversations
    pub total_conversations: u64,
    /// Total messages
    pub total_messages: u64,
    /// Total tokens used
    pub total_tokens: u64,
    /// Active users count
    pub active_users: u64,
    /// Usage by user
    pub usage_by_user: HashMap<String, UserUsage>,
    /// Usage by project
    pub usage_by_project: HashMap<String, ProjectUsage>,
    /// Daily breakdown
    pub daily_breakdown: Vec<DailyUsage>,
}

/// Analytics time period
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnalyticsPeriod {
    /// Last 7 days
    Week,
    /// Last 30 days
    Month,
    /// Last 90 days
    Quarter,
    /// Last 365 days
    Year,
    /// Custom range
    Custom,
}

impl AnalyticsPeriod {
    pub fn display_name(&self) -> &'static str {
        match self {
            AnalyticsPeriod::Week => "Last 7 days",
            AnalyticsPeriod::Month => "Last 30 days",
            AnalyticsPeriod::Quarter => "Last 90 days",
            AnalyticsPeriod::Year => "Last year",
            AnalyticsPeriod::Custom => "Custom range",
        }
    }

    pub fn days(&self) -> i64 {
        match self {
            AnalyticsPeriod::Week => 7,
            AnalyticsPeriod::Month => 30,
            AnalyticsPeriod::Quarter => 90,
            AnalyticsPeriod::Year => 365,
            AnalyticsPeriod::Custom => 0,
        }
    }
}

/// Per-user usage data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUsage {
    /// User ID
    pub user_id: String,
    /// User name
    pub user_name: Option<String>,
    /// Conversations count
    pub conversations: u64,
    /// Messages count
    pub messages: u64,
    /// Tokens used
    pub tokens: u64,
    /// Last active
    pub last_active: DateTime<Utc>,
}

/// Per-project usage data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectUsage {
    /// Project ID
    pub project_id: String,
    /// Project name
    pub project_name: String,
    /// Conversations count
    pub conversations: u64,
    /// Messages count
    pub messages: u64,
    /// Tokens used
    pub tokens: u64,
    /// Contributors count
    pub contributors: u64,
}

/// Daily usage breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyUsage {
    /// Date
    pub date: DateTime<Utc>,
    /// Conversations
    pub conversations: u64,
    /// Messages
    pub messages: u64,
    /// Tokens
    pub tokens: u64,
    /// Active users
    pub active_users: u64,
}
