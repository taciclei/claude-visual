//! Tests for MCP context attachment

use super::types::*;

#[test]
fn test_attachable_resource() {
    let resource = AttachableResource::new("filesystem", "file:///test.rs", "test.rs")
        .with_description("Test file")
        .with_mime_type("text/x-rust")
        .with_size(1024);

    assert_eq!(resource.server, "filesystem");
    assert_eq!(resource.uri, "file:///test.rs");
    assert_eq!(resource.name, "test.rs");
    assert_eq!(resource.description, Some("Test file".to_string()));
    assert_eq!(resource.mime_type, Some("text/x-rust".to_string()));
    assert_eq!(resource.size, Some(1024));
}

#[test]
fn test_attachment_status() {
    let ready = AttachmentStatus::Ready;
    let attached = AttachmentStatus::Attached;
    let failed = AttachmentStatus::Failed("Error".to_string());

    assert!(matches!(ready, AttachmentStatus::Ready));
    assert!(matches!(attached, AttachmentStatus::Attached));
    assert!(matches!(failed, AttachmentStatus::Failed(_)));
}
