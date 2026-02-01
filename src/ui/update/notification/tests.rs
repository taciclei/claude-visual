//! Tests for update notification component

use crate::update::{UpdateInfo, UpdateStatus};

#[test]
fn test_update_info_creation() {
    let info = UpdateInfo {
        version: "1.2.0".to_string(),
        name: "New Release".to_string(),
        body: "Bug fixes and improvements".to_string(),
        download_url: "https://example.com/download".to_string(),
        published_at: "2025-01-26".to_string(),
        prerelease: false,
    };

    assert_eq!(info.version, "1.2.0");
    assert!(!info.prerelease);
}

#[test]
fn test_update_status_variants() {
    let status = UpdateStatus::Unknown;
    assert!(matches!(status, UpdateStatus::Unknown));

    let status = UpdateStatus::UpToDate;
    assert!(matches!(status, UpdateStatus::UpToDate));

    let info = UpdateInfo {
        version: "2.0.0".to_string(),
        name: "Major Update".to_string(),
        body: "".to_string(),
        download_url: "".to_string(),
        published_at: "".to_string(),
        prerelease: false,
    };
    let status = UpdateStatus::UpdateAvailable(info);
    assert!(matches!(status, UpdateStatus::UpdateAvailable(_)));
}
