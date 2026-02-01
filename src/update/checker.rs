//! Update Checker
//!
//! Checks GitHub releases for new versions.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Current application version
pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// GitHub repository for releases
const GITHUB_REPO: &str = "your-username/claude-visual";

/// Update status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateStatus {
    /// No update check performed yet
    Unknown,
    /// Currently checking for updates
    Checking,
    /// Up to date
    UpToDate,
    /// Update available
    UpdateAvailable(UpdateInfo),
    /// Error checking for updates
    Error(String),
}

/// Information about an available update
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateInfo {
    /// New version string
    pub version: String,
    /// Release name/title
    pub name: String,
    /// Release notes/changelog
    pub body: String,
    /// Download URL for the appropriate platform
    pub download_url: String,
    /// Release date
    pub published_at: String,
    /// Whether this is a prerelease
    pub prerelease: bool,
}

/// GitHub release API response
#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    name: Option<String>,
    body: Option<String>,
    published_at: String,
    prerelease: bool,
    assets: Vec<GitHubAsset>,
}

/// GitHub release asset
#[derive(Debug, Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

/// Update checker that queries GitHub releases
pub struct UpdateChecker {
    /// GitHub API URL for releases
    api_url: String,
    /// Whether to include prereleases
    include_prerelease: bool,
}

impl UpdateChecker {
    /// Create a new update checker
    pub fn new() -> Self {
        Self {
            api_url: format!(
                "https://api.github.com/repos/{}/releases/latest",
                GITHUB_REPO
            ),
            include_prerelease: false,
        }
    }

    /// Create an update checker that includes prereleases
    pub fn with_prerelease(mut self) -> Self {
        self.include_prerelease = true;
        self.api_url = format!("https://api.github.com/repos/{}/releases", GITHUB_REPO);
        self
    }

    /// Check for updates asynchronously
    pub async fn check(&self) -> Result<UpdateStatus> {
        // In a real implementation, this would use reqwest to fetch from GitHub
        // For now, we'll return a placeholder

        // Simulated check - in production, replace with actual HTTP request
        // let response = reqwest::get(&self.api_url).await?;
        // let release: GitHubRelease = response.json().await?;

        // For now, return up to date
        Ok(UpdateStatus::UpToDate)
    }

    /// Parse a GitHub release into UpdateInfo
    fn parse_release(&self, release: GitHubRelease) -> Result<UpdateInfo> {
        // Find the appropriate asset for this platform
        let asset_name = self.get_platform_asset_name();

        let download_url = release
            .assets
            .iter()
            .find(|a| a.name.contains(&asset_name))
            .map(|a| a.browser_download_url.clone())
            .ok_or_else(|| anyhow!("No download available for this platform"))?;

        Ok(UpdateInfo {
            version: release.tag_name.trim_start_matches('v').to_string(),
            name: release.name.unwrap_or_else(|| release.tag_name.clone()),
            body: release.body.unwrap_or_default(),
            download_url,
            published_at: release.published_at,
            prerelease: release.prerelease,
        })
    }

    /// Get the expected asset name for this platform
    fn get_platform_asset_name(&self) -> String {
        #[cfg(target_os = "macos")]
        {
            #[cfg(target_arch = "aarch64")]
            return "macos-arm64".to_string();
            #[cfg(target_arch = "x86_64")]
            return "macos-x64".to_string();
        }

        #[cfg(target_os = "linux")]
        {
            return "linux-x64".to_string();
        }

        #[cfg(target_os = "windows")]
        {
            return "windows-x64".to_string();
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            return "unknown".to_string();
        }
    }

    /// Compare version strings
    pub fn compare_versions(current: &str, new: &str) -> Ordering {
        let parse_version = |v: &str| -> Vec<u32> {
            v.trim_start_matches('v')
                .split('.')
                .filter_map(|s| s.parse().ok())
                .collect()
        };

        let current_parts = parse_version(current);
        let new_parts = parse_version(new);

        for (c, n) in current_parts.iter().zip(new_parts.iter()) {
            match c.cmp(n) {
                Ordering::Equal => continue,
                other => return other,
            }
        }

        current_parts.len().cmp(&new_parts.len())
    }
}

impl Default for UpdateChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison() {
        assert_eq!(
            UpdateChecker::compare_versions("1.0.0", "1.0.1"),
            Ordering::Less
        );
        assert_eq!(
            UpdateChecker::compare_versions("1.0.0", "1.0.0"),
            Ordering::Equal
        );
        assert_eq!(
            UpdateChecker::compare_versions("2.0.0", "1.9.9"),
            Ordering::Greater
        );
        assert_eq!(
            UpdateChecker::compare_versions("v1.0.0", "1.0.0"),
            Ordering::Equal
        );
    }
}
