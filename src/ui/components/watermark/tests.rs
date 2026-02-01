use super::*;

#[test]
fn test_watermark_creation() {
    let wm = Watermark::new("wm-1", "CONFIDENTIAL")
        .position(WatermarkPosition::Center)
        .opacity(0.15)
        .rotation(-45.0);
    assert_eq!(wm.position, WatermarkPosition::Center);
    assert_eq!(wm.opacity, 0.15);
}

#[test]
fn test_watermark_tiled() {
    let wm = Watermark::new("wm-2", "DRAFT")
        .position(WatermarkPosition::Tiled)
        .repeat(5, 5);
    assert_eq!(wm.position, WatermarkPosition::Tiled);
    assert_eq!(wm.repeat_x, 5);
    assert_eq!(wm.repeat_y, 5);
}

#[test]
fn test_security_watermark() {
    let sw = SecurityWatermark::new("sw-1", "user@example.com")
        .timestamp("2024-01-15 10:30:00")
        .ip_address("192.168.1.1")
        .show_ip(true);
    let text = sw.build_text();
    assert!(text.contains("user@example.com"));
    assert!(text.contains("2024-01-15"));
    assert!(text.contains("192.168.1.1"));
}

#[test]
fn test_stamp_overlay() {
    let stamp = StampOverlay::new("stamp-1")
        .stamp_type(StampType::Approved)
        .size(StampSize::Lg);
    assert_eq!(stamp.stamp_type, StampType::Approved);
    assert_eq!(stamp.text.as_ref(), "APPROVED");
}

#[test]
fn test_stamp_type_colors() {
    assert_ne!(StampType::Approved.color(), StampType::Rejected.color());
    assert_ne!(StampType::Draft.color(), StampType::Confidential.color());
}

#[test]
fn test_pattern_overlay() {
    let pattern = PatternOverlay::new("po-1")
        .pattern(PatternType::Grid)
        .spacing(25.0)
        .opacity(0.1);
    assert_eq!(pattern.pattern, PatternType::Grid);
    assert_eq!(pattern.spacing, 25.0);
}

#[test]
fn test_image_watermark() {
    let iw = ImageWatermark::new("iw-1")
        .position(WatermarkPosition::BottomRight)
        .size(100.0, 50.0)
        .grayscale(true);
    assert_eq!(iw.position, WatermarkPosition::BottomRight);
    assert!(iw.grayscale);
}
